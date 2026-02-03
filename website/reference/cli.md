# CLI

```sh
skills-lint [OPTIONS]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--file <path>` | — | Lint a single file (skips aggregate rules) |
| `--config <path>` | `.skills-lint.config.json` | Config file path |
| `--quiet` | `false` | Hide the ASCII banner |
| `--verbose` | `false` | Show all findings including passing rules |
| `--no-cache` | `false` | Disable token-count caching (see [Caching](/guide/caching)) |
| `--help` | — | Print help |
| `--version` | — | Print version |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | All pass |
| `1` | At least one error |
| `2` | Warnings only |
| `3` | Runtime error |

## Output

Each file section shows its applicable rules as a tree. Rules are displayed in this order:

1. **Inline rules** — `skill-structure`, `unique-name`, `unique-description` — shown as single-line findings
2. **Sub-table rules** — `frontmatter-limit`, `token-limit` — shown as tables with columns: **Model**, **Tokens**, **Warning**, **Error**, **Status**

Status is `✓ PASS`, `⚠ WARN`, or `✗ ERROR`. In non-verbose mode, only rules with warnings or errors are shown. Use `--verbose` to see all rules including passing ones.

When `skill-index-budget` is configured, an additional `(skill index)` section appears with the aggregate frontmatter token count. This is not included in the file count.

A summary line follows with total passed, warnings, and errors across all files.

## Examples

```sh
# Default config
skills-lint

# Single file
skills-lint --file .github/skills/code-review/SKILL.md

# CI mode
skills-lint --quiet

# Custom config
skills-lint --config configs/skills-lint.json
```
