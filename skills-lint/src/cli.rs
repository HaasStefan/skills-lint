use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "skills-lint", version, about = "Lint agent skill markdown files against per-model token budgets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Lint a single file instead of using config patterns
    #[arg(long)]
    pub file: Option<String>,

    /// Config file path
    #[arg(long, default_value = ".skills-lint.config.json")]
    pub config: String,

    /// Suppress the ASCII banner (for CI)
    #[arg(long)]
    pub quiet: bool,

    /// Show all findings including passing rules
    #[arg(long)]
    pub verbose: bool,

    /// Disable token-count caching
    #[arg(long)]
    pub no_cache: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a new .skills-lint.config.json
    Init,
}
