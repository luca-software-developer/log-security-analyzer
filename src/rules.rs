//! Rule definitions and TOML loader.

use crate::severity::Severity;
use fancy_regex::Regex;
use log::warn;
use std::io;
use std::path::Path;

/// Represents a secret detection rule.
pub struct Rule {
    pub description: String,
    pub regex: Regex,
    pub severity: Severity,
}

/// Loads rules from a TOML file.
pub fn load_rules<P: AsRef<Path>>(path: P) -> io::Result<Vec<Rule>> {
    let content = std::fs::read_to_string(path)?;
    let toml_value: toml::Value =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut rules = Vec::new();

    if let Some(rules_array) = toml_value.get("rules").and_then(|v| v.as_array()) {
        for rule in rules_array {
            let description = match rule.get("description").and_then(|v| v.as_str()) {
                Some(v) => v,
                None => continue,
            };

            let regex_str = match rule.get("regex").and_then(|v| v.as_str()) {
                Some(v) => v,
                None => continue,
            };

            let regex = match Regex::new(regex_str) {
                Ok(r) => r,
                Err(e) => {
                    warn!("Skipping invalid regex [{}]: {}", description, e);
                    continue;
                }
            };

            let severity = rule
                .get("severity")
                .and_then(|v| v.as_str())
                .map(Severity::from_str)
                .unwrap_or(Severity::Unknown);

            rules.push(Rule {
                description: description.to_string(),
                regex,
                severity,
            });
        }
    }

    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_rules_basic() {
        let toml_str = r#"
        [[rules]]
        description = "Test Token"
        regex = "tok_[0-9]+"
        severity = "high"
        "#;

        let tmp = std::env::temp_dir().join("test_rules.toml");
        std::fs::write(&tmp, toml_str).unwrap();

        let rules = load_rules(&tmp).unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].description, "Test Token");
        assert_eq!(rules[0].severity, Severity::High);
        assert!(rules[0].regex.is_match("tok_123").unwrap());
    }
}
