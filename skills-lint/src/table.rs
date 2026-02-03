use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use skills_lint_core::rules::skill_index_budget::AGGREGATE_LABEL;
use skills_lint_core::types::{LintReport, Severity};

fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, ch) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }
    result.chars().rev().collect()
}

fn severity_color(severity: Severity) -> Color {
    match severity {
        Severity::Pass => Color::Green,
        Severity::Warning => Color::Yellow,
        Severity::Error => Color::Red,
    }
}

fn status_text(severity: Severity) -> &'static str {
    match severity {
        Severity::Pass => "✓ PASS",
        Severity::Warning => "⚠ WARN",
        Severity::Error => "✗ ERROR",
    }
}

fn colored_rule_name(name: &str, severity: Severity) -> String {
    match severity {
        Severity::Pass => name.green().to_string(),
        Severity::Warning => name.yellow().bold().to_string(),
        Severity::Error => name.red().bold().to_string(),
    }
}

fn colored_status(severity: Severity) -> String {
    let text = status_text(severity);
    match severity {
        Severity::Pass => text.green().to_string(),
        Severity::Warning => text.yellow().bold().to_string(),
        Severity::Error => text.red().bold().to_string(),
    }
}

/// Build a token-limit sub-table (no File column) for a set of findings.
fn build_token_table(findings: &[&skills_lint_core::types::LintFinding]) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Model").add_attribute(Attribute::Bold),
            Cell::new("Tokens")
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
            Cell::new("Warning")
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
            Cell::new("Error")
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
            Cell::new("Status").add_attribute(Attribute::Bold),
        ]);

    for finding in findings {
        let color = severity_color(finding.severity);

        let mut tokens_cell = Cell::new(format_number(finding.token_count))
            .set_alignment(CellAlignment::Right)
            .fg(color);
        if finding.severity == Severity::Error {
            tokens_cell = tokens_cell.add_attribute(Attribute::Bold);
        }

        let mut status_cell = Cell::new(status_text(finding.severity)).fg(color);
        if finding.severity != Severity::Pass {
            status_cell = status_cell.add_attribute(Attribute::Bold);
        }

        table.add_row(vec![
            Cell::new(&finding.model),
            tokens_cell,
            Cell::new(format_number(finding.warning_threshold))
                .set_alignment(CellAlignment::Right)
                .fg(Color::DarkGrey),
            Cell::new(format_number(finding.error_threshold))
                .set_alignment(CellAlignment::Right)
                .fg(Color::DarkGrey),
            status_cell,
        ]);
    }

    table
}

/// Returns true if the severity should be shown in non-verbose mode.
fn is_notable(severity: Severity) -> bool {
    matches!(severity, Severity::Warning | Severity::Error)
}

pub fn print_report(report: &LintReport, verbose: bool) {
    if report.findings.is_empty() && report.structure_findings.is_empty() {
        println!("  {}", "No files found to lint.".dimmed());
        return;
    }

    // Collect unique file paths in order (excluding aggregate label).
    let mut file_paths: Vec<&str> = Vec::new();
    for f in &report.structure_findings {
        if !file_paths.contains(&f.file.as_str()) {
            file_paths.push(&f.file);
        }
    }
    for f in &report.findings {
        if f.file != AGGREGATE_LABEL && !file_paths.contains(&f.file.as_str()) {
            file_paths.push(&f.file);
        }
    }

    let has_aggregate = report.findings.iter().any(|f| f.file == AGGREGATE_LABEL);

    // In non-verbose mode, filter to only sections with issues.
    let file_paths: Vec<&str> = if verbose {
        file_paths
    } else {
        file_paths
            .into_iter()
            .filter(|path| {
                let structure_notable = report
                    .structure_findings
                    .iter()
                    .any(|f| f.file == *path && is_notable(f.severity));
                let token_notable = report
                    .findings
                    .iter()
                    .any(|f| f.file == *path && is_notable(f.severity));
                structure_notable || token_notable
            })
            .collect()
    };

    let show_aggregate = has_aggregate
        && (verbose
            || report
                .findings
                .iter()
                .any(|f| f.file == AGGREGATE_LABEL && is_notable(f.severity)));

    let total_sections = file_paths.len() + if show_aggregate { 1 } else { 0 };
    let mut section_idx = 0;

    // Print each file section.
    for file_path in &file_paths {
        println!("  {}", file_path.bold());

        // Gather which rules apply to this file.
        let structure = report
            .structure_findings
            .iter()
            .find(|f| f.file == *file_path);
        let token_findings: Vec<&_> = report
            .findings
            .iter()
            .filter(|f| f.file == *file_path)
            .collect();

        // In non-verbose mode, only show non-pass token rows.
        let visible_token_findings: Vec<&_> = if verbose {
            token_findings.clone()
        } else {
            token_findings
                .iter()
                .filter(|f| is_notable(f.severity))
                .copied()
                .collect()
        };

        let show_structure = structure
            .map(|sf| verbose || is_notable(sf.severity))
            .unwrap_or(false);
        let has_visible_tokens = !visible_token_findings.is_empty();

        // Structure finding (inline rule).
        if let Some(sf) = structure {
            if show_structure {
                let is_last = !has_visible_tokens;
                let connector = if is_last { "└─" } else { "├─" };
                println!(
                    "  {} {}   {}   {}",
                    connector.dimmed(),
                    colored_rule_name("skill-structure", sf.severity),
                    sf.message,
                    colored_status(sf.severity),
                );
                if !is_last {
                    println!("  {}", "│".dimmed());
                }
            }
        }

        // Token-limit findings (sub-table rule).
        if has_visible_tokens {
            let worst = visible_token_findings
                .iter()
                .map(|f| f.severity)
                .max()
                .unwrap_or(Severity::Pass);
            println!(
                "  {} {}",
                "└─".dimmed(),
                colored_rule_name("token-limit", worst),
            );

            let table = build_token_table(&visible_token_findings);
            for line in table.to_string().lines() {
                println!("     {line}");
            }
        }

        section_idx += 1;
        println!();
        if section_idx < total_sections {
            println!("  {}", "─".repeat(50).dimmed());
            println!();
        }
    }

    // Aggregate (skill index) section.
    if show_aggregate {
        let aggregate_findings: Vec<&_> = report
            .findings
            .iter()
            .filter(|f| f.file == AGGREGATE_LABEL)
            .collect();

        let visible_aggregate: Vec<&_> = if verbose {
            aggregate_findings
        } else {
            aggregate_findings
                .into_iter()
                .filter(|f| is_notable(f.severity))
                .collect()
        };

        println!("  {}", AGGREGATE_LABEL.bold());

        let worst = visible_aggregate
            .iter()
            .map(|f| f.severity)
            .max()
            .unwrap_or(Severity::Pass);
        println!(
            "  {} {}",
            "└─".dimmed(),
            colored_rule_name("skill-index-budget", worst),
        );

        let table = build_token_table(&visible_aggregate);
        for line in table.to_string().lines() {
            println!("     {line}");
        }
        println!();
    }

    // Summary — count across both finding types.
    let total_token = report.findings.len();
    let total_structure = report.structure_findings.len();
    let total = total_token + total_structure;

    let errors = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count()
        + report
            .structure_findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
    let warnings = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Warning)
        .count()
        + report
            .structure_findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .count();
    let passed = total - errors - warnings;

    let unique_files = {
        let mut files: Vec<&str> = report
            .findings
            .iter()
            .map(|f| f.file.as_str())
            .filter(|f| *f != AGGREGATE_LABEL)
            .chain(report.structure_findings.iter().map(|f| f.file.as_str()))
            .collect();
        files.sort();
        files.dedup();
        files.len()
    };

    let mut parts = Vec::new();
    if passed > 0 {
        parts.push(format!("{}", format!("{passed} passed").green()));
    }
    if warnings > 0 {
        parts.push(format!("{}", format!("{warnings} warnings").yellow()));
    }
    if errors > 0 {
        parts.push(format!("{}", format!("{errors} errors").red().bold()));
    }

    println!(
        "  {} {} across {} {}",
        "Results:".bold(),
        parts.join(", "),
        unique_files,
        if unique_files == 1 { "file" } else { "files" }
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(12345), "12,345");
        assert_eq!(format_number(1234567), "1,234,567");
    }
}
