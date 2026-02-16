//! Rule definitions and TOML loader.

use crate::severity::Severity;
use fancy_regex::Regex;
use log::warn;
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a secret detection rule.
pub struct Rule {
    pub description: String,
    pub regex: Regex,
    pub severity: Severity,
}

/// Loads rules from a TOML file.
pub fn load_rules<P: AsRef<Path>>(path: P) -> io::Result<Vec<Rule>> {
    let mut visited = HashSet::new();
    load_rules_inner(path.as_ref(), &mut visited)
}

/// Recursively loads rules, tracking visited files to detect circular includes.
fn load_rules_inner(path: &Path, visited: &mut HashSet<PathBuf>) -> io::Result<Vec<Rule>> {
    let canonical = path.canonicalize()?;
    if !visited.insert(canonical) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Circular include detected: {}", path.display()),
        ));
    }

    let base_dir = path.parent().unwrap_or(Path::new("."));
    let content = std::fs::read_to_string(path)?;
    let toml_value: toml::Value =
        toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut rules = Vec::new();

    if let Some(includes) = toml_value.get("includes").and_then(|v| v.as_array()) {
        for include in includes {
            if let Some(include_path) = include.as_str() {
                let full_path = base_dir.join(include_path);
                rules.extend(load_rules_inner(&full_path, visited)?);
            }
        }
    }

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

    #[test]
    fn test_load_rules_with_includes() {
        let base_toml = r#"
        [[rules]]
        description = "Base Rule"
        regex = "base_[0-9]+"
        severity = "low"
        "#;

        let child_toml = r#"
        includes = ["base_rules.toml"]

        [[rules]]
        description = "Child Rule"
        regex = "child_[0-9]+"
        severity = "high"
        "#;

        let tmp_dir = std::env::temp_dir().join("lsa_test_includes");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        std::fs::write(tmp_dir.join("base_rules.toml"), base_toml).unwrap();
        std::fs::write(tmp_dir.join("child_rules.toml"), child_toml).unwrap();

        let rules = load_rules(tmp_dir.join("child_rules.toml")).unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].description, "Base Rule");
        assert_eq!(rules[1].description, "Child Rule");
    }

    #[test]
    fn test_load_rules_circular_include() {
        let a_toml = r#"
        includes = ["b.toml"]

        [[rules]]
        description = "Rule A"
        regex = "a_[0-9]+"
        severity = "low"
        "#;

        let b_toml = r#"
        includes = ["a.toml"]

        [[rules]]
        description = "Rule B"
        regex = "b_[0-9]+"
        severity = "low"
        "#;

        let tmp_dir = std::env::temp_dir().join("lsa_test_circular");
        std::fs::create_dir_all(&tmp_dir).unwrap();

        std::fs::write(tmp_dir.join("a.toml"), a_toml).unwrap();
        std::fs::write(tmp_dir.join("b.toml"), b_toml).unwrap();

        let result = load_rules(tmp_dir.join("a.toml"));
        assert!(result.is_err());
    }
}
