//! Log Security Analyzer Library
//!
//! This library exposes the main modules for log scanning, rule
//! management, and severity handling.

pub mod rules;
pub mod scanner;
pub mod severity;

pub use rules::load_rules;
pub use rules::Rule;
pub use scanner::{scan_log, Match};
pub use severity::Severity;
