mod banner;
mod cli;
mod table;

use std::path::Path;
use std::process;

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use skills_lint_core::config::Config;
use skills_lint_core::lint;
use skills_lint_core::types::{LintFinding, LintReport, Severity};

use cli::Cli;

fn main() {
    let args = Cli::parse();

    if !args.quiet {
        println!();
        banner::print_banner();
    }

    let config = match Config::load(Path::new(&args.config)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} {e}", "error:".red().bold());
            process::exit(3);
        }
    };

    let files = if let Some(ref file) = args.file {
        vec![file.clone()]
    } else {
        match lint::discover(&config) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
    };

    if files.is_empty() {
        println!("{}", "No files found to lint.".dimmed());
        process::exit(0);
    }

    let pb = ProgressBar::new(files.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "  {spinner:.white} Evaluating [{bar:30.white/dim}] {pos}/{len}  {msg}",
        )
        .unwrap()
        .progress_chars("━╸─"),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut all_findings: Vec<LintFinding> = Vec::new();
    for file in &files {
        let short_name = file
            .strip_prefix("./")
            .unwrap_or(file);
        pb.set_message(short_name.to_string());

        match lint::lint_file(&config, file) {
            Ok(findings) => all_findings.extend(findings),
            Err(e) => {
                pb.finish_and_clear();
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    let report = LintReport::new(all_findings);
    println!();
    table::print_report(&report);

    let exit_code = match report.worst_severity() {
        Severity::Error => 1,
        Severity::Warning => 2,
        Severity::Pass => 0,
    };
    process::exit(exit_code);
}
