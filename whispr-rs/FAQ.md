# Whispr - Frequently Asked Questions (FAQ) 🤔

Common questions and answers about Whispr setup, usage, and troubleshooting.

---

## 📦 Installation & Setup

### Q: How long does the first build take?
**A:** First build typically takes 5-10 minutes due to compiling whisper-rs (which includes whisper.cpp). Subsequent builds are much faster (~1-2 minutes for incremental changes).

---

### Q: Why do I need LLVM/Clang on Windows?
**A:** The `whisper-rs` crate uses Rust's FFI to bind to whisper.cpp (a C++ library). On Windows, the build process requires `libclang` to parse C++ headers. macOS/Linux usually have this pre-installed.

**Solution:** Download LLVM from https://github.com/llvm/llvm-project/releases and set:
```powershell
$env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"
```

---

### Q: Can I use a different Whisper model?
**A:** Yes! Download any `ggml-*.bin` model from https://huggingface.co/ggerganov/whisper.cpp and use:
```bash
cargo run --release -- --model path/to/your/model.bin
```

**Model sizes:**
- `tiny` (~75MB) - Fastest, less accurate
- `base` (~140MB) - Good balance (default)
- `small` (~460MB) - Better accuracy
- `medium` (~1.5GB) - High accuracy
- `large` (~3GB) - Best accuracy

---

### Q: Do I need an internet connection?
**A:** Partially. 
- **Audio transcription (Whisper):** ✅ Works offline
- **OCR (Tesseract):** ✅ Works offline
- **AI responses (Gemini):** ❌ Requires internet
- **Blockchain logging (Solana):** ❌ Requires internet

Use `--no-chain` to skip Solana logging if offline.

---

### Q: Is my Gemini API key free?
**A:** Yes! Google provides a generous free tier for Gemini API:
- **Free tier:** 15 requests per minute, 1500 requests per day
- **Vision API:** Included in free tier
- Get yours at: https://aistudio.google.com/app/apikey

---

## 🎯 Usage

### Q: How do I change the recording duration?
**A:** Use the `--duration` flag:
```bash
cargo run --release -- --demo --duration 10  # Record for 10 seconds
```

---

### Q: Can I use a different language for OCR?
**A:** Yes! Use the `--language` flag with Tesseract language codes:
```bash
# Croatian
cargo run --release -- --demo --language hrv

# German
cargo run --release -- --demo --language deu

# French
cargo run --release -- --demo --language fra
```

**Note:** You may need to install additional language data for Tesseract.

---

### Q: How do I change the hotkey from Ctrl+Shift+W?
**A:** Currently, the hotkey is hardcoded in `src/ui.rs`. To change it:
1. Open `whispr-rs/src/ui.rs`
2. Find the hotkey matching code (around line 250)
3. Change `Key::Character("w")` to your desired key
4. Rebuild: `cargo build --release`

---

### Q: Can I run Whispr in the background without the overlay?
**A:** Not currently, but you could modify the code to:
1. Skip `show_overlay()` call in `ui.rs`
2. Just print the response to console instead
3. Or send to a file/webhook

---

### Q: How do I skip the blockchain logging?
**A:** Use the `--no-chain` flag:
```bash
cargo run --release -- --demo --no-chain
```

---

## 🐛 Troubleshooting

### Q: "error: failed to run custom build command for `whisper-rs-sys`"
**A:** This means the whisper-rs compilation failed. Common causes:

**Windows:**
1. LLVM/Clang not installed → Install from https://github.com/llvm/llvm-project/releases
2. `LIBCLANG_PATH` not set → Run `$env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"`
3. CMake not installed → Install from https://cmake.org/download/

**macOS/Linux:**
1. CMake not installed → `brew install cmake` or `sudo apt install cmake`
2. C++ compiler missing → Install Xcode Command Line Tools (macOS) or `build-essential` (Linux)

---

### Q: "Tesseract not found" even though I installed it
**A:** Try these solutions:

**Solution 1:** Specify the path explicitly:
```bash
cargo run --release -- --demo --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
```

**Solution 2:** Add Tesseract to PATH:
```powershell
# Windows
$env:Path += ";C:\Program Files\Tesseract-OCR"
```

**Solution 3:** Set `TESSERACT_PATH` environment variable:
```powershell
$env:TESSERACT_PATH="C:\Program Files\Tesseract-OCR\tesseract.exe"
```

---

### Q: "Error opening data file ... tessdata/eng.traineddata"
**A:** Tesseract can't find its language data files. Solutions:

1. **Verify tessdata folder exists:**
   ```bash
   ls "C:\Program Files\Tesseract-OCR\tessdata"
   ```

2. **Check for eng.traineddata:**
   ```bash
   ls "C:\Program Files\Tesseract-OCR\tessdata\eng.traineddata"
   ```

3. **Set TESSDATA_PREFIX:**
   ```powershell
   $env:TESSDATA_PREFIX="C:\Program Files\Tesseract-OCR\tessdata"
   ```

---

### Q: "Gemini API error: 400 Bad Request"
**A:** This usually means:
1. **Invalid API key** → Verify at https://aistudio.google.com/app/apikey
2. **API key not set** → Check `echo $env:GEMINI_API_KEY` (Windows) or `echo $GEMINI_API_KEY` (Linux/macOS)
3. **Expired key** → Regenerate in AI Studio
4. **Quota exceeded** → Wait for rate limit to reset (free tier: 15 req/min)

---

### Q: "Gemini API error: 404 Not Found"
**A:** This means the API endpoint is wrong (should be fixed in latest code). Ensure you're using:
```
https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent
```

---

### Q: Overlay window doesn't appear
**A:** Check these:
1. **Error in terminal?** → Look for WebView or Window creation errors
2. **Behind other windows?** → Window is set to "always on top" but some fullscreen apps may override
3. **Linux Wayland?** → Try X11 instead: `export GDK_BACKEND=x11`

---

### Q: Audio recording fails or produces no sound
**A:** Common causes:
1. **No microphone detected** → Check system audio settings
2. **Microphone permissions** → Grant microphone access (especially macOS)
3. **Wrong audio device** → Whispr uses default input device
4. **Muted microphone** → Check volume levels

**Debug:** Record audio separately and check the WAV file:
```bash
cargo run --release -- --duration 5
# Check: out/whispr_audio.wav
```

---

### Q: Whisper transcription is empty or inaccurate
**A:** Possible reasons:
1. **Quiet audio** → Speak louder or closer to mic
2. **Background noise** → Use a quieter environment
3. **Wrong language** → Whispr uses English models by default
4. **Model too small** → Try `ggml-small.en.bin` or larger

---

### Q: "Failed to create Solana transaction"
**A:** Usually network-related:
1. **Devnet unavailable** → Solana devnet can be slow/down
2. **Airdrop failed** → Rate limits on devnet airdrops
3. **Node.js script error** → Check `postMemo.js` exists
4. **Dependencies missing** → Run `npm install` in whispr-rs/

**Workaround:** Use `--no-chain` to skip blockchain logging.

---

## 🔐 Security & Privacy

### Q: Is my audio stored anywhere?
**A:** Audio is temporarily saved to `out/whispr_audio.wav` for processing, then transcribed locally. The audio file persists until the next capture (for debugging purposes). You can delete it manually.

---

### Q: What data is sent to Gemini?
**A:** Only:
1. **Transcribed text** (not raw audio)
2. **OCR extracted text**
3. **Screenshot image** (base64 encoded)

**Not sent:** Raw audio, keypair, local file paths.

---

### Q: What's logged to the blockchain?
**A:** Only a **SHA-256 hash** of the AI response. The actual content is NOT stored on-chain. This provides a tamper-proof timestamp without revealing content.

**Example:**
```
Original: "The user asked about crypto prices..."
On-chain: "a3f2c9b1e8d7f6a5..." (64-char hash)
```

---

### Q: Where is the Solana keypair stored?
**A:** In `.whispr-keypair.json` (gitignored). This is a **devnet-only** throwaway keypair generated each run. Never use for mainnet funds!

---

## 💰 Pricing & Costs

### Q: How much does Whispr cost to run?
**A:** Core functionality is **100% free**:
- ✅ Whisper (local model) - Free
- ✅ Tesseract OCR - Free
- ✅ Gemini API - Free tier (15 req/min, 1500/day)
- ✅ Solana Devnet - Free (testnet)

**If upgrading to production:**
- Gemini paid tier: $0.000025 per 1000 chars (~$0.03/month for heavy use)
- Solana mainnet: ~$0.000005 per transaction

---

## 🚀 Performance

### Q: Why is the response slow?
**A:** Breakdown of typical latency:
- Audio recording: **5s** (user-configurable)
- Whisper transcription: **1-3s** (CPU-dependent)
- Screenshot + OCR: **0.5-1s**
- Gemini API call: **2-5s** (network + AI processing)
- Solana transaction: **3-5s** (devnet confirmation)

**Total:** ~12-19 seconds

**Optimization tips:**
1. Use `--duration 3` for faster recording
2. Use `ggml-tiny.en.bin` for faster transcription
3. Use `--no-chain` to skip blockchain (~4s savings)

---

### Q: Can I make it faster?
**A:** Yes! Options:
1. **Smaller Whisper model:** `ggml-tiny.en.bin` is 3-5x faster
2. **Shorter recording:** `--duration 3` instead of 5
3. **Skip Solana:** `--no-chain` flag
4. **Better CPU:** Whisper is CPU-bound
5. **Local AI:** Use Ollama instead of Gemini (no network latency)

---

## 🎨 Customization

### Q: Can I change the overlay design?
**A:** Yes! Edit `src/ui.rs` and modify the HTML/CSS in the `show_overlay()` function. The overlay uses standard web technologies (HTML, CSS, JavaScript).

---

### Q: Can I use a different AI model?
**A:** Yes! The code supports:
1. **Gemini** (default, free)
2. **Ollama** (local, privacy-focused) - Use `--ollama-model mistral`
3. **OpenAI** (paid) - Use `--openai-key YOUR_KEY`

To change the default, edit `src/ai.rs`.

---

### Q: Can I add more OCR languages?
**A:** Yes! Just install the language data for Tesseract:
```bash
# Windows: Download from https://github.com/tesseract-ocr/tessdata
# Place .traineddata files in C:\Program Files\Tesseract-OCR\tessdata

# macOS/Linux
brew install tesseract-lang  # macOS
sudo apt install tesseract-ocr-all  # Linux
```

Then use: `--language deu` (German), `--language fra` (French), etc.

---

## 🤝 Contributing

### Q: How can I contribute to Whispr?
**A:** Contributions welcome!
1. Fork the repository
2. Create a feature branch
3. Make your changes (see `PROJECT_STRUCTURE.md` for code organization)
4. Test with `cargo test` and manual testing
5. Submit a pull request

**Ideas for contributions:**
- Add more AI providers (Anthropic, Cohere, etc.)
- Improve overlay UI
- Add more hotkeys
- Real-time streaming audio
- Multi-language Whisper support

---

### Q: Where should I report bugs?
**A:** Open an issue on GitHub with:
1. Your OS and version
2. Full error message/stack trace
3. Steps to reproduce
4. Output of `check_prerequisites.ps1` or `.sh`

---

## 📚 Additional Resources

- **[QUICKSTART.md](./QUICKSTART.md)** - 5-minute setup guide
- **[PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md)** - Code architecture
- **[DEMO.md](./DEMO.md)** - Demo walkthrough
- **[TESTING.md](./TESTING.md)** - Testing guide

---

## 🎓 Learning Resources

**Want to understand how Whispr works?**

1. **Whisper ASR:** https://github.com/openai/whisper
2. **Tesseract OCR:** https://github.com/tesseract-ocr/tesseract
3. **Gemini Vision API:** https://ai.google.dev/docs
4. **Solana Web3.js:** https://solana-labs.github.io/solana-web3.js/
5. **Rust Audio (cpal):** https://github.com/RustAudio/cpal

---

**Still have questions?** Check the [main README](./README.md) or open a GitHub issue!

