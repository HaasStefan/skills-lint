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
    "skill-index-budget": {
      "models": {
        "gpt-4o": { "warning": 2000, "error": 4000 },
        "gpt-4": { "warning": 1000, "error": 2000 }
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

## Skill Index Budget

Agent skills are discovered at startup by reading the YAML frontmatter (`name`, `description`) of every SKILL.md file. All frontmatter snippets are concatenated and injected into the model's context window. A project with many verbose skill descriptions can silently consume a large chunk of context before any skill is even activated.

The `skill-index-budget` rule aggregates all SKILL.md frontmatter, counts its tokens per model, and checks against configurable thresholds. It uses the same `models` structure as `token-limit`.

```json
{
  "rules": {
    "skill-index-budget": {
      "models": {
        "gpt-4o": { "warning": 2000, "error": 4000 },
        "gpt-4": { "warning": 1000, "error": 2000 }
      }
    }
  }
}
```

In the output table, the aggregate result appears as a `(skill index)` row. It is not counted in the "across N files" summary.

::: tip
This rule only runs in aggregate mode (all files). It is skipped when using `--file` to lint a single file.
:::

This rule is optional. If the `skill-index-budget` key is omitted, the rule is silently skipped.

## Overrides

Per-file threshold overrides:

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
