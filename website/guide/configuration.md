# Configuration

Config file: `.skills-lint.config.json` in your project root.

## Example

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 16000 },
        "gpt-4": { "warning": 2000, "error": 4000 }
      }
    },
    "frontmatter-limit": {
      "models": {
        "gpt-4o": { "warning": 1000, "error": 2000 },
        "gpt-4": { "warning": 500, "error": 1000 }
      }
    },
    "skill-index-budget": {
      "models": {
        "gpt-4o": { "warning": 2000, "error": 4000 },
        "gpt-4": { "warning": 1000, "error": 2000 }
      }
    },
    "skill-structure": true,
    "unique-name": true,
    "unique-description": true
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

## Patterns

Glob patterns to find skill files. Relative to the config file.

```json
{ "patterns": ["./.github/**/SKILL.md", "./docs/skills/**/*.md"] }
```

## Models

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `warning` | `number` | Yes | Warning threshold |
| `error` | `number` | Yes | Error threshold |
| `encoding` | `string` | No | Override default encoding (see [Encodings](/reference/encodings)) |

Encoding is auto-selected from the model name.

### Supported models

| Model | Context | Max Input | Max Output | Encoding | Recommended Warning | Recommended Error |
|-------|---------|-----------|------------|----------|--------------------:|------------------:|
| `gpt-5` | 400K | 272K | 128K | `o200k_base` | 16,000 | 32,000 |
| `gpt-4o` | 128K | 112K | 16K | `o200k_base` | 8,000 | 16,000 |
| `gpt-4o-mini` | 128K | 112K | 16K | `o200k_base` | 8,000 | 16,000 |
| `gpt-4-turbo` | 128K | 124K | 4K | `cl100k_base` | 8,000 | 16,000 |
| `gpt-4` | 8K | 4K | 4K | `cl100k_base` | 2,000 | 4,000 |
| `gpt-3.5-turbo` | 16K | 12K | 4K | `cl100k_base` | 4,000 | 8,000 |

Skill files are loaded lazily into the model's context window when activated. The recommended budgets keep skill files to roughly 5–10% of the model's effective input capacity, leaving room for system instructions, conversation history, and output. Tighter budgets on `gpt-4` and `gpt-3.5-turbo` reflect their smaller context windows.

Unsupported model names are rejected at config load.

## Rules

See the [Rules](/guide/rules) page for detailed documentation of all six rules: `token-limit`, `frontmatter-limit`, `skill-index-budget`, `skill-structure`, `unique-name`, and `unique-description`.

## Overrides

Per-file threshold overrides (applies to `token-limit` only):

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

- `files` — exact paths (not globs)
- Only specified fields are overridden; the rest inherit from global config
- Overrides are applied in order
- Unlisted models keep global values
