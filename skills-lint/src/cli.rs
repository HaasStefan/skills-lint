use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "skills-lint", version, about = "Lint agent skill markdown files against per-model token budgets")]
pub struct Cli {
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
}
