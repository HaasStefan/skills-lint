# Encodings

skills-lint uses [tiktoken](https://github.com/openai/tiktoken) tokenizer encodings to count tokens. This page lists all supported encodings and which models commonly use them.

## Supported Encodings

| Encoding | Description | Common Models |
|----------|-------------|---------------|
| `cl100k_base` | 100k vocab, used by most modern models | Claude (Opus, Sonnet, Haiku), GPT-4, GPT-4 Turbo, GPT-3.5 Turbo |
| `o200k_base` | 200k vocab, optimized for newer OpenAI models | GPT-4o, GPT-4o-mini |
| `p50k_base` | 50k vocab, legacy | GPT-3 (davinci, curie, babbage, ada), Codex |
| `r50k_base` | 50k vocab, oldest | GPT-2 |

## Default Encoding

If the `encoding` field is omitted in a model configuration, skills-lint defaults to `cl100k_base`.

```json
{
  "opus-4.5": {
    "warning": 8000,
    "error": 12000
  }
}
```

This is equivalent to:

```json
{
  "opus-4.5": {
    "encoding": "cl100k_base",
    "warning": 8000,
    "error": 12000
  }
}
```

## Choosing an Encoding

Pick the encoding that matches the model consuming your skill files:

- **Claude models** (Opus, Sonnet, Haiku) — use `cl100k_base`. While Claude uses its own tokenizer internally, `cl100k_base` provides a close approximation for budget planning.
- **GPT-4o / GPT-4o-mini** — use `o200k_base`.
- **GPT-4 / GPT-3.5 Turbo** — use `cl100k_base`.
- **Legacy GPT-3** — use `p50k_base`.

## Token Count Differences

Different encodings produce different token counts for the same text. A file that is 8,000 tokens with `cl100k_base` might be 7,500 tokens with `o200k_base`, or 9,200 tokens with `p50k_base`.

This is why skills-lint supports multiple models in a single config — you can lint the same files against different encodings simultaneously.

## Further Reading

- [tiktoken](https://github.com/openai/tiktoken) — OpenAI's tokenizer library
- [OpenAI Tokenizer Tool](https://platform.openai.com/tokenizer) — interactive token counter
