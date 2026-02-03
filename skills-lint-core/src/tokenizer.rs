use tiktoken_rs::CoreBPE;

use crate::errors::LintError;

/// Get a tiktoken BPE encoder by encoding name.
pub fn get_encoding(name: &str) -> Result<CoreBPE, LintError> {
    match name {
        "cl100k_base" => tiktoken_rs::cl100k_base(),
        "o200k_base" => tiktoken_rs::o200k_base(),
        "p50k_base" => tiktoken_rs::p50k_base(),
        "r50k_base" => tiktoken_rs::r50k_base(),
        other => return Err(LintError::UnknownEncoding(other.to_string())),
    }
    .map_err(|e| LintError::Tokenizer(e.to_string()))
}

/// Count tokens in the given text using the specified encoding name.
pub fn count_tokens(text: &str, encoding_name: &str) -> Result<usize, LintError> {
    let bpe = get_encoding(encoding_name)?;
    Ok(bpe.encode_with_special_tokens(text).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tokens_cl100k() {
        let count = count_tokens("Hello, world!", "cl100k_base").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_count_tokens_o200k() {
        let count = count_tokens("Hello, world!", "o200k_base").unwrap();
        assert!(count > 0);
    }

    #[test]
    fn test_unknown_encoding() {
        let result = count_tokens("test", "nonexistent_encoding");
        assert!(result.is_err());
    }
}
