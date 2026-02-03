# skills-lint

Token budget linter for agent skill files. Counts tokens per model encoding and reports warnings/errors when skill files exceed configurable thresholds.

## Installation

```sh
npm install -g skills-lint
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
        "opus-4.5": {
          "encoding": "cl100k_base",
          "warning": 8000,
          "error": 12000
        },
        "gpt-4o": {
          "encoding": "o200k_base",
          "warning": 8000,
          "error": 12000
        }
      }
    }
  }
}
```

### Fields

- **patterns** -- glob patterns to discover skill files
- **rules.token-limit.models** -- map of model name to `{ encoding, warning, error }`
  - `encoding` -- tokenizer encoding (`cl100k_base`, `o200k_base`, etc.)
  - `warning` -- token count threshold for warnings
  - `error` -- token count threshold for errors

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
            "opus-4.5": { "warning": 16000, "error": 24000 }
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
