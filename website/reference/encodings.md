# Encodings

Token counting uses [tiktoken](https://github.com/openai/tiktoken). Encoding is auto-selected from the model name.

## Model → Encoding

| Model | Encoding |
|-------|----------|
| `gpt-5` | `o200k_base` |
| `gpt-4o` | `o200k_base` |
| `gpt-4o-mini` | `o200k_base` |
| `gpt-4-turbo` | `cl100k_base` |
| `gpt-4` | `cl100k_base` |
| `gpt-3.5-turbo` | `cl100k_base` |

## Override

To use a non-default encoding:

```json
{
  "gpt-4o": {
    "encoding": "cl100k_base",
    "warning": 8000,
    "error": 12000
  }
}
```

Valid values: `cl100k_base`, `o200k_base`, `p50k_base`, `r50k_base`.

## Links

- [tiktoken](https://github.com/openai/tiktoken)
- [OpenAI Tokenizer](https://platform.openai.com/tokenizer)
