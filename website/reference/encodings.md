# Encodings

Token counting uses [tiktoken](https://github.com/openai/tiktoken). Encoding is auto-selected from the model name.

## Model → Encoding

| Model | Context Window | Max Input | Encoding |
|-------|---------------|-----------|----------|
| `gpt-5` | 400K | 272K | `o200k_base` |
| `gpt-4o` | 128K | 112K | `o200k_base` |
| `gpt-4o-mini` | 128K | 112K | `o200k_base` |
| `gpt-4-turbo` | 128K | 124K | `cl100k_base` |
| `gpt-4` | 8K | 4K | `cl100k_base` |
| `gpt-3.5-turbo` | 16K | 12K | `cl100k_base` |

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
