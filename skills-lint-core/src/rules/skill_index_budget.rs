use std::path::Path;

use crate::config::{Config, ResolvedBudget};
use crate::errors::LintError;
use crate::rules::token_limit;
use crate::types::LintFinding;

/// Label used for the aggregate finding (not a real file path).
pub const AGGREGATE_LABEL: &str = "(skill index)";

/// Extract YAML frontmatter from a SKILL.md file.
///
/// The first line (after an optional UTF-8 BOM) must be `---`.
/// Lines are collected until a closing `---` is found.
/// Returns `None` if the file has no frontmatter or the closing delimiter is missing.
pub fn extract_frontmatter(content: &str) -> Option<String> {
    let content = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut lines = content.lines();

    let first = lines.next()?;
    if first.trim() != "---" {
        return None;
    }

    let mut fm_lines = Vec::new();
    for line in lines {
        if line.trim() == "---" {
            return Some(fm_lines.join("\n"));
        }
        fm_lines.push(line);
    }

    // No closing delimiter found
    None
}

/// Check the aggregated frontmatter string against a resolved budget for one model.
pub fn check(
    aggregated: &str,
    model: &str,
    budget: &ResolvedBudget,
) -> Result<LintFinding, LintError> {
    token_limit::check(AGGREGATE_LABEL, model, aggregated, budget)
}

/// Check all discovered files' frontmatter against the skill-index-budget rule.
///
/// Returns an empty vec if the rule is not configured.
pub fn check_all(config: &Config, files: &[String]) -> Result<Vec<LintFinding>, LintError> {
    if config.rules.skill_index_budget.is_none() {
        return Ok(Vec::new());
    }

    let mut frontmatter_parts = Vec::new();
    for file in files {
        let content = std::fs::read_to_string(Path::new(file))
            .map_err(|e| LintError::FileRead(file.clone(), e))?;
        if let Some(fm) = extract_frontmatter(&content) {
            frontmatter_parts.push(fm);
        }
    }

    let aggregated = frontmatter_parts.join("\n");

    let sib = config.rules.skill_index_budget.as_ref().unwrap();
    let mut model_names: Vec<&String> = sib.models.keys().collect();
    model_names.sort();

    let mut findings = Vec::new();
    for model in &model_names {
        if let Some(budget) = config.resolve_skill_index_budget(model) {
            findings.push(check(&aggregated, model, &budget)?);
        }
    }

    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Severity;

    #[test]
    fn test_extract_frontmatter_basic() {
        let content = "---\nname: my-skill\ndescription: A skill\n---\n# Body\n";
        let fm = extract_frontmatter(content).unwrap();
        assert_eq!(fm, "name: my-skill\ndescription: A skill");
    }

    #[test]
    fn test_extract_frontmatter_no_frontmatter() {
        let content = "# Just a heading\nSome content\n";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn test_extract_frontmatter_unclosed() {
        let content = "---\nname: my-skill\nno closing delimiter\n";
        assert!(extract_frontmatter(content).is_none());
    }

    #[test]
    fn test_extract_frontmatter_empty() {
        let content = "---\n---\n# Body\n";
        let fm = extract_frontmatter(content).unwrap();
        assert_eq!(fm, "");
    }

    #[test]
    fn test_extract_frontmatter_bom() {
        let content = "\u{feff}---\nname: bom-skill\n---\n";
        let fm = extract_frontmatter(content).unwrap();
        assert_eq!(fm, "name: bom-skill");
    }

    #[test]
    fn test_check_pass() {
        let budget = ResolvedBudget {
            encoding: "cl100k_base".to_string(),
            warning: 8000,
            error: 12000,
        };
        let finding = check("name: tiny", "gpt-4", &budget).unwrap();
        assert_eq!(finding.severity, Severity::Pass);
        assert_eq!(finding.file, AGGREGATE_LABEL);
    }

    #[test]
    fn test_check_error() {
        let budget = ResolvedBudget {
            encoding: "cl100k_base".to_string(),
            warning: 1,
            error: 2,
        };
        // Generate enough text to exceed 2 tokens
        let big = "word ".repeat(100);
        let finding = check(&big, "gpt-4", &budget).unwrap();
        assert_eq!(finding.severity, Severity::Error);
    }
}
