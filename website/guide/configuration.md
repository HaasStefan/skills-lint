# Configuration

Config file: `.skills-lint.config.json` in your project root.

## Example

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 12000 },
        "gpt-4o-mini": { "warning": 8000, "error": 12000 },
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

| Model | Default Encoding |
|-------|-----------------|
| `gpt-5` | `o200k_base` |
| `gpt-4o` | `o200k_base` |
| `gpt-4o-mini` | `o200k_base` |
| `gpt-4-turbo` | `cl100k_base` |
| `gpt-4` | `cl100k_base` |
| `gpt-3.5-turbo` | `cl100k_base` |

Unsupported model names are rejected at config load.

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
            "gpt-4o": { "warning": 16000, "error": 24000 }
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
