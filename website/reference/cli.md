# CLI

```sh
skills-lint [OPTIONS]
```

## Flags

| Flag | Default | Description |
|------|---------|-------------|
| `--file <path>` | — | Lint a single file |
| `--config <path>` | `.skills-lint.config.json` | Config file path |
| `--quiet` | `false` | Hide the ASCII banner |
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

Table columns: **File**, **Model**, **Tokens**, **Warning**, **Error**, **Status**.

Repeated files show `┆`. Status is `✓ PASS`, `⚠ WARN`, or `✗ ERROR`. Summary line follows the table.

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
