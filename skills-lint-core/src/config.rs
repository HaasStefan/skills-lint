use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::errors::LintError;

/// Top-level config loaded from `.skills-lint.config.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub patterns: Vec<String>,
    pub rules: RulesConfig,
    #[serde(default)]
    pub overrides: Vec<OverrideEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RulesConfig {
    #[serde(rename = "token-limit")]
    pub token_limit: TokenLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenLimitConfig {
    pub models: HashMap<String, ModelBudget>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelBudget {
    pub encoding: Option<String>,
    pub warning: usize,
    pub error: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OverrideEntry {
    pub files: Vec<String>,
    pub rules: OverrideRules,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OverrideRules {
    #[serde(rename = "token-limit")]
    pub token_limit: OverrideTokenLimit,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OverrideTokenLimit {
    pub models: HashMap<String, OverrideModelBudget>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OverrideModelBudget {
    pub encoding: Option<String>,
    pub warning: Option<usize>,
    pub error: Option<usize>,
}

impl Config {
    /// Load config from a JSON file.
    pub fn load(path: &Path) -> Result<Self, LintError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LintError::ConfigRead(path.display().to_string(), e))?;
        let config: Config = serde_json::from_str(&content)
            .map_err(|e| LintError::ConfigParse(path.display().to_string(), e))?;
        Ok(config)
    }

    /// Resolve the effective token-limit budget for a given file and model.
    /// Applies overrides on top of the global config.
    pub fn resolve_token_limit(&self, file: &str, model: &str) -> Option<ResolvedBudget> {
        let global = self.rules.token_limit.models.get(model)?;

        let mut encoding = global.encoding.clone();
        let mut warning = global.warning;
        let mut error = global.error;

        for entry in &self.overrides {
            if entry.files.iter().any(|f| f == file) {
                if let Some(ovr) = entry.rules.token_limit.models.get(model) {
                    if let Some(ref enc) = ovr.encoding {
                        encoding = Some(enc.clone());
                    }
                    if let Some(w) = ovr.warning {
                        warning = w;
                    }
                    if let Some(e) = ovr.error {
                        error = e;
                    }
                }
            }
        }

        Some(ResolvedBudget {
            encoding: encoding.unwrap_or_else(|| "cl100k_base".to_string()),
            warning,
            error,
        })
    }
}

/// Fully resolved budget for a file × model pair.
#[derive(Debug, Clone)]
pub struct ResolvedBudget {
    pub encoding: String,
    pub warning: usize,
    pub error: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_global_budget() {
        let json = r#"{
            "patterns": ["*.md"],
            "rules": {
                "token-limit": {
                    "models": {
                        "opus-4.5": { "encoding": "cl100k_base", "warning": 8000, "error": 12000 }
                    }
                }
            }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        let budget = config.resolve_token_limit("foo.md", "opus-4.5").unwrap();
        assert_eq!(budget.warning, 8000);
        assert_eq!(budget.error, 12000);
        assert_eq!(budget.encoding, "cl100k_base");
    }

    #[test]
    fn test_resolve_with_override() {
        let json = r#"{
            "patterns": ["*.md"],
            "rules": {
                "token-limit": {
                    "models": {
                        "opus-4.5": { "encoding": "cl100k_base", "warning": 8000, "error": 12000 }
                    }
                }
            },
            "overrides": [{
                "files": ["big.md"],
                "rules": {
                    "token-limit": {
                        "models": {
                            "opus-4.5": { "warning": 16000, "error": 24000 }
                        }
                    }
                }
            }]
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();

        let normal = config.resolve_token_limit("foo.md", "opus-4.5").unwrap();
        assert_eq!(normal.warning, 8000);

        let overridden = config.resolve_token_limit("big.md", "opus-4.5").unwrap();
        assert_eq!(overridden.warning, 16000);
        assert_eq!(overridden.error, 24000);
        assert_eq!(overridden.encoding, "cl100k_base");
    }

    #[test]
    fn test_unknown_model_returns_none() {
        let json = r#"{
            "patterns": ["*.md"],
            "rules": {
                "token-limit": {
                    "models": {
                        "opus-4.5": { "encoding": "cl100k_base", "warning": 8000, "error": 12000 }
                    }
                }
            }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(config.resolve_token_limit("foo.md", "unknown-model").is_none());
    }
}
