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
          "error": 12000
        },
        "gpt-4": {
          "warning": 8000,
          "error": 12000
        }
      }
    }
  }
}
```

### Supported models

| Model | Encoding |
|---|---|
| `gpt-5` | `o200k_base` |
| `gpt-4o` | `o200k_base` |
| `gpt-4o-mini` | `o200k_base` |
| `gpt-4-turbo` | `cl100k_base` |
| `gpt-4` | `cl100k_base` |
| `gpt-3.5-turbo` | `cl100k_base` |

The encoding is automatically selected based on the model name. You can override it with the optional `encoding` field.

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
            "gpt-4o": { "warning": 16000, "error": 24000 }
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
