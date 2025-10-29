# Whispr-rs

A discrete AI assistant that listens when you can't.

## Quick Start

### 1. Install Prerequisites

**Windows:**
```powershell
winget install LLVM.LLVM
setx LIBCLANG_PATH "C:\Program Files\LLVM\bin"
# Close and reopen terminal
```

**macOS:**
```bash
brew install llvm
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"
```

### 2. Download Model

```bash
mkdir models
# Download ggml-base.en.bin from https://huggingface.co/ggerganov/whisper.cpp
# Place it in models/ directory
```

### 3. Build & Run

```bash
cargo build --release
cargo run --release -- --model ./models/ggml-base.en.bin --duration 5
```

## Usage

```bash
whispr-rs [OPTIONS]

Options:
  -m, --model <MODEL>      Path to Whisper model [default: ./models/ggml-base.en.bin]
  -d, --duration <SECS>    Recording duration in seconds [default: 5]
  -l, --language <LANG>    Language hint [default: en]
```

## For Detailed Testing Instructions

See [TESTING.md](./TESTING.md) for step-by-step testing guide.


