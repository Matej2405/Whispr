# Whispr - Quick Start Guide ⚡

Get Whispr running in 5 minutes!

---

## 🔍 Check Prerequisites First!

Before starting, run the prerequisite checker to verify your system:

**Windows (PowerShell):**
```powershell
cd whispr-rs
.\check_prerequisites.ps1
```

**macOS/Linux:**
```bash
cd whispr-rs
chmod +x check_prerequisites.sh
./check_prerequisites.sh
```

This will tell you exactly what's missing!

---

## 📋 Prerequisites Checklist

- [ ] **Rust 1.83+** installed
- [ ] **Node.js** installed
- [ ] **Tesseract OCR** installed
- [ ] **Gemini API key** (free from Google)
- [ ] **LLVM/Clang** (Windows only)
- [ ] **Whisper model** downloaded

---

## 🚀 Installation Steps

### Step 1: Install Rust
```bash
# If you don't have Rust installed:
# Visit https://rustup.rs and follow instructions

# Update existing Rust:
rustup update stable
```

### Step 2: Install Tesseract

**Windows:**
1. Download: https://github.com/UB-Mannheim/tesseract/wiki
2. Run installer (default path: `C:\Program Files\Tesseract-OCR`)
3. Remember the installation path!

**macOS:**
```bash
brew install tesseract
```

**Linux:**
```bash
sudo apt install tesseract-ocr
```

### Step 3: Install LLVM (Windows Only)
1. Download: https://github.com/llvm/llvm-project/releases
2. Install and add to PATH
3. Set environment variable:
```powershell
$env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"
```

### Step 4: Clone & Install Dependencies
```bash
cd whispr-rs
npm install
```

### Step 5: Download Whisper Model
```bash
mkdir models
curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
```

### Step 6: Get Gemini API Key
1. Visit: https://aistudio.google.com/app/apikey
2. Click "Create API Key"
3. Copy your key
4. Create a `.env` file in the `whispr-rs` folder:

**Create `.env` file:**
```bash
cd whispr-rs
# Create a file named .env with this content:
GEMINI_API_KEY=your-api-key-here
```

**Example `.env` file:**
```env
GEMINI_API_KEY=AIzaSyB54z0DmFKHkRhvkEr37921aCZwvwsWYUE
```

> **💡 Tip:** The `.env` file is gitignored, so your API key won't be committed to version control!

### Step 7: Build
```bash
cargo build --release
```

⏳ First build takes 5-10 minutes (compiling whisper-rs). Grab a coffee! ☕

---

## 🎯 Run Your First Demo

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

### What happens:
1. You'll be prompted to speak for 5 seconds 🎤
2. Screenshot is captured 📸
3. Gemini analyzes both audio + image 🧠
4. Overlay window shows the response 💬
5. Transaction logged to Solana ⛓️

> **💡 Note:** The API key is automatically loaded from your `.env` file!

---

## ⌨️ Enable Hotkey Mode

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

## 🐛 Common Issues

### "Tesseract not found"
✅ **Fix:** Use `--tesseract` flag with full path to `tesseract.exe`

### "Failed to load Whisper model"
✅ **Fix:** Verify `models/ggml-base.en.bin` exists

### "Gemini API error: 400"
✅ **Fix:** Double-check your API key is correct and environment variable is set

### "failed to run custom build command for whisper-rs-sys"
✅ **Fix (Windows):** Install LLVM and set `LIBCLANG_PATH`

### Build takes forever
✅ **Expected:** First build compiles whisper-rs (~5-10 minutes)

---

## 📖 Next Steps

- Read full [README.md](./README.md) for all features
- Check [DEMO.md](./DEMO.md) for advanced usage
- See [TESTING.md](./TESTING.md) for component testing

---

## 🎉 You're Ready!

Whispr is now installed and ready to be your discrete AI assistant!

**Pro tip:** Keep the hotkey listener running in the background and press Ctrl+Shift+W whenever you need AI assistance during meetings or conversations.

---

**Need help?** Check the [Troubleshooting section in README.md](./README.md#-troubleshooting)

