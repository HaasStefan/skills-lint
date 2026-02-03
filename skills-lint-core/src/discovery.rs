use crate::errors::LintError;

/// Discover files matching the given glob patterns.
pub fn discover_files(patterns: &[String]) -> Result<Vec<String>, LintError> {
    let mut files = Vec::new();
    for pattern in patterns {
        let paths = glob::glob(pattern)
            .map_err(|e| LintError::GlobPattern(pattern.clone(), e.to_string()))?;
        for entry in paths {
            let path = entry.map_err(|e| LintError::GlobIteration(e.to_string()))?;
            files.push(path.display().to_string());
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_matches_returns_empty() {
        let result = discover_files(&["nonexistent_path_xyz/**/*.md".to_string()]).unwrap();
        assert!(result.is_empty());
    }
}
