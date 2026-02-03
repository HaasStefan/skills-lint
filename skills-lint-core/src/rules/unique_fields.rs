use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;
use crate::errors::LintError;
use crate::rules::skill_index_budget::extract_frontmatter;
use crate::types::{Severity, StructureFinding};

/// Extract a frontmatter field value (e.g. `name: foo` -> `foo`).
fn extract_field<'a>(frontmatter: &'a str, field: &str) -> Option<&'a str> {
    let prefix = format!("{field}:");
    for line in frontmatter.lines() {
        if let Some(rest) = line.strip_prefix(&prefix) {
            let value = rest.trim();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }
    None
}

/// Check all discovered files for duplicate names and descriptions.
///
/// Returns an empty vec if neither rule is configured.
pub fn check_all(config: &Config, files: &[String]) -> Result<Vec<StructureFinding>, LintError> {
    let check_name = config.rules.unique_name == Some(true);
    let check_desc = config.rules.unique_description == Some(true);

    if !check_name && !check_desc {
        return Ok(Vec::new());
    }

    // Map of field value -> list of file paths that have that value.
    let mut names: HashMap<String, Vec<String>> = HashMap::new();
    let mut descriptions: HashMap<String, Vec<String>> = HashMap::new();

    for file in files {
        let content = std::fs::read_to_string(Path::new(file))
            .map_err(|e| LintError::FileRead(file.clone(), e))?;

        let fm = match extract_frontmatter(&content) {
            Some(fm) => fm,
            None => continue,
        };

        if check_name {
            if let Some(name) = extract_field(&fm, "name") {
                names
                    .entry(name.to_string())
                    .or_default()
                    .push(file.clone());
            }
        }

        if check_desc {
            if let Some(desc) = extract_field(&fm, "description") {
                descriptions
                    .entry(desc.to_string())
                    .or_default()
                    .push(file.clone());
            }
        }
    }

    // Build a set of duplicate values for quick lookup.
    let duplicate_names: HashMap<&str, &Vec<String>> = if check_name {
        names
            .iter()
            .filter(|(_, fs)| fs.len() > 1)
            .map(|(k, v)| (k.as_str(), v))
            .collect()
    } else {
        HashMap::new()
    };

    let duplicate_descs: HashMap<&str, &Vec<String>> = if check_desc {
        descriptions
            .iter()
            .filter(|(_, fs)| fs.len() > 1)
            .map(|(k, v)| (k.as_str(), v))
            .collect()
    } else {
        HashMap::new()
    };

    let mut findings = Vec::new();

    // Produce a finding per file, in file order.
    for file in files {
        let content = std::fs::read_to_string(Path::new(file))
            .map_err(|e| LintError::FileRead(file.clone(), e))?;

        let fm = match extract_frontmatter(&content) {
            Some(fm) => fm,
            None => continue,
        };

        if check_name {
            if let Some(name) = extract_field(&fm, "name") {
                if let Some(dupe_files) = duplicate_names.get(name) {
                    let others: Vec<&str> = dupe_files
                        .iter()
                        .filter(|f| *f != file)
                        .map(|f| f.as_str())
                        .collect();
                    findings.push(StructureFinding {
                        rule: "unique-name".to_string(),
                        file: file.clone(),
                        message: format!(
                            "duplicate name \"{}\" (also in {})",
                            name,
                            others.join(", ")
                        ),
                        severity: Severity::Error,
                    });
                } else {
                    findings.push(StructureFinding {
                        rule: "unique-name".to_string(),
                        file: file.clone(),
                        message: "unique".to_string(),
                        severity: Severity::Pass,
                    });
                }
            }
        }

        if check_desc {
            if let Some(desc) = extract_field(&fm, "description") {
                if let Some(dupe_files) = duplicate_descs.get(desc) {
                    let truncated = if desc.len() > 40 {
                        format!("{}...", &desc[..40])
                    } else {
                        desc.to_string()
                    };
                    let others: Vec<&str> = dupe_files
                        .iter()
                        .filter(|f| *f != file)
                        .map(|f| f.as_str())
                        .collect();
                    findings.push(StructureFinding {
                        rule: "unique-description".to_string(),
                        file: file.clone(),
                        message: format!(
                            "duplicate description \"{}\" (also in {})",
                            truncated,
                            others.join(", ")
                        ),
                        severity: Severity::Error,
                    });
                } else {
                    findings.push(StructureFinding {
                        rule: "unique-description".to_string(),
                        file: file.clone(),
                        message: "unique".to_string(),
                        severity: Severity::Pass,
                    });
                }
            }
        }
    }

    Ok(findings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_field_name() {
        let fm = "name: my-skill\ndescription: A skill";
        assert_eq!(extract_field(fm, "name"), Some("my-skill"));
        assert_eq!(extract_field(fm, "description"), Some("A skill"));
    }

    #[test]
    fn test_extract_field_missing() {
        let fm = "name: my-skill";
        assert_eq!(extract_field(fm, "description"), None);
    }

    #[test]
    fn test_extract_field_empty_value() {
        let fm = "name:";
        assert_eq!(extract_field(fm, "name"), None);
    }

    #[test]
    fn test_extract_field_whitespace_value() {
        let fm = "name:   ";
        assert_eq!(extract_field(fm, "name"), None);
    }
}
