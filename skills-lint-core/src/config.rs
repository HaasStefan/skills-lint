use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::errors::LintError;

/// Supported models and their default encodings.
const SUPPORTED_MODELS: &[(&str, &str)] = &[
    ("gpt-5", "o200k_base"),
    ("gpt-4o", "o200k_base"),
    ("gpt-4o-mini", "o200k_base"),
    ("gpt-4-turbo", "cl100k_base"),
    ("gpt-4", "cl100k_base"),
    ("gpt-3.5-turbo", "cl100k_base"),
];

/// Return the default encoding for a supported model, or None if unsupported.
pub fn default_encoding(model: &str) -> Option<&'static str> {
    SUPPORTED_MODELS
        .iter()
        .find(|(name, _)| *name == model)
        .map(|(_, enc)| *enc)
}

/// Return a list of all supported model names.
pub fn supported_model_names() -> Vec<&'static str> {
    SUPPORTED_MODELS.iter().map(|(name, _)| *name).collect()
}

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
        config.validate()?;
        Ok(config)
    }

    /// Validate that all model names in the config are supported.
    fn validate(&self) -> Result<(), LintError> {
        for model in self.rules.token_limit.models.keys() {
            if default_encoding(model).is_none() {
                return Err(LintError::UnsupportedModel(
                    model.clone(),
                    supported_model_names(),
                ));
            }
        }
        for entry in &self.overrides {
            for model in entry.rules.token_limit.models.keys() {
                if default_encoding(model).is_none() {
                    return Err(LintError::UnsupportedModel(
                        model.clone(),
                        supported_model_names(),
                    ));
                }
            }
        }
        Ok(())
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

        let default_enc = default_encoding(model)
            .unwrap_or("cl100k_base")
            .to_string();

        Some(ResolvedBudget {
            encoding: encoding.unwrap_or(default_enc),
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
                        "gpt-4": { "warning": 8000, "error": 12000 }
                    }
                }
            }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        let budget = config.resolve_token_limit("foo.md", "gpt-4").unwrap();
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
                        "gpt-4o": { "warning": 8000, "error": 12000 }
                    }
                }
            },
            "overrides": [{
                "files": ["big.md"],
                "rules": {
                    "token-limit": {
                        "models": {
                            "gpt-4o": { "warning": 16000, "error": 24000 }
                        }
                    }
                }
            }]
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();

        let normal = config.resolve_token_limit("foo.md", "gpt-4o").unwrap();
        assert_eq!(normal.warning, 8000);
        assert_eq!(normal.encoding, "o200k_base");

        let overridden = config.resolve_token_limit("big.md", "gpt-4o").unwrap();
        assert_eq!(overridden.warning, 16000);
        assert_eq!(overridden.error, 24000);
        assert_eq!(overridden.encoding, "o200k_base");
    }

    #[test]
    fn test_unknown_model_returns_none() {
        let json = r#"{
            "patterns": ["*.md"],
            "rules": {
                "token-limit": {
                    "models": {
                        "gpt-4": { "warning": 8000, "error": 12000 }
                    }
                }
            }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(config.resolve_token_limit("foo.md", "unknown-model").is_none());
    }

    #[test]
    fn test_default_encoding_lookup() {
        assert_eq!(default_encoding("gpt-5"), Some("o200k_base"));
        assert_eq!(default_encoding("gpt-4o"), Some("o200k_base"));
        assert_eq!(default_encoding("gpt-4o-mini"), Some("o200k_base"));
        assert_eq!(default_encoding("gpt-4"), Some("cl100k_base"));
        assert_eq!(default_encoding("gpt-4-turbo"), Some("cl100k_base"));
        assert_eq!(default_encoding("gpt-3.5-turbo"), Some("cl100k_base"));
        assert_eq!(default_encoding("unknown"), None);
    }

    #[test]
    fn test_validate_rejects_unsupported_model() {
        let json = r#"{
            "patterns": ["*.md"],
            "rules": {
                "token-limit": {
                    "models": {
                        "not-a-real-model": { "warning": 8000, "error": 12000 }
                    }
                }
            }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(config.validate().is_err());
    }
}
