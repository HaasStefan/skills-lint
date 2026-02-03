use std::fmt;

/// Severity level for a lint finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Pass,
    Warning,
    Error,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Pass => write!(f, "PASS"),
            Severity::Warning => write!(f, "WARN"),
            Severity::Error => write!(f, "ERROR"),
        }
    }
}

/// A single lint finding for one file × model combination.
#[derive(Debug, Clone)]
pub struct LintFinding {
    pub file: String,
    pub model: String,
    pub token_count: usize,
    pub warning_threshold: usize,
    pub error_threshold: usize,
    pub severity: Severity,
}

/// Aggregated results from a lint run.
#[derive(Debug, Clone)]
pub struct LintReport {
    pub findings: Vec<LintFinding>,
}

impl LintReport {
    pub fn new(findings: Vec<LintFinding>) -> Self {
        Self { findings }
    }

    /// Returns the worst severity across all findings.
    pub fn worst_severity(&self) -> Severity {
        self.findings
            .iter()
            .map(|f| f.severity)
            .max()
            .unwrap_or(Severity::Pass)
    }
}
