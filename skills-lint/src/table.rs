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

pub fn print_report(report: &LintReport) {
    if report.findings.is_empty() {
        println!("  {}", "No files found to lint.".dimmed());
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("File").add_attribute(Attribute::Bold),
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

    let mut prev_file: Option<&str> = None;
    for finding in &report.findings {
        let file_cell = if prev_file == Some(&finding.file) {
            Cell::new("  ┆").fg(Color::DarkGrey)
        } else {
            Cell::new(&finding.file)
        };
        prev_file = Some(&finding.file);

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
            file_cell,
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

    println!("{table}");

    // Summary
    let total = report.findings.len();
    let errors = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = report
        .findings
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
            .collect();
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

    println!();
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
