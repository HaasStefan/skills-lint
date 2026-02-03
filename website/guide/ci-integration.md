# CI Integration

## GitHub Actions

```yaml
name: Lint Skills

on:
  push:
    branches: [master]
    paths: ['.github/skills/**', '.skills-lint.config.json']
  pull_request:
    paths: ['.github/skills/**', '.skills-lint.config.json']

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
      - run: npm install -g @haasstefan/skills-lint
      - run: skills-lint --quiet
```

## Exit Codes

| Code | Meaning | CI behavior |
|------|---------|-------------|
| `0` | Pass | Success |
| `1` | Error | Fail |
| `2` | Warning only | Up to you |
| `3` | Runtime error | Fail |

## `--quiet`

Suppresses the ASCII banner. Table output and summary still print.

## Fail on warnings

```yaml
- name: Lint (strict)
  run: |
    skills-lint --quiet
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
      exit 1
    fi
```

## Custom config path

```sh
skills-lint --config path/to/config.json --quiet
```

## Single file

```sh
skills-lint --file .github/skills/my-skill/SKILL.md --quiet
```
