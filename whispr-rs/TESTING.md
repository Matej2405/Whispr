# Testing Guide: Audio Input & Transcription

## Prerequisites

### Windows

1. **Install LLVM/Clang** (required for whisper-rs compilation):
   ```powershell
   # Run PowerShell as Administrator, then:
   winget install LLVM.LLVM
   ```
   
   After installation, find where LLVM was installed (usually `C:\Program Files\LLVM\bin`).

2. **Set environment variable**:
   ```powershell
   # Close current terminal, open new one, then set:
   setx LIBCLANG_PATH "C:\Program Files\LLVM\bin"
   ```
   
   **Important**: Close and reopen your terminal/PowerShell after setting the variable.

   To verify it worked:
   ```powershell
   echo $env:LIBCLANG_PATH
   ```

### macOS

1. **Install LLVM via Homebrew**:
   ```bash
   brew install llvm
   ```

2. **Set environment variable** (add to `~/.zshrc` or `~/.bash_profile`):
   ```bash
   export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
   ```
   
   Then reload:
   ```bash
   source ~/.zshrc  # or source ~/.bash_profile
   ```

## Step 1: Download Whisper Model

1. Download the base English model:
   - **Option A** (Hugging Face): Visit https://huggingface.co/ggerganov/whisper.cpp/tree/main
     - Download `ggml-base.en.bin`
   - **Option B** (Direct): 
     ```powershell
     # Windows PowerShell
     cd "D:\Business\Startups\Whispr\Colloseum hakathon\whispr-rs"
     New-Item -ItemType Directory -Force -Path models
     Invoke-WebRequest -Uri "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin" -OutFile "models\ggml-base.en.bin"
     ```
     
     ```bash
     # macOS/Linux
     mkdir -p models
     curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
     ```

2. Verify the file exists:
   ```powershell
   # Windows
   dir models\ggml-base.en.bin
   
   # macOS/Linux
   ls -lh models/ggml-base.en.bin
   ```
   
   Expected size: ~150MB

## Step 2: Build the Project

1. Navigate to project directory:
   ```powershell
   # Windows
   cd "D:\Business\Startups\Whispr\Colloseum hakathon\whispr-rs"
   
   # macOS
   cd /path/to/whispr-rs
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```
   
   This will take a few minutes on first build (downloading and compiling dependencies).
   
   If you get errors about `LIBCLANG_PATH`, make sure you:
   - Installed LLVM/Clang
   - Set the environment variable
   - Closed and reopened your terminal

## Step 3: Test Audio Recording

### Basic Test (5 seconds default)

1. Make sure your microphone is connected and working.

2. Run the application:
   ```bash
   # Windows
   cargo run --release -- --model .\models\ggml-base.en.bin
   
   # macOS/Linux
   cargo run --release -- --model ./models/ggml-base.en.bin
   ```

3. **Speak clearly into your microphone** when you see "Recording 5s of audio..."

4. Wait for transcription to complete.

5. You should see the transcribed text printed to the console.

### Test with Custom Duration

Record for 10 seconds:
```bash
cargo run --release -- --model .\models\ggml-base.en.bin --duration 10
```

### Test Different Language

If testing non-English (replace model file accordingly):
```bash
cargo run --release -- --model .\models\ggml-base.en.bin --language en
```

## Step 4: Verify Accuracy

1. **Read a known phrase** clearly into your microphone (e.g., "Hello, this is a test of the Whisper transcription system.")

2. Compare the output with what you said.

3. Check for:
   - ✅ Correct words
   - ✅ Proper capitalization (may vary)
   - ✅ Punctuation (may be minimal)

## Troubleshooting

### Error: "no default input device available"
- **Solution**: Make sure your microphone is connected and not being used by another application.

### Error: "failed to load model"
- **Solution**: Verify the model file path is correct and the file exists.

### Error: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll']"
- **Solution**: 
  1. Install LLVM/Clang (see Prerequisites)
  2. Set `LIBCLANG_PATH` environment variable
  3. Close and reopen terminal
  4. Try building again

### Poor transcription quality
- **Solutions**:
  - Speak more clearly and closer to microphone
  - Ensure quiet environment (reduce background noise)
  - Check microphone input levels in system settings
  - Try a different model (larger models = better accuracy, but slower)

### Audio format issues
- The app automatically handles:
  - Different sample rates (resamples to 16kHz)
  - Different channel counts (downmixes to mono)
  - Different bit depths (converts to f32)

## Expected Output

When running successfully, you should see:
```
Recording 5s of audio...
Resampling from 48000 Hz to 16000 Hz (simple linear interpolation)...
Loading Whisper model: .\models\ggml-base.en.bin
Transcribing...
Hello this is a test of the Whisper transcription system
```

## Next Steps

Once testing is complete and working:
- ✅ Audio recording captures 5 seconds correctly
- ✅ Transcription prints text to console
- ✅ Accuracy is acceptable
- ✅ Works on Windows/macOS

You're ready to move to the next ticket!

