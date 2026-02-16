//! Log scanning logic module.

use crate::rules::Rule;
use crate::severity::Severity;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Row, Table};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Represents a match found in the log.
#[derive(Clone)]
pub struct Match {
    pub line_num: usize,
    pub description: String,
    pub severity: Severity,
    pub line: String,
}

/// Scans a log file using the provided rules and prints matches in a formatted table.
pub fn scan_log<P: AsRef<std::path::Path>>(log_path: P, rules: &[Rule]) -> io::Result<()> {
    let file = File::open(log_path)?;
    let reader = BufReader::new(file);

    let mut matches = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        for rule in rules {
            if let Ok(true) = rule.regex.is_match(&line) {
                matches.push(Match {
                    line_num: line_num + 1,
                    description: rule.description.clone(),
                    severity: rule.severity.clone(),
                    line: line.clone(),
                });
            }
        }
    }

    matches.sort_by(|a, b| {
        b.severity
            .rank()
            .cmp(&a.severity.rank())
            .then(a.line_num.cmp(&b.line_num))
    });

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(Row::from(vec![
        Cell::new(""),
        Cell::new("Line"),
        Cell::new("Description"),
        Cell::new("Severity"),
        Cell::new("Log"),
    ]));

    for m in matches {
        let color = match m.severity {
            Severity::Critical => Color::Red,
            Severity::High => Color::Yellow,
            Severity::Medium => Color::Blue,
            Severity::Low => Color::Green,
            Severity::Unknown => Color::White,
        };

        table.add_row(Row::from(vec![
            Cell::new("●").fg(color),
            Cell::new(m.line_num),
            Cell::new(m.description),
            Cell::new(m.severity.as_str()),
            Cell::new(m.line),
        ]));
    }

    println!("{table}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::Rule;
    use crate::severity::Severity;
    use fancy_regex::Regex;

    #[test]
    fn test_scan_log_basic() {
        let rules = vec![Rule {
            description: "Token".to_string(),
            regex: Regex::new("tok_[0-9]+").unwrap(),
            severity: Severity::High,
        }];

        let log_content = "info tok_123\nerror nothing\nwarn tok_456";
        let tmp = std::env::temp_dir().join("test_log.log");
        std::fs::write(&tmp, log_content).unwrap();

        scan_log(&tmp, &rules).unwrap();
    }

    #[test]
    fn test_scan_log_ordering_by_severity() {
        let matches = vec![
            Match {
                line_num: 1,
                description: "low item".to_string(),
                severity: Severity::Low,
                line: "line1".to_string(),
            },
            Match {
                line_num: 2,
                description: "critical item".to_string(),
                severity: Severity::Critical,
                line: "line2".to_string(),
            },
            Match {
                line_num: 3,
                description: "high item".to_string(),
                severity: Severity::High,
                line: "line3".to_string(),
            },
        ];

        let mut sorted = matches.clone();
        sorted.sort_by(|a, b| {
            b.severity
                .rank()
                .cmp(&a.severity.rank())
                .then(a.line_num.cmp(&b.line_num))
        });

        let expected = vec![Severity::Critical, Severity::High, Severity::Low];

        let result: Vec<_> = sorted.iter().map(|m| m.severity.clone()).collect();

        assert_eq!(result, expected);
    }
}
