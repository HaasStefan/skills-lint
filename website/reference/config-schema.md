# Config Schema

Full reference for `.skills-lint.config.json`.

## Top-Level

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `patterns` | `string[]` | Yes | Glob patterns to discover skill files |
| `rules` | `object` | Yes | Rule configurations |
| `overrides` | `object[]` | No | Per-file threshold overrides |

## `rules`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token-limit` | `object` | Yes | Token limit rule configuration |

## `rules.token-limit`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `models` | `object` | Yes | Map of model name → budget configuration |

## `rules.token-limit.models.<name>`

Each model entry defines the encoding and thresholds:

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `encoding` | `string` | No | `"cl100k_base"` | Tokenizer encoding name |
| `warning` | `number` | Yes | — | Token count that triggers a warning |
| `error` | `number` | Yes | — | Token count that triggers an error |

The model name key (e.g. `"opus-4.5"`) is a free-form label used in output.

## `overrides[]`

Each override entry:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `files` | `string[]` | Yes | Exact file paths this override applies to |
| `rules` | `object` | Yes | Rule overrides (same structure as top-level `rules`) |

## `overrides[].rules.token-limit.models.<name>`

Override model entries — all fields are optional:

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `encoding` | `string` | No | Inherited | Override the tokenizer encoding |
| `warning` | `number` | No | Inherited | Override the warning threshold |
| `error` | `number` | No | Inherited | Override the error threshold |

Only specified fields are overridden; unspecified fields inherit from the global config.

## Minimal Example

```json
{
  "patterns": ["./skills/**/*.md"],
  "rules": {
    "token-limit": {
      "models": {
        "opus-4.5": {
          "encoding": "cl100k_base",
          "warning": 8000,
          "error": 12000
        }
      }
    }
  }
}
```

## Full Example

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
  },
  "overrides": [
    {
      "files": [".github/skills/large-skill/SKILL.md"],
      "rules": {
        "token-limit": {
          "models": {
            "opus-4.5": { "warning": 16000, "error": 24000 },
            "gpt-4o": { "warning": 16000, "error": 24000 }
          }
        }
      }
    }
  ]
}
```
