use crate::rules::skill_index_budget::extract_frontmatter;
use crate::types::{Severity, StructureFinding};

/// Check skill file structure. Returns a list of error messages (empty if valid).
pub fn check_file(content: &str) -> Vec<String> {
    let mut issues = Vec::new();

    let fm = match extract_frontmatter(content) {
        Some(fm) => fm,
        None => {
            issues.push("invalid frontmatter".to_string());
            return issues;
        }
    };

    let has_name = fm.lines().any(|line| {
        if let Some(rest) = line.strip_prefix("name:") {
            !rest.trim().is_empty()
        } else {
            false
        }
    });
    if !has_name {
        issues.push("missing name".to_string());
    }

    let has_description = fm.lines().any(|line| {
        if let Some(rest) = line.strip_prefix("description:") {
            !rest.trim().is_empty()
        } else {
            false
        }
    });
    if !has_description {
        issues.push("missing description".to_string());
    }

    // Body is everything after the closing `---`
    let content_stripped = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut lines = content_stripped.lines();
    // Skip opening ---
    let mut found_close = false;
    if let Some(first) = lines.next() {
        if first.trim() == "---" {
            for line in lines.by_ref() {
                if line.trim() == "---" {
                    found_close = true;
                    break;
                }
            }
        }
    }

    if found_close {
        let body: String = lines.collect::<Vec<_>>().join("\n");
        if body.trim().is_empty() {
            issues.push("empty body".to_string());
        }
    } else {
        // No closing delimiter means no body either, but we already flagged missing frontmatter
        issues.push("empty body".to_string());
    }

    issues
}

/// Lint a single file for structural validity.
pub fn lint_file(file: &str, content: &str) -> StructureFinding {
    let issues = check_file(content);
    if issues.is_empty() {
        StructureFinding {
            rule: "skill-structure".to_string(),
            file: file.to_string(),
            message: "valid".to_string(),
            severity: Severity::Pass,
        }
    } else {
        StructureFinding {
            rule: "skill-structure".to_string(),
            file: file.to_string(),
            message: issues.join(", "),
            severity: Severity::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_file() {
        let content = "---\nname: my-skill\ndescription: A useful skill\n---\n# Heading\nBody content\n";
        let issues = check_file(content);
        assert!(issues.is_empty());

        let finding = lint_file("test.md", content);
        assert_eq!(finding.severity, Severity::Pass);
        assert_eq!(finding.message, "valid");
    }

    #[test]
    fn test_missing_frontmatter() {
        let content = "# Just a heading\nSome content\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["invalid frontmatter"]);

        let finding = lint_file("test.md", content);
        assert_eq!(finding.severity, Severity::Error);
        assert!(finding.message.contains("invalid frontmatter"));
    }

    #[test]
    fn test_missing_name() {
        let content = "---\ndescription: A skill\n---\n# Body\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["missing name"]);
    }

    #[test]
    fn test_missing_description() {
        let content = "---\nname: my-skill\n---\n# Body\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["missing description"]);
    }

    #[test]
    fn test_empty_body() {
        let content = "---\nname: my-skill\ndescription: A skill\n---\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["empty body"]);
    }

    #[test]
    fn test_empty_body_whitespace_only() {
        let content = "---\nname: my-skill\ndescription: A skill\n---\n   \n  \n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["empty body"]);
    }

    #[test]
    fn test_multiple_issues() {
        let content = "---\n---\n";
        let issues = check_file(content);
        assert!(issues.contains(&"missing name".to_string()));
        assert!(issues.contains(&"missing description".to_string()));
        assert!(issues.contains(&"empty body".to_string()));

        let finding = lint_file("test.md", content);
        assert_eq!(finding.severity, Severity::Error);
        assert_eq!(finding.message, "missing name, missing description, empty body");
    }

    #[test]
    fn test_empty_name_value() {
        let content = "---\nname:\ndescription: A skill\n---\n# Body\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["missing name"]);
    }

    #[test]
    fn test_empty_description_value() {
        let content = "---\nname: my-skill\ndescription:   \n---\n# Body\n";
        let issues = check_file(content);
        assert_eq!(issues, vec!["missing description"]);
    }
}
