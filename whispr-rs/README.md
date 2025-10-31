# Whispr 🎤

**Diskretan AI asistent koji "sluša kada ti ne možeš"**

Whispr is a discrete AI assistant that captures audio, reads on-screen text, generates helpful responses, and logs interactions on Solana blockchain. Built for the Colosseum Hackathon 2025.

> **🚀 New to Whispr?** Check out the [Quick Start Guide](./QUICKSTART.md) to get running in 5 minutes!

## ✨ Features

- 🎙️ **Speech-to-Text** - Local Whisper model for privacy-first transcription
- 👁️ **Visual Understanding** - Screenshot capture + Gemini Vision API for context awareness
- 🧠 **AI Response Generation** - Gemini API with natural, conversational responses
- ⛓️ **Blockchain Logging** - Immutable interaction logs on Solana devnet
- ⌨️ **Global Hotkey** - Press Ctrl+Shift+W to trigger capture anywhere
- 💬 **Chat Overlay** - Cluely-inspired floating window for responses
- 🎨 **Beautiful CLI** - Colored output with progress indicators

---

## 🚀 Quick Start

> **💡 Tip:** Run `check_prerequisites.ps1` (Windows) or `check_prerequisites.sh` (Linux/macOS) to verify your system before starting!

### 1️⃣ Install Prerequisites

#### **Rust** (1.83+)
```bash
# Install/update Rust
rustup update stable
```

#### **Node.js** (for Solana)
```bash
# Navigate to project directory
cd whispr-rs
npm install
```

#### **Tesseract OCR**

**Windows:**
1. Download installer: https://github.com/UB-Mannheim/tesseract/wiki
2. Install to `C:\Program Files\Tesseract-OCR\` (or remember your path)
3. Add to PATH or note the installation path

**macOS:**
```bash
brew install tesseract
```

**Linux:**
```bash
sudo apt install tesseract-ocr
```

#### **LLVM/Clang** (Windows only - for whisper-rs compilation)
1. Download LLVM: https://github.com/llvm/llvm-project/releases
2. Install and add to PATH
3. Set environment variable:
```powershell
$env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"
```

#### **Whisper Model**
```bash
# Create models directory
mkdir models

# Download base English model (~140MB)
curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
```

### 2️⃣ Set Up Gemini API

1. Get your free API key: https://aistudio.google.com/app/apikey
2. Create a `.env` file in the `whispr-rs` folder:

```bash
cd whispr-rs
```

Create `.env` with this content:
```env
GEMINI_API_KEY=your-api-key-here
```

**Alternative:** Set as system environment variable:

**Windows (PowerShell - Permanent):**
```powershell
[System.Environment]::SetEnvironmentVariable('GEMINI_API_KEY', 'your-api-key-here', 'User')
```

**macOS/Linux:**
```bash
echo 'export GEMINI_API_KEY="your-api-key-here"' >> ~/.bashrc
source ~/.bashrc
```

### 3️⃣ Build the Project

```bash
cd whispr-rs
cargo build --release
```

⏳ This will take 5-10 minutes on first build (compiling whisper-rs and dependencies).

---

## 🎯 Usage

> **Note:** Make sure your `.env` file is set up with `GEMINI_API_KEY` before running!

### 🌟 Demo Mode (Recommended for First Run)

**Windows:**
```powershell
cd whispr-rs
.\target\release\whispr-rs.exe --demo --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

**macOS/Linux:**
```bash
cd whispr-rs
./target/release/whispr-rs --demo --language eng
```

**What happens:**
1. 🎤 Records 5 seconds of audio
2. 📝 Transcribes speech using Whisper
3. 📸 Captures screenshot and extracts text (OCR)
4. 🤖 Sends to Gemini Vision API for intelligent response
5. 💬 Displays response in beautiful chat overlay
6. ⛓️ Logs interaction hash to Solana devnet

---

### ⌨️ Hotkey Listener Mode (Background Mode)

**Windows:**
```powershell
.\target\release\whispr-rs.exe --listen --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

**macOS/Linux:**
```bash
./target/release/whispr-rs --listen --language eng
```

Then press **Ctrl+Shift+W** anywhere to trigger Whispr! 🚀

---

### 🧪 Test Individual Components

**Audio transcription only:**
```bash
.\target\release\whispr-rs.exe --duration 5
```

**Screenshot + OCR only:**
```bash
.\target\release\whispr-rs.exe --ocr --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

**Combined pipeline (ASR + OCR + Gemini + Solana):**
```bash
.\target\release\whispr-rs.exe --combined --solana-log --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

**Skip blockchain logging (faster):**
```bash
.\target\release\whispr-rs.exe --demo --no-chain --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

---

## 📝 CLI Options Reference

| Option | Description | Default |
|--------|-------------|---------|
| `--demo` | Full pipeline with styled output | - |
| `--listen` | Background hotkey listener (Ctrl+Shift+W) | - |
| `--combined` | Run full pipeline (ASR + OCR + Response + Solana) | - |
| `--ocr` | Screenshot and OCR only | - |
| `--no-chain` | Skip blockchain logging | - |
| `--solana-log` | Enable Solana in combined mode | - |
| `-d, --duration` | Recording duration in seconds | 5 |
| `-l, --language` | Tesseract language code (e.g., eng, hrv) | en |
| `-m, --model` | Whisper model path | `./models/ggml-base.en.bin` |
| `--tesseract` | Tesseract executable path | Auto-detect |
| `--gemini-key` | Gemini API key (or use env var) | `$GEMINI_API_KEY` |

---

## 🐛 Troubleshooting

### "Tesseract not found"
- **Windows:** Use `--tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"`
- **macOS/Linux:** Ensure `tesseract` is in PATH: `which tesseract`

### "Failed to load Whisper model"
- Verify model file exists: `ls models/ggml-base.en.bin`
- Re-download if corrupted

### "Gemini API error: 400 Bad Request"
- Verify API key: https://aistudio.google.com/app/apikey
- Check `.env` file exists in `whispr-rs` folder with `GEMINI_API_KEY=your-key`
- Verify environment variable: `echo $env:GEMINI_API_KEY` (Windows) or `echo $GEMINI_API_KEY` (Linux/macOS)
- If key expired, create a new one in Google AI Studio

### "failed to run custom build command for whisper-rs-sys"
- **Windows:** Install LLVM and set `$env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"`
- Ensure CMake is installed and in PATH

### Overlay window not appearing
- Check terminal output for errors
- Ensure you're running in `--demo` or `--listen` mode
- Try running without `--no-chain` flag

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[QUICKSTART.md](./QUICKSTART.md)** | 🚀 5-minute setup guide for beginners |
| **[SETUP_SUMMARY.md](./SETUP_SUMMARY.md)** | 📋 Complete setup summary & demo checklist |
| **[FAQ.md](./FAQ.md)** | ❓ Frequently asked questions & troubleshooting |
| **[PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md)** | 🏗️ Codebase architecture and module breakdown |
| **[PACKAGING.md](./PACKAGING.md)** | 📦 Distribution and packaging guide |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       WHISPR PIPELINE                        │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
    ┌──────────────────┐          ┌──────────────────┐
    │   AUDIO INPUT    │          │  SCREEN CAPTURE  │
    │   (cpal crate)   │          │  (screenshots)   │
    └────────┬─────────┘          └────────┬─────────┘
             │                              │
             ▼                              ▼
    ┌──────────────────┐          ┌──────────────────┐
    │  WHISPER MODEL   │          │   TESSERACT OCR  │
    │  (whisper-rs)    │          │  (text extract)  │
    └────────┬─────────┘          └────────┬─────────┘
             │                              │
             └──────────┬───────────────────┘
                        ▼
              ┌──────────────────┐
              │   GEMINI VISION  │
              │   (text + image) │
              └────────┬─────────┘
                       │
         ┌─────────────┼─────────────┐
         ▼                           ▼
┌──────────────────┐        ┌──────────────────┐
│  CHAT OVERLAY    │        │  SOLANA DEVNET   │
│  (tao + wry)     │        │  (memo tx hash)  │
└──────────────────┘        └──────────────────┘
```

---

## 🛠️ Technology Stack

### **Core**
- **Rust** - Main application language (fast, safe, cross-platform)
- **clap** - CLI argument parsing
- **colored** + **indicatif** - Beautiful terminal UI

### **Audio Processing**
- **cpal** - Cross-platform audio input
- **whisper-rs** - Local Whisper model bindings
- **hound** - WAV file encoding

### **Vision & OCR**
- **screenshots** - Multi-monitor screenshot capture
- **Tesseract** - OCR engine (via CLI)
- **image** - Image processing

### **AI & API**
- **Gemini Vision API** - Multimodal AI response generation
- **reqwest** - HTTP client for API calls
- **serde** + **serde_json** - JSON serialization

### **UI**
- **tao** - Window management (Electron alternative)
- **wry** - WebView rendering
- **rdev** - Global keyboard hooks

### **Blockchain**
- **Node.js** + **@solana/web3.js** - Solana devnet transactions
- **bs58** - Base58 encoding for keys

---

## ✅ Hackathon Deliverables

- [x] **Audio Input & Transcription** - Whisper-powered ASR
- [x] **Screen Capture & OCR** - Tesseract text extraction
- [x] **AI Response Generation** - Gemini Vision API integration
- [x] **Solana Devnet Logging** - Immutable interaction logs
- [x] **CLI Demo Interface** - Beautiful, user-friendly commands
- [x] **Global Hotkey** - Ctrl+Shift+W background listener
- [x] **Chat Overlay** - Cluely-inspired floating window
- [x] **Comprehensive Documentation** - Setup, usage, troubleshooting

---

## 🎓 What I Learned

- **Rust FFI** - Integrating C libraries (whisper.cpp, Tesseract)
- **Audio Processing** - Sample rate conversion, mono downmixing
- **Cross-platform UI** - Window management without Electron
- **Blockchain Integration** - Solana devnet transactions from Rust
- **API Integration** - Gemini Vision multimodal prompts
- **Error Handling** - Rust's Result type for robust error propagation

---

## 🚧 Future Improvements

- [ ] **Real-time streaming** - Continuous audio capture instead of 5-second chunks
- [ ] **Multi-language support** - Dynamic Whisper model loading
- [ ] **Local AI fallback** - Ollama integration for offline usage
- [ ] **Session management** - View past interactions in a dashboard
- [ ] **Encrypted storage** - Secure local cache of transcriptions
- [ ] **Mobile app** - React Native companion app

---

## 📄 License

MIT License - Feel free to use, modify, and distribute!

---

## 🙏 Acknowledgments

- **Colosseum Hackathon 2025** - For the opportunity
- **Whisper.cpp** - For the incredible speech recognition models
- **Google Gemini Team** - For the generous free tier
- **Solana Foundation** - For the devnet infrastructure

---

**Built with ❤️ for the Colosseum Hackathon**  
*Whispr - Kada ne možeš slušati, mi slušamo za tebe.*


