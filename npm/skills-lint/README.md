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
        "opus-4.5": {
          "encoding": "cl100k_base",
          "warning": 8000,
          "error": 12000
        },
        "sonnet-4.5": {
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

| Field | Description |
|---|---|
| `patterns` | Glob patterns to discover skill files |
| `rules.token-limit.models` | Per-model token counting configuration |
| `models.<name>.encoding` | Tokenizer encoding (`cl100k_base`, `o200k_base`, `p50k_base`, `r50k_base`) |
| `models.<name>.warning` | Token count threshold for warnings |
| `models.<name>.error` | Token count threshold for errors |

### Per-file overrides

Override thresholds for specific files:

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "rules": {
    "token-limit": {
      "models": {
        "opus-4.5": { "encoding": "cl100k_base", "warning": 8000, "error": 12000 }
      }
    }
  },
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
