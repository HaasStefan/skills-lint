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

The encoding is automatically selected based on the model name.

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
        "gpt-4o": { "warning": 8000, "error": 12000 }
      }
    }
  },
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
