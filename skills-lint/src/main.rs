mod banner;
mod cli;
mod init;
mod table;

use std::path::Path;
use std::process;

use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use skills_lint_core::cache::TokenCache;
use skills_lint_core::config::Config;
use skills_lint_core::lint;
use skills_lint_core::rules::{skill_index_budget, unique_fields};
use skills_lint_core::types::{LintFinding, LintReport, Severity, StructureFinding};

use cli::{Cli, Command};

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Command::Init) => init::run(),
        None => run_lint(args),
    }
}

fn run_lint(args: Cli) {
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

    let use_cache = config.cache && !args.no_cache;
    let mut cache = if use_cache {
        Some(TokenCache::load())
    } else {
        None
    };

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
    let mut all_structure_findings: Vec<StructureFinding> = Vec::new();
    for file in &files {
        let short_name = file
            .strip_prefix("./")
            .unwrap_or(file);
        pb.set_message(short_name.to_string());

        match lint::lint_file(&config, file, cache.as_mut()) {
            Ok(findings) => all_findings.extend(findings),
            Err(e) => {
                pb.finish_and_clear();
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
        match lint::check_structure(&config, file) {
            Ok(Some(sf)) => all_structure_findings.push(sf),
            Ok(None) => {}
            Err(e) => {
                pb.finish_and_clear();
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    if args.file.is_none() {
        match skill_index_budget::check_all(&config, &files, cache.as_mut()) {
            Ok(findings) => all_findings.extend(findings),
            Err(e) => {
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
        match unique_fields::check_all(&config, &files) {
            Ok(findings) => all_structure_findings.extend(findings),
            Err(e) => {
                eprintln!("{} {e}", "error:".red().bold());
                process::exit(3);
            }
        }
    }

    if let Some(ref c) = cache {
        c.flush();
    }

    let report = LintReport::new(all_findings, all_structure_findings);
    println!();
    table::print_report(&report, args.verbose);

    let exit_code = match report.worst_severity() {
        Severity::Error => 1,
        Severity::Warning => 2,
        Severity::Pass => 0,
    };
    process::exit(exit_code);
}
