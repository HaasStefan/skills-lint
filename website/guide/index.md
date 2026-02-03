# Getting Started

## What is skills-lint?

Token budget linter for agent skill files. Counts tokens using [tiktoken](https://github.com/openai/tiktoken) and reports when files exceed per-model thresholds.

## Install

```sh
npm install -g @haasstefan/skills-lint
```

Pre-built binaries for macOS, Linux, and Windows (ARM64 + x64).

## Quick Start

**1. Add `.skills-lint.config.json` to your project root:**

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
  }
}
```

Encoding is selected automatically from the model name.

**2. Run:**

```sh
skills-lint
```

**3. Exit codes:**

| Code | Meaning |
|------|---------|
| `0` | All pass |
| `1` | Error (at least one file over error threshold) |
| `2` | Warning only |
| `3` | Runtime error |

## Next

- [Configuration](/guide/configuration) — models, overrides, patterns
- [CI Integration](/guide/ci-integration) — GitHub Actions setup
- [CLI Reference](/reference/cli) — flags and output format
