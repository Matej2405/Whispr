# Whispr - Diskretan AI Asistent 🎤

**AI assistant that listens when you can't** - A hackathon MVP that combines speech recognition, OCR, AI response generation, and blockchain logging.

## Features

✅ **Audio Input & Transcription (ASR)** - Whisper-powered speech-to-text  
✅ **Screen Capture & OCR** - Tesseract text extraction  
✅ **AI Response Generation** - Rules-based + API fallback (Ollama/OpenAI)  
✅ **Blockchain Logging** - Solana devnet memo transactions  
✅ **Beautiful CLI** - Colored output with progress indicators  
✅ **Global Hotkey** - Ctrl+Shift+W to trigger capture  

## Quick Demo

```bash
cargo run --release -- --demo
```

Press Ctrl+Shift+W in background mode:
```bash
cargo run --release -- --listen
```

## Installation

### Prerequisites

1. **Rust** (1.83+)
```bash
rustup update stable
```

2. **Node.js** (for Solana logging)
```bash
npm install
```

3. **Tesseract OCR**
- Windows: Download from [GitHub](https://github.com/UB-Mannheim/tesseract/wiki)
- macOS: `brew install tesseract`
- Linux: `apt install tesseract-ocr`

4. **Whisper Model**
```bash
mkdir models
curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
```

### Build

```bash
cargo build --release
```

## Usage

### Demo Mode (Recommended)
```bash
cargo run --release -- --demo --language eng --tesseract "path/to/tesseract.exe"
```

### Hotkey Listener Mode
```bash
cargo run --release -- --listen
# Press Ctrl+Shift+W to trigger
```

### Individual Components
```bash
# ASR only
cargo run --release -- --duration 5

# OCR only
cargo run --release -- --ocr

# Combined pipeline
cargo run --release -- --combined --solana-log
```

## CLI Options

```
--demo              Full pipeline with styled output
--listen            Background hotkey listener (Ctrl+Shift+W)
--combined          Run pipeline (ASR + OCR + Response + Solana)
--ocr               Screenshot and OCR only
--no-chain          Skip blockchain logging
--solana-log        Enable Solana in combined mode

-d, --duration      Recording duration in seconds [default: 5]
-l, --language      Language code [default: en]
-m, --model         Whisper model path [default: ./models/ggml-base.en.bin]
--tesseract         Tesseract executable path
--ollama-model      Use local Ollama (e.g., mistral)
--openai-key        OpenAI API key
```

## Documentation

- **[DEMO.md](./DEMO.md)** - Complete demo guide with examples
- **[TESTING.md](./TESTING.md)** - Testing instructions
- **[PACKAGING.md](./PACKAGING.md)** - Distribution and packaging
- **[GITHUB_SETUP.md](../GITHUB_SETUP.md)** - Git repository setup

## Architecture

```
┌──────────────────────────────────────────────────────┐
│ Input Layer                                          │
│  ┌──────────────┐           ┌──────────────┐       │
│  │ Microphone   │           │ Screen       │       │
│  │ (cpal)       │           │ (screenshots)│       │
│  └──────────────┘           └──────────────┘       │
│        │                            │                │
│        ▼                            ▼                │
│  ┌──────────────┐           ┌──────────────┐       │
│  │ Whisper ASR  │           │ Tesseract    │       │
│  │ (whisper-rs) │           │ OCR          │       │
│  └──────────────┘           └──────────────┘       │
│        │                            │                │
│        └────────────┬───────────────┘                │
│                     ▼                                 │
│            ┌──────────────────┐                     │
│            │ Response Gen     │                     │
│            │ Rules/Ollama/GPT │                     │
│            └──────────────────┘                     │
│                     │                                 │
│                     ▼                                 │
│            ┌──────────────────┐                     │
│            │ Solana Devnet    │                     │
│            │ Memo Logger      │                     │
│            └──────────────────┘                     │
└──────────────────────────────────────────────────────┘
```

## Technology Stack

- **Rust** - Core application
- **whisper-rs** - Speech-to-text
- **cpal** - Audio capture
- **screenshots** - Screen capture
- **Tesseract** - OCR engine
- **reqwest** - HTTP client (API calls)
- **Node.js + @solana/web3.js** - Blockchain logging
- **colored + indicatif** - Beautiful terminal UI
- **rdev** - Global hotkey listener
- **clap** - CLI parsing

## Hackathon Checklist

- [x] Audio Input & Transcription
- [x] Screen Capture & OCR
- [x] Local Response Generator
- [x] Solana Devnet Logging
- [x] CLI Demo Interface
- [x] Global Hotkey Support
- [x] Comprehensive Documentation

## License

MIT

## Credits

Built for the Colosseum Hackathon 2025


