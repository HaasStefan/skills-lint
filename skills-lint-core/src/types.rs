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
    pub rule: String,
    pub file: String,
    pub model: String,
    pub token_count: usize,
    pub warning_threshold: usize,
    pub error_threshold: usize,
    pub severity: Severity,
}

/// A structural validation finding for one file.
#[derive(Debug, Clone)]
pub struct StructureFinding {
    pub rule: String,
    pub file: String,
    pub message: String,
    pub severity: Severity,
}

/// Aggregated results from a lint run.
#[derive(Debug, Clone)]
pub struct LintReport {
    pub findings: Vec<LintFinding>,
    pub structure_findings: Vec<StructureFinding>,
}

impl LintReport {
    pub fn new(findings: Vec<LintFinding>, structure_findings: Vec<StructureFinding>) -> Self {
        Self {
            findings,
            structure_findings,
        }
    }

    /// Returns the worst severity across all findings.
    pub fn worst_severity(&self) -> Severity {
        let token_worst = self.findings.iter().map(|f| f.severity).max();
        let structure_worst = self.structure_findings.iter().map(|f| f.severity).max();
        token_worst
            .into_iter()
            .chain(structure_worst)
            .max()
            .unwrap_or(Severity::Pass)
    }
}
