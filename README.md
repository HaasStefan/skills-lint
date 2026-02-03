# skills-lint

Token budget linter for agent skill files. Counts tokens per model encoding and reports warnings/errors when skill files exceed configurable thresholds.

## Installation

```sh
npm install -g @haasstefan/skills-lint
```

Pre-built binaries are available for:

- macOS (ARM64, x64)
- Linux (x64, ARM64)
- Windows (x64, ARM64)

## Usage

Run in a directory containing a `.skills-lint.config.json`:

```sh
skills-lint
```

### Options

| Flag | Description |
|---|---|
| `--file <path>` | Lint a single file instead of using config patterns |
| `--config <path>` | Config file path (default: `.skills-lint.config.json`) |
| `--quiet` | Suppress the ASCII banner (useful for CI) |

### Exit codes

| Code | Meaning |
|---|---|
| `0` | All files pass |
| `1` | At least one error |
| `2` | Warnings only |
| `3` | Runtime error (bad config, file not found, etc.) |

## Configuration

Create a `.skills-lint.config.json` in your project root:

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": {
          "warning": 8000,
          "error": 16000
        },
        "gpt-4": {
          "warning": 2000,
          "error": 4000
        }
      }
    }
  }
}
```

### Supported models

| Model | Context | Max Input | Encoding | Recommended Warning | Recommended Error |
|---|---|---|---|---:|---:|
| `gpt-5` | 400K | 272K | `o200k_base` | 16,000 | 32,000 |
| `gpt-4o` | 128K | 112K | `o200k_base` | 8,000 | 16,000 |
| `gpt-4o-mini` | 128K | 112K | `o200k_base` | 8,000 | 16,000 |
| `gpt-4-turbo` | 128K | 124K | `cl100k_base` | 8,000 | 16,000 |
| `gpt-4` | 8K | 4K | `cl100k_base` | 2,000 | 4,000 |
| `gpt-3.5-turbo` | 16K | 12K | `cl100k_base` | 4,000 | 8,000 |

Skill files are loaded lazily into the model's context window. The recommended budgets keep skill files to roughly 5–10% of the model's effective input capacity. Encoding is auto-selected from the model name and can be overridden with the optional `encoding` field.

### Fields

- **patterns** -- glob patterns to discover skill files
- **rules.token-limit.models** -- map of model name to `{ warning, error }`
  - `warning` -- token count threshold for warnings
  - `error` -- token count threshold for errors
  - `encoding` -- (optional) override the default tokenizer encoding

### Overrides

Apply different thresholds to specific files:

```json
{
  "overrides": [
    {
      "files": [".github/skills/large-skill/SKILL.md"],
      "rules": {
        "token-limit": {
          "models": {
            "gpt-4o": { "warning": 16000, "error": 32000 }
          }
        }
      }
    }
  ]
}
```

## Building from source

```sh
cargo build --release
```

The binary is output to `target/release/skills-lint`.

## License

MIT
