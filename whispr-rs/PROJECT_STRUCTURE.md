# Whispr Project Structure 📂

This document explains the organization of the Whispr codebase to help developers understand and contribute.

---

## 📁 Directory Layout

```
whispr-rs/
├── src/
│   ├── main.rs          # CLI entry point and orchestration
│   ├── audio.rs         # Audio capture and Whisper transcription
│   ├── ocr.rs           # Screenshot capture and Tesseract OCR
│   ├── ai.rs            # Gemini API integration and response generation
│   ├── ui.rs            # Overlay window and hotkey listener
│   ├── blockchain.rs    # Solana devnet logging
│   └── utils.rs         # Utility functions (HTML escaping, etc.)
│
├── models/              # Whisper model files (not in git)
│   └── ggml-base.en.bin
│
├── node_modules/        # Node.js dependencies (not in git)
├── out/                 # Screenshot output directory
│
├── postMemo.js          # Solana memo transaction script
├── package.json         # Node.js dependencies
├── Cargo.toml           # Rust dependencies
│
├── README.md            # Main project documentation
├── QUICKSTART.md        # 5-minute setup guide
├── DEMO.md              # Demo walkthrough
├── TESTING.md           # Testing instructions
├── GEMINI_SETUP.md      # Gemini API setup
├── GITHUB_SETUP.md      # Git repository setup
├── HACKATHON_SUMMARY.md # Hackathon deliverables
└── PROJECT_STRUCTURE.md # This file
```

---

## 🧩 Module Breakdown

### `main.rs` - Entry Point (350 lines)

**Purpose:** CLI argument parsing and orchestration of the entire pipeline.

**Key Functions:**
- `main()` - Entry point that handles different modes (`--demo`, `--listen`, `--combined`, etc.)
- `run_demo_mode()` - Executes the full pipeline with styled output
- `run_combined_mode()` - Runs ASR + OCR + AI + Blockchain without styling

**Dependencies:**
- `clap` - CLI argument parsing
- `colored` - Terminal colors
- `indicatif` - Progress indicators

**Example Usage:**
```rust
// CLI args are parsed into Args struct
let args = Args::parse();

// Different execution paths based on flags
if args.listen {
    ui::run_hotkey_listener(&args)?;
} else if args.demo {
    run_demo_mode(&args)?;
}
```

---

### `audio.rs` - Audio Processing (205 lines)

**Purpose:** Capture microphone input and transcribe speech using Whisper.

**Key Functions:**
- `record_audio(duration_secs: u32, output_path: &Path)` - Records audio to WAV file
- `downmix_to_mono_f32(samples: &[i16], channels: usize)` - Converts stereo to mono
- `linear_resample(input: &[f32], src_rate: f32, dst_rate: f32)` - Resamples to 16kHz for Whisper

**Dependencies:**
- `cpal` - Cross-platform audio I/O
- `whisper-rs` - Whisper.cpp Rust bindings
- `hound` - WAV file encoding

**Data Flow:**
```
Microphone (cpal) 
  → Samples (i16) 
  → Downmix to mono 
  → Resample to 16kHz 
  → Whisper model 
  → Transcribed text
```

**Example Usage:**
```rust
// Record 5 seconds of audio
let audio_path = Path::new("out/whispr_audio.wav");
record_audio(5, audio_path)?;

// Load Whisper model
let ctx = WhisperContext::new_with_params(model_path, params)?;
let mut state = ctx.create_state()?;

// Transcribe
state.full(FullParams::new(whisper_rs::SamplingStrategy::Greedy), &samples)?;
```

---

### `ocr.rs` - Screen Capture & OCR (104 lines)

**Purpose:** Capture screenshots and extract text using Tesseract.

**Key Functions:**
- `run_ocr(language: &str, tesseract_path: Option<&str>)` - Full OCR pipeline
- `find_tesseract()` - Auto-detect Tesseract installation

**Dependencies:**
- `screenshots` - Multi-monitor screenshot capture
- `image` - Image processing and encoding
- Tesseract (external CLI tool)

**Data Flow:**
```
Primary screen 
  → Screenshot (RGBA) 
  → Convert to PNG 
  → Save to out/screenshot.png 
  → Tesseract CLI 
  → Extracted text
```

**Example Usage:**
```rust
// Capture screenshot and run OCR
let (ocr_text, screenshot_path) = ocr::run_ocr("eng", None)?;
println!("Extracted text: {}", ocr_text);
```

---

### `ai.rs` - AI Response Generation (119 lines)

**Purpose:** Send audio transcription and screenshot to Gemini Vision API.

**Key Functions:**
- `generate_response(transcript: &str, ocr_text: &str, screenshot_path: &Path, gemini_key: Option<&str>)` - Main response generator
- `call_gemini(transcript: &str, ocr_text: &str, screenshot_path: &Path, api_key: &str)` - Gemini Vision API call
- `truncate(s: &str, max_len: usize)` - Limit text length

**Dependencies:**
- `reqwest` - HTTP client
- `serde` + `serde_json` - JSON serialization
- `base64` - Image encoding

**Data Flow:**
```
Transcript + OCR text + Screenshot 
  → Base64 encode image 
  → Build JSON request 
  → POST to Gemini Vision API 
  → Parse response 
  → Extract conversational text
```

**Prompt Engineering:**
```rust
// System prompt encourages natural responses
"You are a helpful AI assistant. Respond naturally like a friend..."
```

**Example Usage:**
```rust
let response = ai::generate_response(
    &transcript, 
    &ocr_text, 
    &screenshot_path, 
    Some("YOUR_GEMINI_API_KEY")
)?;
println!("AI: {}", response);
```

---

### `ui.rs` - User Interface (273 lines)

**Purpose:** Overlay window for displaying responses and global hotkey listener.

**Key Functions:**
- `show_overlay(content: &str)` - Display chat overlay window
- `run_hotkey_listener(args: &Args)` - Listen for Ctrl+Shift+W

**Dependencies:**
- `tao` - Window management
- `wry` - WebView rendering
- `rdev` - Global keyboard hooks

**Overlay Design:**
```html
<!-- Cluely-inspired gradient background -->
<div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
  <h2>Whispr AI</h2>
  <div class="response-box">
    <!-- AI response here -->
  </div>
</div>
```

**Example Usage:**
```rust
// Show response in overlay
ui::show_overlay(&response)?;

// Or run background listener
ui::run_hotkey_listener(&args)?; // Ctrl+Shift+W
```

---

### `blockchain.rs` - Solana Logging (41 lines)

**Purpose:** Log interaction hashes to Solana devnet for immutability.

**Key Functions:**
- `log_to_solana(summary: &str)` - Call Node.js script to post memo transaction

**Dependencies:**
- Node.js + `@solana/web3.js` (via `postMemo.js`)

**Data Flow:**
```
Summary text 
  → SHA-256 hash 
  → Solana memo transaction 
  → Devnet confirmation 
  → Explorer URL
```

**Example Usage:**
```rust
// Log to blockchain
if !args.no_chain {
    blockchain::log_to_solana(&response)?;
}
```

---

### `utils.rs` - Utilities (9 lines)

**Purpose:** Helper functions used across modules.

**Key Functions:**
- `html_escape(s: &str)` - Escape HTML special characters for overlay

**Example Usage:**
```rust
let safe_html = utils::html_escape(&user_input);
```

---

## 🔄 Data Flow (Full Pipeline)

```
1. User triggers capture (--demo or Ctrl+Shift+W)
   ↓
2. audio::record_audio(5) → "Hello world" (transcript)
   ↓
3. ocr::run_ocr("eng") → "Code on screen" (OCR text) + screenshot.png
   ↓
4. ai::generate_response(transcript, ocr_text, screenshot_path, api_key)
   ├─ Base64 encode screenshot
   ├─ POST to Gemini Vision API
   └─ "Based on what you said and what's on screen..."
   ↓
5. ui::show_overlay(response) → Display in floating window
   ↓
6. blockchain::log_to_solana(response) → SHA256 hash → Solana memo tx
   ↓
7. Explorer URL printed to console
```

---

## 🛠️ Build Process

### Dependencies (Cargo.toml)

```toml
[dependencies]
# Audio
cpal = "0.15"
whisper-rs = "0.12"
hound = "3.5"

# OCR
screenshots = "0.8"
image = { version = "0.24", features = ["png", "jpeg"] }

# AI
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
base64 = "0.22"

# UI
tao = "0.16"
wry = "0.24"
rdev = "0.5"

# CLI
clap = { version = "4", features = ["derive"] }
colored = "2"
indicatif = "0.17"
```

### Build Steps

1. **Install LLVM/Clang** (Windows only) - Required for whisper-rs compilation
2. **Install Tesseract** - External dependency for OCR
3. **Download Whisper model** - `ggml-base.en.bin` (~140MB)
4. **Run `cargo build --release`** - Compiles all Rust code
5. **Run `npm install`** - Installs Solana dependencies

---

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Component Tests
```bash
# ASR only
cargo run --release -- --duration 5

# OCR only
cargo run --release -- --ocr

# Combined
cargo run --release -- --combined
```

### Integration Test
```bash
# Full pipeline
cargo run --release -- --demo
```

---

## 🔐 Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `GEMINI_API_KEY` | Gemini API key from AI Studio | Yes |
| `LIBCLANG_PATH` | Path to LLVM bin (Windows only) | Build-time |
| `TESSERACT_PATH` | Custom Tesseract path | Optional |
| `TESSDATA_PREFIX` | Tesseract data directory | Auto-detected |

---

## 📦 Binary Size

After `cargo build --release`:
- **Windows:** ~150MB (includes whisper-rs + dependencies)
- **macOS:** ~120MB
- **Linux:** ~110MB

Large size due to:
- Whisper.cpp static library (~80MB)
- WebView runtime (wry)
- Audio libraries (cpal)

---

## 🚀 Performance

**Typical latency breakdown:**
- Audio recording: 5s (user-configurable)
- Whisper transcription: 1-3s (CPU-dependent)
- Screenshot + OCR: 0.5-1s
- Gemini API call: 2-5s (network-dependent)
- Solana transaction: 3-5s (devnet)

**Total:** ~12-19 seconds for full pipeline

**Optimization opportunities:**
- Parallel execution of OCR and Whisper
- Smaller Whisper model (tiny vs base)
- Local AI fallback (Ollama)

---

## 🤝 Contributing

To add a new feature:
1. Create a new module in `src/` if it's a major feature
2. Add necessary dependencies to `Cargo.toml`
3. Update `main.rs` to integrate the feature
4. Update documentation (README.md, TESTING.md)
5. Test all modes (`--demo`, `--listen`, `--combined`)

**Code style:**
- Use `anyhow::Result` for error handling
- Add `colored` output for user-facing messages
- Follow Rust naming conventions (snake_case)

---

## 📝 License

MIT License - See LICENSE file for details.

---

**Questions?** Check the [main README](./README.md) or open an issue!

