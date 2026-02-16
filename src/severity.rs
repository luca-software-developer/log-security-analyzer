//! Severity levels for detected secrets.

/// Represents severity of a secret leak.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

impl Severity {
    /// Returns a numeric rank for sorting
    pub fn rank(&self) -> u8 {
        match self {
            Severity::Critical => 5,
            Severity::High => 4,
            Severity::Medium => 3,
            Severity::Low => 2,
            Severity::Unknown => 1,
        }
    }

    /// Converts a string to a Severity level.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Self::Critical,
            "high" => Self::High,
            "medium" => Self::Medium,
            "low" => Self::Low,
            _ => Self::Unknown,
        }
    }

    /// Returns string representation of severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Unknown => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Severity::from_str("critical"), Severity::Critical);
        assert_eq!(Severity::from_str("HIGH"), Severity::High);
        assert_eq!(Severity::from_str("unknown"), Severity::Unknown);
        assert_eq!(Severity::from_str("foo"), Severity::Unknown);
    }

    #[test]
    fn test_ordering() {
        let mut levels = vec![Severity::Low, Severity::Critical, Severity::Medium];
        levels.sort();
        assert_eq!(
            levels,
            vec![Severity::Low, Severity::Medium, Severity::Critical]
        );
    }
}
