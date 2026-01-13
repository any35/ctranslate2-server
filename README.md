# CTranslate2 Server

A high-performance, OpenAI-compatible HTTP server for [CTranslate2](https://github.com/OpenNMT/CTranslate2) models, built with Rust and Axum.

## Features

- **OpenAI Compatible:** Implements `/v1/chat/completions` for text generation.
- **High Performance:** Powered by CTranslate2 (C++) with efficient Rust bindings (`ct2rs`).
- **Multi-Model Support:** Native support for **NLLB** and **T5** models.
- **Flexible Configuration:**
    - **Aliases:** Map friendly names (e.g., `nllb`) to specific model folders.
    - **Lazy Loading:** Models are loaded into memory only when first requested.
    - **Hardware Acceleration:** Support for **CPU** (MKL/DNNL) and **GPU** (CUDA).
- **Advanced Generation Control:**
    - Beam Size
    - Repetition Penalty
    - Target Language (for multilingual models like NLLB)

## Quick Start

### 1. Prepare Models
Download your CTranslate2-converted models into a `models/` directory.
Structure example:
```
models/
├── nllb-200-distilled-600M/
│   ├── model.bin
│   ├── sentencepiece.model
│   └── shared_vocabulary.txt
└── t5-small/
    ├── model.bin
    └── ...
```

### 2. Generate Configuration
Run the built-in config generator to scan your `models/` directory and create a `config.toml`.

**Using Docker:**
```bash
docker run --rm -v $(pwd):/app -w /app any35/ctranslate2-server:latest /app/server --config-gen
# Note: You might need to adjust permissions or run the generator locally if using the provided binary directly.
# Alternatively, copy the example config below.
```

**Using Cargo (Local):**
```bash
cargo run --bin config_generator
```

### 3. Run with Docker Compose

Create a `docker-compose.yml` (see [Docker Compose](#docker-compose) section) and run:

```bash
docker-compose up -d
```

### 4. Test API

**Generate Text (NLLB Translation):**
```bash
curl http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "nllb",
    "messages": [
      {"role": "user", "content": "Hello world"}
    ],
    "target_lang": "zho_Hans"
  }'
```

## Configuration (`config.toml`)

```toml
[server]
host = "0.0.0.0"
port = 8080

# Global Defaults
default_model = "nllb"
target_lang = "eng_Latn"
device = "cpu"          # "cpu" or "cuda"
device_indices = [0]    # GPU IDs
beam_size = 5
repetition_penalty = 1.2

[aliases]
"nllb" = "nllb-200-distilled-600M"

[models]
[models."nllb-200-distilled-600M"]
path = "./models/nllb-200-distilled-600M"
model_type = "nllb"
target_lang = "fra_Latn" # Per-model default
```

## API Reference

### POST `/v1/chat/completions`

**Parameters:**
- `model`: (string) Model alias or directory name.
- `messages`: (array) List of messages. Last user message is used as prompt.
- `target_lang`: (string, optional) Target language code (e.g., `fra_Latn`, `zho_Hans`). Overrides config.
- `beam_size`: (int, optional) Beam size for search (default: 5).
- `repetition_penalty`: (float, optional) Penalty for repeated tokens (default: 1.2).
- `no_repeat_ngram_size`: (int, optional) Prevent repeating n-grams of this size.

## Docker

### Build Locally
```bash
./scripts/build_docker.sh
```

### Run (CPU)
```bash
docker run -p 8080:8080 -v $(pwd)/models:/app/models -v $(pwd)/config.toml:/app/config.toml any35/ctranslate2-server:cpu
```

### Run (GPU)
Requires NVIDIA Container Toolkit.
```bash
docker run --gpus all -p 8080:8080 -v $(pwd)/models:/app/models -v $(pwd)/config.toml:/app/config.toml any35/ctranslate2-server:gpu
```

### Convert Models with Quantization

```bash
# for NLLB 3.3B, quantization with int8_float16
ct2-transformers-converter --model ./ --output_dir ./quant_16 --quantization int8_float16 

# for NLLB 1.3B, quantization with int8
ct2-transformers-converter --model ./ --output_dir ./quant_8 --quantization int8

```