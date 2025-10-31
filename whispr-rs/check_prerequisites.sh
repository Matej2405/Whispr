#!/bin/bash
# Whispr Prerequisites Checker (Linux/macOS)
# Run this script to verify all dependencies are installed correctly

echo -e "\n\033[1;36m=== Whispr Prerequisites Checker ===\033[0m"
echo -e "\033[0;37mChecking your system for required dependencies...\n\033[0m"

all_good=true

# 1. Check Rust
echo -e "\033[1;33m[1/7] Checking Rust...\033[0m"
if command -v rustc &> /dev/null; then
    rust_version=$(rustc --version)
    echo -e "  \033[0;32m✓ $rust_version\033[0m"
    
    # Extract version number and check if >= 1.83
    version=$(echo $rust_version | awk '{print $2}')
    major=$(echo $version | cut -d'.' -f1)
    minor=$(echo $version | cut -d'.' -f2)
    
    if [ "$major" -lt 1 ] || ([ "$major" -eq 1 ] && [ "$minor" -lt 83 ]); then
        echo -e "  \033[1;33m⚠ Warning: Rust 1.83+ recommended. Run 'rustup update stable'\033[0m"
    fi
else
    echo -e "  \033[0;31m✗ Rust not found. Install from https://rustup.rs\033[0m"
    all_good=false
fi

# 2. Check Cargo
echo -e "\n\033[1;33m[2/7] Checking Cargo...\033[0m"
if command -v cargo &> /dev/null; then
    cargo_version=$(cargo --version)
    echo -e "  \033[0;32m✓ $cargo_version\033[0m"
else
    echo -e "  \033[0;31m✗ Cargo not found (should come with Rust)\033[0m"
    all_good=false
fi

# 3. Check Node.js
echo -e "\n\033[1;33m[3/7] Checking Node.js...\033[0m"
if command -v node &> /dev/null; then
    node_version=$(node --version)
    echo -e "  \033[0;32m✓ Node.js $node_version\033[0m"
else
    echo -e "  \033[0;31m✗ Node.js not found. Install from https://nodejs.org\033[0m"
    all_good=false
fi

# 4. Check npm
echo -e "\n\033[1;33m[4/7] Checking npm...\033[0m"
if command -v npm &> /dev/null; then
    npm_version=$(npm --version)
    echo -e "  \033[0;32m✓ npm $npm_version\033[0m"
    
    # Check if node_modules exists
    if [ -d "./node_modules" ]; then
        echo -e "  \033[0;32m✓ node_modules installed\033[0m"
    else
        echo -e "  \033[1;33m⚠ node_modules not found. Run 'npm install'\033[0m"
    fi
else
    echo -e "  \033[0;31m✗ npm not found (should come with Node.js)\033[0m"
    all_good=false
fi

# 5. Check Tesseract
echo -e "\n\033[1;33m[5/7] Checking Tesseract OCR...\033[0m"
if command -v tesseract &> /dev/null; then
    tesseract_version=$(tesseract --version 2>&1 | head -n1)
    echo -e "  \033[0;32m✓ $tesseract_version\033[0m"
    
    # Check for tessdata
    if [ -n "$TESSDATA_PREFIX" ]; then
        echo -e "  \033[0;32m✓ TESSDATA_PREFIX set to: $TESSDATA_PREFIX\033[0m"
    fi
    
    # Try to find eng.traineddata
    tessdata_locations=(
        "/usr/share/tesseract-ocr/4.00/tessdata"
        "/usr/share/tesseract-ocr/tessdata"
        "/usr/local/share/tessdata"
        "/opt/homebrew/share/tessdata"
        "$TESSDATA_PREFIX"
    )
    
    found_eng=false
    for loc in "${tessdata_locations[@]}"; do
        if [ -f "$loc/eng.traineddata" ]; then
            echo -e "  \033[0;32m✓ English language data found at: $loc\033[0m"
            found_eng=true
            break
        fi
    done
    
    if [ "$found_eng" = false ]; then
        echo -e "  \033[1;33m⚠ eng.traineddata not found (may still work)\033[0m"
    fi
else
    echo -e "  \033[0;31m✗ Tesseract not found.\033[0m"
    echo -e "  \033[0;37m  macOS: brew install tesseract\033[0m"
    echo -e "  \033[0;37m  Linux: sudo apt install tesseract-ocr\033[0m"
    all_good=false
fi

# 6. Check CMake (needed for whisper-rs)
echo -e "\n\033[1;33m[6/7] Checking CMake...\033[0m"
if command -v cmake &> /dev/null; then
    cmake_version=$(cmake --version | head -n1)
    echo -e "  \033[0;32m✓ $cmake_version\033[0m"
else
    echo -e "  \033[1;33m⚠ CMake not found (needed for whisper-rs build)\033[0m"
    echo -e "  \033[0;37m  macOS: brew install cmake\033[0m"
    echo -e "  \033[0;37m  Linux: sudo apt install cmake\033[0m"
fi

# 7. Check Whisper Model
echo -e "\n\033[1;33m[7/7] Checking Whisper Model...\033[0m"
if [ -f "./models/ggml-base.en.bin" ]; then
    model_size=$(du -h "./models/ggml-base.en.bin" | cut -f1)
    echo -e "  \033[0;32m✓ Whisper model found ($model_size)\033[0m"
else
    echo -e "  \033[0;31m✗ Whisper model not found at ./models/ggml-base.en.bin\033[0m"
    echo -e "  \033[0;37m  Download with:\033[0m"
    echo -e "  \033[0;37m  mkdir -p models\033[0m"
    echo -e "  \033[0;37m  curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin\033[0m"
    all_good=false
fi

# Check Gemini API Key
echo -e "\n\033[1;33m[OPTIONAL] Checking Gemini API Key...\033[0m"
if [ -n "$GEMINI_API_KEY" ]; then
    key_length=${#GEMINI_API_KEY}
    echo -e "  \033[0;32m✓ GEMINI_API_KEY set (length: $key_length)\033[0m"
else
    echo -e "  \033[1;33m⚠ GEMINI_API_KEY not set (required for AI responses)\033[0m"
    echo -e "  \033[0;37m  Get a key from: https://aistudio.google.com/app/apikey\033[0m"
    echo -e "  \033[0;37m  Set with: export GEMINI_API_KEY=\"your-key-here\"\033[0m"
fi

# Summary
echo -e "\n\033[1;36m=== Summary ===\033[0m"
if [ "$all_good" = true ]; then
    echo -e "\033[0;32m✓ All required dependencies are installed!\033[0m"
    echo -e "\n\033[0;32mYou can now build the project with:\033[0m"
    echo -e "  \033[0;37mcargo build --release\033[0m"
else
    echo -e "\033[0;31m✗ Some dependencies are missing. Please install them and run this script again.\033[0m"
fi

echo -e "\n\033[0;37mFor detailed setup instructions, see QUICKSTART.md\033[0m"
echo ""

