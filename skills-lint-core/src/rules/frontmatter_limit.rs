use std::path::Path;

use crate::cache::TokenCache;
use crate::config::{Config, ResolvedBudget};
use crate::errors::LintError;
use crate::rules::skill_index_budget::extract_frontmatter;
use crate::rules::token_limit;
use crate::types::LintFinding;

/// Check frontmatter token count for a single file against a resolved budget for one model.
pub fn check(
    file: &str,
    model: &str,
    frontmatter: &str,
    budget: &ResolvedBudget,
    cache: Option<&mut TokenCache>,
) -> Result<LintFinding, LintError> {
    token_limit::check("frontmatter-limit", file, model, frontmatter, budget, cache)
}

/// Check a single file's frontmatter against the frontmatter-limit rule for all configured models.
///
/// Returns an empty vec if the rule is not configured or the file has no frontmatter.
pub fn check_file(config: &Config, file: &str, mut cache: Option<&mut TokenCache>) -> Result<Vec<LintFinding>, LintError> {
    let fl = match config.rules.frontmatter_limit.as_ref() {
        Some(fl) => fl,
        None => return Ok(Vec::new()),
    };

    let content = std::fs::read_to_string(Path::new(file))
        .map_err(|e| LintError::FileRead(file.to_string(), e))?;

    let frontmatter = match extract_frontmatter(&content) {
        Some(fm) => fm,
        None => return Ok(Vec::new()),
    };

    let mut model_names: Vec<&String> = fl.models.keys().collect();
    model_names.sort();

    let mut findings = Vec::new();
    for model in &model_names {
        if let Some(budget) = config.resolve_frontmatter_limit(model) {
            findings.push(check(file, model, &frontmatter, &budget, cache.as_deref_mut())?);
        }
    }

    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;

    #[test]
    fn test_check_pass() {
        let budget = ResolvedBudget {
            encoding: "cl100k_base".to_string(),
            warning: 8000,
            error: 12000,
        };
        let finding = check("test.md", "gpt-4", "name: tiny", &budget, None).unwrap();
        assert_eq!(finding.severity, Severity::Pass);
        assert_eq!(finding.rule, "frontmatter-limit");
    }

    #[test]
    fn test_check_error() {
        let budget = ResolvedBudget {
            encoding: "cl100k_base".to_string(),
            warning: 1,
            error: 2,
        };
        let big = "word ".repeat(100);
        let finding = check("test.md", "gpt-4", &big, &budget, None).unwrap();
        assert_eq!(finding.severity, Severity::Error);
        assert_eq!(finding.rule, "frontmatter-limit");
    }
}
