# Rules

skills-lint ships with six built-in rules. Each rule is optional and can be enabled independently in `.skills-lint.config.json`.

| Rule | Scope | Type | Default |
|------|-------|------|---------|
| [`token-limit`](#token-limit) | Per file | Token budget | **Required** |
| [`frontmatter-limit`](#frontmatter-limit) | Per file | Token budget | Off |
| [`skill-index-budget`](#skill-index-budget) | Aggregate | Token budget | Off |
| [`skill-structure`](#skill-structure) | Per file | Structural | Off |
| [`unique-name`](#unique-name) | Aggregate | Structural | Off |
| [`unique-description`](#unique-description) | Aggregate | Structural | Off |

**Per file** rules run on each discovered SKILL.md independently (and in `--file` single-file mode).
**Aggregate** rules need all files and are skipped when using `--file`.

## token-limit

Counts tokens in the **entire file** and checks against per-model thresholds.

```json
{
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 16000 },
        "gpt-4": { "warning": 2000, "error": 4000 }
      }
    }
  }
}
```

This is the only required rule. Each model entry needs `warning` and `error` thresholds. An optional `encoding` field overrides the auto-detected encoding (see [Encodings](/reference/encodings)).

Per-file overrides can raise or lower thresholds for specific files — see [Overrides](/guide/configuration#overrides).

## frontmatter-limit

Counts tokens in just the **YAML frontmatter** of each file and checks against per-model thresholds. Useful for keeping skill metadata concise since frontmatter is loaded into context at startup.

```json
{
  "rules": {
    "frontmatter-limit": {
      "models": {
        "gpt-4o": { "warning": 1000, "error": 2000 },
        "gpt-4": { "warning": 500, "error": 1000 }
      }
    }
  }
}
```

Same `models` schema as `token-limit`. Files without frontmatter are skipped. Omit the key to disable.

## skill-index-budget

Aggregates the YAML frontmatter from **all** discovered SKILL.md files, concatenates it, and checks the combined token count per model.

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

In the output, the aggregate result appears under a `(skill index)` heading. It is not included in the per-file count.

::: tip
This rule only runs in aggregate mode. It is skipped when using `--file`.
:::

## skill-structure

Validates the structure of each SKILL.md file:

1. Has YAML frontmatter (`---` delimiters)
2. Frontmatter contains `name` with a non-empty value
3. Frontmatter contains `description` with a non-empty value
4. Body after frontmatter is non-empty

```json
{
  "rules": {
    "skill-structure": true
  }
}
```

Each file produces a single inline finding: `PASS` with message "valid", or `ERROR` with a comma-separated list of issues (e.g. "missing name, empty body").

## unique-name

Checks that no two skill files share the same `name` in their frontmatter.

```json
{
  "rules": {
    "unique-name": true
  }
}
```

Each file with a name gets an inline finding: `PASS` with message "unique", or `ERROR` listing the conflicting files (e.g. `duplicate name "foo" (also in .github/skills/other/SKILL.md)`).

Files without a `name` field are skipped by this rule (but would be caught by `skill-structure`).

::: tip
This rule only runs in aggregate mode. It is skipped when using `--file`.
:::

## unique-description

Checks that no two skill files share the same `description` in their frontmatter.

```json
{
  "rules": {
    "unique-description": true
  }
}
```

Same behavior as `unique-name` but for the `description` field. Long descriptions are truncated in the error message for readability.

::: tip
This rule only runs in aggregate mode. It is skipped when using `--file`.
:::

## Full example

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
  }
}
```
