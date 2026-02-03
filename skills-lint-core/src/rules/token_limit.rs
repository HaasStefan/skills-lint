use crate::config::ResolvedBudget;
use crate::tokenizer;
use crate::types::{LintFinding, Severity};
use crate::errors::LintError;

/// Check token count for a file against a resolved budget for a specific model.
pub fn check(
    file: &str,
    model: &str,
    content: &str,
    budget: &ResolvedBudget,
) -> Result<LintFinding, LintError> {
    let token_count = tokenizer::count_tokens(content, &budget.encoding)?;

    let severity = if token_count >= budget.error {
        Severity::Error
    } else if token_count >= budget.warning {
        Severity::Warning
    } else {
        Severity::Pass
    };

    Ok(LintFinding {
        file: file.to_string(),
        model: model.to_string(),
        token_count,
        warning_threshold: budget.warning,
        error_threshold: budget.error,
        severity,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass() {
        let budget = ResolvedBudget {
            encoding: "cl100k_base".to_string(),
            warning: 8000,
            error: 12000,
        };
        let finding = check("test.md", "gpt-4", "Hello", &budget).unwrap();
        assert_eq!(finding.severity, Severity::Pass);
    }
}
