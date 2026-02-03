use std::path::Path;

use crate::config::Config;
use crate::discovery;
use crate::errors::LintError;
use crate::rules::{frontmatter_limit, skill_index_budget, skill_structure, token_limit, unique_fields};
use crate::types::{LintFinding, LintReport, StructureFinding};

/// Discover files based on config patterns.
pub fn discover(config: &Config) -> Result<Vec<String>, LintError> {
    discovery::discover_files(&config.patterns)
}

/// Lint a single file against all configured models. Returns token findings for that file.
pub fn lint_file(config: &Config, file: &str) -> Result<Vec<LintFinding>, LintError> {
    let content = std::fs::read_to_string(Path::new(file))
        .map_err(|e| LintError::FileRead(file.to_string(), e))?;

    let mut model_names: Vec<&String> = config.rules.token_limit.models.keys().collect();
    model_names.sort();

    let mut findings = Vec::new();
    for model in &model_names {
        if let Some(budget) = config.resolve_token_limit(file, model) {
            let finding = token_limit::check("token-limit", file, model, &content, &budget)?;
            findings.push(finding);
        }
    }

    findings.extend(frontmatter_limit::check_file(config, file)?);

    Ok(findings)
}

/// Check a single file for structural validity. Returns a finding if the rule is enabled.
pub fn check_structure(config: &Config, file: &str) -> Result<Option<StructureFinding>, LintError> {
    if config.rules.skill_structure != Some(true) {
        return Ok(None);
    }

    let content = std::fs::read_to_string(Path::new(file))
        .map_err(|e| LintError::FileRead(file.to_string(), e))?;

    Ok(Some(skill_structure::lint_file(file, &content)))
}

/// Run the full lint pipeline using config-based file discovery.
pub fn run(config: &Config) -> Result<LintReport, LintError> {
    let files = discover(config)?;
    let mut findings = Vec::new();
    let mut structure_findings = Vec::new();

    for file in &files {
        findings.extend(lint_file(config, file)?);
        if let Some(sf) = check_structure(config, file)? {
            structure_findings.push(sf);
        }
    }

    findings.extend(skill_index_budget::check_all(config, &files)?);
    structure_findings.extend(unique_fields::check_all(config, &files)?);
    Ok(LintReport::new(findings, structure_findings))
}

/// Run the lint pipeline on a single file.
pub fn run_single(config: &Config, file_path: &str) -> Result<LintReport, LintError> {
    let findings = lint_file(config, file_path)?;
    let mut structure_findings = Vec::new();
    if let Some(sf) = check_structure(config, file_path)? {
        structure_findings.push(sf);
    }
    Ok(LintReport::new(findings, structure_findings))
}
