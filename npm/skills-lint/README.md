# skills-lint

Token budget linter for agent skill files. Validates that your markdown skill files stay within per-model token limits so they work reliably across LLMs.

## Install

```sh
npm install skills-lint
```

Or run directly:

```sh
npx skills-lint
```

## Usage

```sh
# Lint all files matched by .skills-lint.config.json
skills-lint

# Lint a single file
skills-lint --file path/to/SKILL.md

# Use a custom config
skills-lint --config my-config.json

# Suppress the ASCII banner (useful in CI)
skills-lint --quiet
```

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

Skill files are loaded lazily into the model's context window. The recommended budgets keep skill files to roughly 5–10% of the model's effective input capacity. Encoding is auto-selected from the model name.

### Fields

| Field | Description |
|---|---|
| `patterns` | Glob patterns to discover skill files |
| `rules.token-limit.models` | Per-model token counting configuration |
| `models.<name>.warning` | Token count threshold for warnings |
| `models.<name>.error` | Token count threshold for errors |
| `models.<name>.encoding` | (Optional) override the default tokenizer encoding |

### Per-file overrides

Override thresholds for specific files:

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 16000 }
      }
    }
  },
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

## Exit codes

| Code | Meaning |
|---|---|
| `0` | All checks passed |
| `1` | One or more files exceed the error threshold |
| `2` | One or more files exceed the warning threshold (no errors) |
| `3` | Configuration or runtime error |

## Supported platforms

Pre-built binaries are available for:

| OS | Architecture |
|---|---|
| macOS | ARM64, x64 |
| Linux | ARM64, x64 |
| Windows | ARM64, x64 |

## License

MIT
