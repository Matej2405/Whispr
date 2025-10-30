# Packaging Guide

## Windows Packaging

### Option 1: Standalone Executable (Simple)

Build a release binary:
```powershell
cargo build --release
```

The executable will be at: `target/release/whispr-rs.exe`

Package with dependencies:
1. Create distribution folder:
```powershell
mkdir dist
Copy-Item target\release\whispr-rs.exe dist\
Copy-Item models\ggml-base.en.bin dist\models\
Copy-Item postMemo.js dist\
Copy-Item package.json dist\
```

2. Install Node.js dependencies:
```powershell
cd dist
npm install --production
```

3. Create `run.bat`:
```batch
@echo off
whispr-rs.exe --demo --language eng --tesseract "C:\Program Files\Tesseract-OCR\tesseract.exe"
pause
```

4. Zip the folder and distribute.

### Option 2: Installer with Inno Setup

1. Install Inno Setup: https://jrsoftware.org/isinfo.php

2. Create `installer.iss`:
```iss
[Setup]
AppName=Whispr
AppVersion=1.0.0
DefaultDirName={autopf}\Whispr
DefaultGroupName=Whispr
OutputDir=.\
OutputBaseFilename=whispr-setup
Compression=lzma2
SolidCompression=yes

[Files]
Source: "target\release\whispr-rs.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "models\ggml-base.en.bin"; DestDir: "{app}\models"; Flags: ignoreversion
Source: "postMemo.js"; DestDir: "{app}"; Flags: ignoreversion
Source: "package.json"; DestDir: "{app}"; Flags: ignoreversion
Source: "node_modules\*"; DestDir: "{app}\node_modules"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\Whispr Demo"; Filename: "{app}\whispr-rs.exe"; Parameters: "--demo"
Name: "{group}\Whispr Listener"; Filename: "{app}\whispr-rs.exe"; Parameters: "--listen"

[Run]
Filename: "{app}\whispr-rs.exe"; Description: "Launch Whispr"; Flags: postinstall nowait
```

3. Compile with Inno Setup.

### Option 3: MSI Installer with WiX

Install WiX toolset and use `cargo-wix`:
```powershell
cargo install cargo-wix
cargo wix init
cargo wix
```

## macOS Packaging

### Option 1: cargo-bundle

1. Install:
```bash
cargo install cargo-bundle
```

2. Add to `Cargo.toml`:
```toml
[package.metadata.bundle]
name = "Whispr"
identifier = "com.whispr.app"
icon = ["icon.icns"]
version = "1.0.0"
copyright = "Copyright (c) 2025"
category = "Productivity"
short_description = "Diskretan AI asistent"
long_description = "AI assistant that listens when you can't"
```

3. Build:
```bash
cargo bundle --release
```

App bundle will be in: `target/release/bundle/osx/Whispr.app`

### Option 2: Manual .app Bundle

```bash
mkdir -p Whispr.app/Contents/MacOS
mkdir -p Whispr.app/Contents/Resources

cp target/release/whispr-rs Whispr.app/Contents/MacOS/
cp -r models Whispr.app/Contents/Resources/
cp postMemo.js Whispr.app/Contents/Resources/
cp package.json Whispr.app/Contents/Resources/

# Create Info.plist
cat > Whispr.app/Contents/Info.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>whispr-rs</string>
    <key>CFBundleIdentifier</key>
    <string>com.whispr.app</string>
    <key>CFBundleName</key>
    <string>Whispr</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF
```

## Linux Packaging

### AppImage

1. Install `cargo-appimage`:
```bash
cargo install cargo-appimage
```

2. Create AppImage:
```bash
cargo appimage
```

### Debian Package

Use `cargo-deb`:
```bash
cargo install cargo-deb
cargo deb
```

Package will be in: `target/debian/whispr_1.0.0_amd64.deb`

## Distribution Checklist

- [ ] Include Whisper model (`ggml-base.en.bin`)
- [ ] Bundle Node.js dependencies (Solana web3.js)
- [ ] Document Tesseract installation requirement
- [ ] Include sample config/env file
- [ ] Create quick-start guide
- [ ] Test on clean system

## Size Optimization

Reduce binary size:
```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
strip = true        # Strip symbols
```

Then build:
```bash
cargo build --release
```

## Dependencies to Document

Required by user:
- **Tesseract OCR** (install from package manager)
- **Node.js** (for Solana logging)
- **Whisper model** (can be downloaded on first run)

Optional:
- **LLVM/Clang** (for building from source)
- **CMake** (for building from source)

## Post-Install Scripts

Create first-run setup:
```bash
#!/bin/bash
# setup.sh

# Download Whisper model if missing
if [ ! -f "models/ggml-base.en.bin" ]; then
    echo "Downloading Whisper model..."
    mkdir -p models
    curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin
fi

# Install Node dependencies
npm install --production

# Check Tesseract
if ! command -v tesseract &> /dev/null; then
    echo "⚠️  Tesseract not found. Please install:"
    echo "   macOS: brew install tesseract"
    echo "   Windows: Download from https://github.com/UB-Mannheim/tesseract/wiki"
fi

echo "✅ Setup complete!"
```

## CI/CD

GitHub Actions example (`.github/workflows/release.yml`):
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: whispr-windows
          path: target/release/whispr-rs.exe

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
        with:
          name: whispr-macos
          path: target/release/whispr-rs
```

