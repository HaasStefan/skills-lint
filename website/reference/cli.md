# CLI Reference

## Usage

```sh
skills-lint [OPTIONS]
```

Run in a directory containing a `.skills-lint.config.json` (or specify a config path with `--config`).

## Flags

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--file <path>` | `string` | — | Lint a single file instead of using config patterns |
| `--config <path>` | `string` | `.skills-lint.config.json` | Path to the config file |
| `--quiet` | `bool` | `false` | Suppress the ASCII banner (useful for CI) |
| `--help` | — | — | Print help information |
| `--version` | — | — | Print version |

## Exit Codes

| Code | Constant | Description |
|------|----------|-------------|
| `0` | Pass | All files are within token budgets |
| `1` | Error | At least one file exceeds an error threshold |
| `2` | Warning | At least one file exceeds a warning threshold (no errors) |
| `3` | Runtime Error | Configuration error, file not found, or other runtime failure |

## Output Format

skills-lint prints a table with the following columns:

| Column | Description |
|--------|-------------|
| **File** | Path to the skill file. Repeated entries for the same file show `┆` |
| **Model** | The model name from config |
| **Tokens** | Actual token count for this file × model pair |
| **Warning** | Warning threshold from config |
| **Error** | Error threshold from config |
| **Status** | `✓ PASS` (green), `⚠ WARN` (yellow), or `✗ ERROR` (red) |

After the table, a summary line shows counts of passed, warnings, and errors across all files.

## Examples

### Lint with default config

```sh
skills-lint
```

### Lint a single file

```sh
skills-lint --file .github/skills/code-review/SKILL.md
```

### CI mode (no banner)

```sh
skills-lint --quiet
```

### Custom config location

```sh
skills-lint --config configs/skills-lint.json
```
