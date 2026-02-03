use std::path::Path;
use std::process;

use colored::Colorize;
use dialoguer::{Confirm, Input, MultiSelect};
use serde_json::{json, Map, Value};

const CONFIG_PATH: &str = ".skills-lint.config.json";

const MODELS: &[&str] = &[
    "gpt-5",
    "gpt-4o",
    "gpt-4o-mini",
    "gpt-4-turbo",
    "gpt-4",
    "gpt-3.5-turbo",
];

const OPTIONAL_RULES: &[&str] = &[
    "frontmatter-limit",
    "skill-index-budget",
    "skill-structure",
    "unique-name",
    "unique-description",
];

struct ModelBudgets {
    token_limit: (usize, usize),
    frontmatter_limit: (usize, usize),
    skill_index_budget: (usize, usize),
}

fn default_budgets(model: &str) -> ModelBudgets {
    match model {
        "gpt-5" => ModelBudgets {
            token_limit: (16000, 32000),
            frontmatter_limit: (2000, 4000),
            skill_index_budget: (4000, 8000),
        },
        "gpt-4o" | "gpt-4o-mini" | "gpt-4-turbo" => ModelBudgets {
            token_limit: (8000, 16000),
            frontmatter_limit: (1000, 2000),
            skill_index_budget: (2000, 4000),
        },
        "gpt-4" => ModelBudgets {
            token_limit: (2000, 4000),
            frontmatter_limit: (500, 1000),
            skill_index_budget: (1000, 2000),
        },
        "gpt-3.5-turbo" => ModelBudgets {
            token_limit: (4000, 8000),
            frontmatter_limit: (500, 1000),
            skill_index_budget: (1000, 2000),
        },
        _ => ModelBudgets {
            token_limit: (8000, 16000),
            frontmatter_limit: (1000, 2000),
            skill_index_budget: (2000, 4000),
        },
    }
}

pub fn run() {
    println!();
    println!(
        "{}",
        "skills-lint init — configuration wizard".bold()
    );
    println!();

    // 1. Check for existing config
    if Path::new(CONFIG_PATH).exists() {
        let overwrite = Confirm::new()
            .with_prompt(format!("{CONFIG_PATH} already exists. Overwrite?"))
            .default(false)
            .interact()
            .unwrap_or(false);

        if !overwrite {
            println!("{}", "Aborted.".dimmed());
            process::exit(0);
        }
    }

    // 2. Glob pattern
    let pattern: String = Input::new()
        .with_prompt("Glob pattern for skill files")
        .default("./.github/**/SKILL.md".to_string())
        .interact_text()
        .unwrap_or_else(|_| {
            eprintln!("{} failed to read input", "error:".red().bold());
            process::exit(1);
        });

    // 3. Models (all enabled by default)
    let model_selections = MultiSelect::new()
        .with_prompt("Models (space to toggle, enter to confirm)")
        .items(MODELS)
        .defaults(&[true; 6])
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("{} failed to read input", "error:".red().bold());
            process::exit(1);
        });

    if model_selections.is_empty() {
        eprintln!(
            "{} at least one model must be selected",
            "error:".red().bold()
        );
        process::exit(1);
    }

    let selected_models: Vec<&str> = model_selections.iter().map(|&i| MODELS[i]).collect();

    // 4. Rules (all enabled by default)
    let rule_selections = MultiSelect::new()
        .with_prompt("Rules (space to toggle, enter to confirm)")
        .items(OPTIONAL_RULES)
        .defaults(&[true; 5])
        .interact()
        .unwrap_or_else(|_| {
            eprintln!("{} failed to read input", "error:".red().bold());
            process::exit(1);
        });

    let selected_rules: Vec<&str> = rule_selections.iter().map(|&i| OPTIONAL_RULES[i]).collect();

    // 5. Build config
    let config = build_config(&pattern, &selected_models, &selected_rules);

    // 6. Write file
    let json_str = serde_json::to_string_pretty(&config).expect("failed to serialize config");
    match std::fs::write(CONFIG_PATH, format!("{json_str}\n")) {
        Ok(()) => {
            println!();
            println!(
                "{} wrote {CONFIG_PATH}",
                "done:".green().bold()
            );
            update_gitignore();
        }
        Err(e) => {
            eprintln!(
                "{} failed to write {CONFIG_PATH}: {e}",
                "error:".red().bold()
            );
            process::exit(1);
        }
    }
}

fn build_config(pattern: &str, models: &[&str], optional_rules: &[&str]) -> Value {
    let mut rules = Map::new();

    // token-limit is always included
    rules.insert(
        "token-limit".to_string(),
        build_model_budgets(models, |b| b.token_limit),
    );

    for &rule in optional_rules {
        match rule {
            "frontmatter-limit" => {
                rules.insert(
                    "frontmatter-limit".to_string(),
                    build_model_budgets(models, |b| b.frontmatter_limit),
                );
            }
            "skill-index-budget" => {
                rules.insert(
                    "skill-index-budget".to_string(),
                    build_model_budgets(models, |b| b.skill_index_budget),
                );
            }
            "skill-structure" => {
                rules.insert("skill-structure".to_string(), json!(true));
            }
            "unique-name" => {
                rules.insert("unique-name".to_string(), json!(true));
            }
            "unique-description" => {
                rules.insert("unique-description".to_string(), json!(true));
            }
            _ => {}
        }
    }

    json!({
        "patterns": [pattern],
        "rules": rules,
    })
}

const CACHE_IGNORE_ENTRY: &str = ".skills-lint-cache/";

fn update_gitignore() {
    let path = Path::new(".gitignore");
    if !path.exists() {
        println!(
            "{}",
            "tip: add .skills-lint-cache/ to your .gitignore".dimmed()
        );
        return;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            println!(
                "{}",
                "tip: add .skills-lint-cache/ to your .gitignore".dimmed()
            );
            return;
        }
    };

    if content.lines().any(|line| line.trim() == CACHE_IGNORE_ENTRY) {
        return;
    }

    let separator = if content.ends_with('\n') || content.is_empty() {
        ""
    } else {
        "\n"
    };

    match std::fs::write(path, format!("{content}{separator}{CACHE_IGNORE_ENTRY}\n")) {
        Ok(()) => {
            println!(
                "{} added {CACHE_IGNORE_ENTRY} to .gitignore",
                "done:".green().bold()
            );
        }
        Err(_) => {
            println!(
                "{}",
                "tip: add .skills-lint-cache/ to your .gitignore".dimmed()
            );
        }
    }
}

fn build_model_budgets(
    models: &[&str],
    extract: fn(&ModelBudgets) -> (usize, usize),
) -> Value {
    let mut model_map = Map::new();
    for &model in models {
        let budgets = default_budgets(model);
        let (warning, error) = extract(&budgets);
        model_map.insert(
            model.to_string(),
            json!({
                "warning": warning,
                "error": error,
            }),
        );
    }
    json!({ "models": model_map })
}
