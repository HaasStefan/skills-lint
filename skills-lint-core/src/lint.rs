use std::path::Path;

use crate::config::Config;
use crate::discovery;
use crate::errors::LintError;
use crate::rules::token_limit;
use crate::types::{LintFinding, LintReport};

/// Discover files based on config patterns.
pub fn discover(config: &Config) -> Result<Vec<String>, LintError> {
    discovery::discover_files(&config.patterns)
}

/// Lint a single file against all configured models. Returns findings for that file.
pub fn lint_file(config: &Config, file: &str) -> Result<Vec<LintFinding>, LintError> {
    let content = std::fs::read_to_string(Path::new(file))
        .map_err(|e| LintError::FileRead(file.to_string(), e))?;

    let mut model_names: Vec<&String> = config.rules.token_limit.models.keys().collect();
    model_names.sort();

    let mut findings = Vec::new();
    for model in &model_names {
        if let Some(budget) = config.resolve_token_limit(file, model) {
            let finding = token_limit::check(file, model, &content, &budget)?;
            findings.push(finding);
        }
    }

    Ok(findings)
}

/// Run the full lint pipeline using config-based file discovery.
pub fn run(config: &Config) -> Result<LintReport, LintError> {
    let files = discover(config)?;
    let mut findings = Vec::new();
    for file in &files {
        findings.extend(lint_file(config, file)?);
    }
    Ok(LintReport::new(findings))
}

/// Run the lint pipeline on a single file.
pub fn run_single(config: &Config, file_path: &str) -> Result<LintReport, LintError> {
    let findings = lint_file(config, file_path)?;
    Ok(LintReport::new(findings))
}
