# Whispr - MVP Demo Guide

**Diskretan AI asistent koji sluša kada ti ne možeš** 🎤

## Quick Demo

Run the full pipeline with a single command:

```bash
cargo run --release -- --demo --language eng --tesseract "path/to/tesseract.exe"
```

## Features Demonstrated

### 1. Audio Input & Transcription (ASR)
- Records microphone input (default 5 seconds, configurable)
- Uses Whisper base.en model for speech-to-text
- Supports multiple languages
- Automatic resampling to 16kHz and mono downmix

### 2. Screen Capture & OCR
- Captures full primary screen
- Uses Tesseract for text extraction
- Auto-detects tessdata directory
- Cleans and formats output

### 3. AI Response Generation
- Rules-based summarizer with keyword detection
- Optional API fallbacks:
  - Ollama (local LLM)
  - OpenAI GPT-3.5-turbo
- Context-aware suggestions

### 4. Blockchain Logging
- Posts SHA-256 hash memo to Solana devnet
- Auto-generates persistent keypair
- Requests airdrop if balance low
- Displays Solana Explorer link

## Demo Modes

### Standard Demo
```bash
cargo run --release -- --demo
```
Full pipeline with blockchain logging.

### Demo Without Blockchain
```bash
cargo run --release -- --demo --no-chain
```
Skip Solana logging for faster demos.

### Hotkey Listener Mode
```bash
cargo run --release -- --listen
```
Run in background, press **Ctrl+Shift+W** to trigger capture.

## Configuration Options

### Basic Options
- `--duration <secs>` - Recording duration (default: 5)
- `--language <lang>` - Language code (default: en)
- `--model <path>` - Whisper model path
- `--tesseract <path>` - Tesseract executable path

### AI Options
- `--ollama-model <model>` - Use local Ollama (e.g., mistral, llama2)
- `--openai-key <key>` - OpenAI API key (or set OPENAI_API_KEY env)

### Mode Flags
- `--demo` - Run full demo with styled output
- `--combined` - Run pipeline without styling
- `--ocr` - OCR only mode
- `--listen` - Background hotkey listener
- `--no-chain` - Skip blockchain logging
- `--solana-log` - Enable Solana in combined mode

## Example Commands

### Quick 3-second capture with Ollama
```bash
cargo run --release -- --demo --duration 3 --ollama-model mistral
```

### Background listener (Windows)
```bash
cargo run --release -- --listen --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

### macOS demo
```bash
cargo run --release -- --demo --language en --tesseract /usr/local/bin/tesseract
```

## Output Example

```
╔═══════════════════════════════════════════════════════════════╗
║          🎤 WHISPR - AI Assistant Demo Mode                ║
║  Diskretan AI asistent koji sluša kada ti ne možeš         ║
╚═══════════════════════════════════════════════════════════════╝

┌─ Step 1: Audio Input & Transcription (ASR)
  ✓ Audio captured
  ✓ Transcription complete
  → "This is a test of the Whisper transcription system"

┌─ Step 2: Screen Capture & OCR
  ✓ Screenshot captured & processed
  → "Meeting notes: Q4 review scheduled for Friday 3pm"

┌─ Step 3: AI Response Generation
  ✓ Response generated

┌─────────────────────────────────────────────────────────────┐
│ 📝 Summary:
│ You said: This is a test of the Whisper transcription system
│ 
│ On screen: Meeting notes: Q4 review scheduled for Friday 3pm
│ 
│ 💡 Tip: Looks like meeting/scheduling content. Consider reviewing calendar.
└─────────────────────────────────────────────────────────────┘

┌─ Step 4: Blockchain Logging (Solana Devnet)
  ✓ Transaction confirmed on-chain
  → Signature: 5pfQDEqwWSgVg7C1LTDrgDjqbtkNbHiaxHc8aQJgPUVY...
  → Memo: whispr:cf87eabdf7c3e156
  🔗 Explorer: https://explorer.solana.com/tx/...?cluster=devnet

═══════════════════════════════════════════════════════════════
  ✨ Demo Complete! All systems operational.
═══════════════════════════════════════════════════════════════
```

## Troubleshooting

### Tesseract Not Found
Set environment variables:
```bash
# Windows
setx TESSDATA_PREFIX "C:\Program Files\Tesseract-OCR\tessdata"

# macOS/Linux
export TESSDATA_PREFIX="/usr/local/share/tessdata"
```

### Whisper Model Missing
Download base.en model:
```bash
mkdir models
curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
```

### Hotkey Not Working (Windows)
Run as Administrator for global hotkey access.

### Solana Airdrop Failed
Rate limited. Wait 30 seconds or use `--no-chain` flag.

## Performance Tips

1. **Faster demos**: Use `--duration 2` or `--duration 3`
2. **Skip blockchain**: Add `--no-chain`
3. **Better accuracy**: Download larger Whisper model (small, medium)
4. **Local AI**: Use Ollama instead of OpenAI for offline operation

## Next Steps

- Add active window capture for cleaner OCR
- Implement clipboard auto-copy for responses
- Create system tray icon for always-on mode
- Package as standalone executable

