use thiserror::Error;

#[derive(Debug, Error)]
pub enum LintError {
    #[error("failed to read config file '{0}': {1}")]
    ConfigRead(String, #[source] std::io::Error),

    #[error("failed to parse config file '{0}': {1}")]
    ConfigParse(String, #[source] serde_json::Error),

    #[error("unknown encoding: {0}")]
    UnknownEncoding(String),

    #[error("tokenizer error: {0}")]
    Tokenizer(String),

    #[error("invalid glob pattern '{0}': {1}")]
    GlobPattern(String, String),

    #[error("glob iteration error: {0}")]
    GlobIteration(String),

    #[error("failed to read file '{0}': {1}")]
    FileRead(String, #[source] std::io::Error),
}
