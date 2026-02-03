# CI Integration

skills-lint is designed to work in CI pipelines. It uses distinct exit codes to differentiate between pass, warning, and error states.

## GitHub Actions

Add skills-lint to your workflow:

```yaml
name: Lint Skills

on:
  push:
    branches: [master]
    paths:
      - '.github/skills/**'
      - '.skills-lint.config.json'
  pull_request:
    paths:
      - '.github/skills/**'
      - '.skills-lint.config.json'

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install skills-lint
        run: npm install -g @haasstefan/skills-lint

      - name: Run skills-lint
        run: skills-lint --quiet
```

## Exit Codes

| Code | Meaning | Suggested CI behavior |
|------|---------|----------------------|
| `0` | All files pass | Success |
| `1` | At least one error | Fail the build |
| `2` | Warnings only | Pass or fail depending on policy |
| `3` | Runtime error | Fail the build |

## The `--quiet` Flag

Use `--quiet` to suppress the ASCII banner. This keeps CI logs clean:

```sh
skills-lint --quiet
```

The table output and summary are still printed — only the decorative banner is hidden.

## Treating Warnings as Errors

By default, exit code `2` (warnings only) does not fail a standard CI step. To fail on warnings:

```yaml
- name: Run skills-lint (strict)
  run: |
    skills-lint --quiet
    exit_code=$?
    if [ $exit_code -ne 0 ]; then
      echo "skills-lint failed with exit code $exit_code"
      exit 1
    fi
```

## Custom Config Path

If your config file is in a non-default location:

```sh
skills-lint --config path/to/config.json --quiet
```

## Single File Mode

Lint a single file (useful for pre-commit hooks):

```sh
skills-lint --file .github/skills/my-skill/SKILL.md --quiet
```
