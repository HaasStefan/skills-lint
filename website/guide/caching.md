# Caching

Token counting is the most expensive operation in skills-lint. Caching stores the results in `.skills-lint-cache/tokens.json` so subsequent runs skip redundant work when files haven't changed.

## How It Works

Each token count is keyed by a SHA-256 hash of the file content and the encoding name. On the first run, skills-lint writes the cache file. On later runs, unchanged files get an instant cache hit and skip tokenization entirely.

The cache file lives at `.skills-lint-cache/tokens.json` relative to where you run the command:

```
.skills-lint-cache/
  tokens.json
```

::: tip
Running `skills-lint init` will add `.skills-lint-cache/` to your `.gitignore` automatically if the file exists.
:::

## Configuration

Caching is **on by default**. To disable it in the config file:

```json
{
  "patterns": ["./.github/**/SKILL.md"],
  "cache": false,
  "rules": {
    "token-limit": {
      "models": {
        "gpt-4o": { "warning": 8000, "error": 16000 }
      }
    }
  }
}
```

You can also disable caching for a single run with the `--no-cache` flag:

```sh
skills-lint --no-cache
```

The flag takes precedence — `--no-cache` disables caching even if `"cache": true` is set in the config.

## Clearing the Cache

Delete the cache directory to force a full recount:

```sh
rm -rf .skills-lint-cache
```

The next run will recreate it.
