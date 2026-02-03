# Configuration

skills-lint is configured via a JSON file, by default `.skills-lint.config.json` in your project root.

## Full Annotated Example

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
            "opus-4.5": { "warning": 16000, "error": 24000 }
          }
        }
      }
    }
  ]
}
```

## Patterns

The `patterns` array contains glob patterns used to discover skill files:

```json
{
  "patterns": [
    "./.github/**/SKILL.md",
    "./docs/skills/**/*.md"
  ]
}
```

- Standard glob syntax (`*`, `**`, `?`)
- Paths are relative to the config file location
- Multiple patterns are supported; all matched files are linted

## Models

Each entry in `rules.token-limit.models` maps a model name to its budget:

```json
{
  "encoding": "cl100k_base",
  "warning": 8000,
  "error": 12000
}
```

| Field | Type | Description |
|-------|------|-------------|
| `encoding` | `string` | Tokenizer encoding name (see [Encodings](/reference/encodings)) |
| `warning` | `number` | Token count threshold that triggers a warning |
| `error` | `number` | Token count threshold that triggers an error |

The model name is a free-form label (e.g. `"opus-4.5"`, `"gpt-4o"`, `"my-custom-model"`). It appears in the output table for identification.

## Encodings

Supported tokenizer encodings:

| Encoding | Common Models |
|----------|--------------|
| `cl100k_base` | Claude (Opus, Sonnet, Haiku), GPT-4 |
| `o200k_base` | GPT-4o, GPT-4o-mini |
| `p50k_base` | GPT-3, Codex |
| `r50k_base` | GPT-2 |

If `encoding` is omitted, it defaults to `cl100k_base`.

See [Encodings Reference](/reference/encodings) for details.

## Overrides

Apply different thresholds to specific files without changing global limits:

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

### Override behavior

- `files` — exact file paths (not globs) that this override applies to
- Only the fields you specify are overridden; others inherit from the global config
- You can override `encoding`, `warning`, `error`, or any combination
- Multiple overrides can target the same file; they are applied in order
- An override only applies to models listed in it — unlisted models use global values

### Example: grant a larger budget for one file

```json
{
  "overrides": [
    {
      "files": [".github/skills/complex-workflow/SKILL.md"],
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
