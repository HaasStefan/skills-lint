# Config Schema

Reference for `.skills-lint.config.json`.

## Top-Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `patterns` | `string[]` | Yes | Glob patterns for skill files |
| `rules` | `object` | Yes | Rule config |
| `overrides` | `object[]` | No | Per-file overrides |

## `rules.token-limit.models.<name>`

Model name must be one of: `gpt-5`, `gpt-4o`, `gpt-4o-mini`, `gpt-4-turbo`, `gpt-4`, `gpt-3.5-turbo`.

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `warning` | `number` | Yes | — | Warning threshold |
| `error` | `number` | Yes | — | Error threshold |
| `encoding` | `string` | No | Auto | Encoding override |

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
        "gpt-4o": { "warning": 8000, "error": 12000 }
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
        "gpt-4o": { "warning": 8000, "error": 12000 },
        "gpt-4": { "warning": 8000, "error": 12000 }
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
