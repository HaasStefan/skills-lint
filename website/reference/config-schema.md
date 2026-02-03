# Config Schema

Reference for `.skills-lint.config.json`.

## Top-Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `patterns` | `string[]` | Yes | Glob patterns for skill files |
| `rules` | `object` | Yes | Rule config (see below) |
| `overrides` | `object[]` | No | Per-file overrides (applies to `token-limit` only) |

## `rules.token-limit.models.<name>`

Model name must be one of: `gpt-5`, `gpt-4o`, `gpt-4o-mini`, `gpt-4-turbo`, `gpt-4`, `gpt-3.5-turbo`.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `warning` | `number` | Yes | — | Warning threshold |
| `error` | `number` | Yes | — | Error threshold |
| `encoding` | `string` | No | Auto | Encoding override |

## `rules.skill-index-budget` <span style="font-weight:normal; font-size:0.85em">(optional)</span>

Aggregates YAML frontmatter from all discovered SKILL.md files and checks the combined token count per model. Same schema as `token-limit` — no per-file overrides.

### `rules.skill-index-budget.models.<name>`

Model name must be one of: `gpt-5`, `gpt-4o`, `gpt-4o-mini`, `gpt-4-turbo`, `gpt-4`, `gpt-3.5-turbo`.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `warning` | `number` | Yes | — | Warning threshold for aggregate frontmatter |
| `error` | `number` | Yes | — | Error threshold for aggregate frontmatter |
| `encoding` | `string` | No | Auto | Encoding override |

Omit the entire `skill-index-budget` key to disable the rule. Skipped when using `--file`.

## `rules.skill-structure` <span style="font-weight:normal; font-size:0.85em">(optional)</span>

Validates the structure of each SKILL.md file: frontmatter presence, `name` and `description` fields, and non-empty body.

| Value | Behavior |
|-------|----------|
| `true` | Rule is enabled |
| `false` or absent | Rule is skipped |

Runs in both aggregate and `--file` modes.

## `overrides[]`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `files` | `string[]` | Yes | Exact file paths |
| `rules` | `object` | Yes | Same structure as top-level `rules` |

Override fields are optional — unspecified fields inherit from global config.

## Minimal

```json
{
  "patterns": ["./skills/**/*.md"],
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 16000 }
      }
    }
  }
}
```

## Full

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
    },
    "skill-structure": true
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
