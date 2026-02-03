# Getting Started

## What is skills-lint?

**skills-lint** is a token budget linter for agent skill files. It counts tokens per model encoding and reports warnings or errors when skill files exceed configurable thresholds.

If you maintain agent skill files (e.g. `SKILL.md` files for Claude Code, Copilot, or other AI coding agents), skills-lint helps you ensure those files stay within the token context budgets of the models that consume them.

## Installation

Install globally via npm:

```sh
npm install -g @haasstefan/skills-lint
```

Pre-built binaries are available for macOS (ARM64, x64), Linux (x64, ARM64), and Windows (x64, ARM64). The correct binary is selected automatically via npm optional dependencies.

## Quick Start

### 1. Create a config file

Add a `.skills-lint.config.json` to your project root:

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
        "gpt-4o": {
          "encoding": "o200k_base",
          "warning": 8000,
          "error": 12000
        }
      }
    }
  }
}
```

### 2. Run the linter

```sh
skills-lint
```

The output shows a table with token counts, thresholds, and pass/warn/error status for each file × model combination.

### 3. Interpret the results

| Exit Code | Meaning |
|-----------|---------|
| `0` | All files pass |
| `1` | At least one error |
| `2` | Warnings only (no errors) |
| `3` | Runtime error (bad config, file not found, etc.) |

## Next Steps

- [Configuration](/guide/configuration) — full config reference with overrides
- [CI Integration](/guide/ci-integration) — run skills-lint in GitHub Actions
- [CLI Reference](/reference/cli) — all flags and options
