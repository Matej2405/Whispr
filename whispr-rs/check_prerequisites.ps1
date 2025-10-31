# Whispr Prerequisites Checker (Windows PowerShell)
# Run this script to verify all dependencies are installed correctly

Write-Host "`n=== Whispr Prerequisites Checker ===" -ForegroundColor Cyan
Write-Host "Checking your system for required dependencies...`n" -ForegroundColor Gray

$allGood = $true

# Function to check if command exists
function Test-Command($cmdname) {
    return [bool](Get-Command -Name $cmdname -ErrorAction SilentlyContinue)
}

# 1. Check Rust
Write-Host "[1/7] Checking Rust..." -ForegroundColor Yellow
if (Test-Command rustc) {
    $rustVersion = rustc --version
    Write-Host "  ✓ $rustVersion" -ForegroundColor Green
    
    # Check if Rust version is 1.83+
    $version = ($rustVersion -split ' ')[1]
    $versionNum = [version]($version -replace '[^0-9.]','')
    if ($versionNum -lt [version]"1.83.0") {
        Write-Host "  ⚠ Warning: Rust 1.83+ recommended. Run 'rustup update stable'" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ✗ Rust not found. Install from https://rustup.rs" -ForegroundColor Red
    $allGood = $false
}

# 2. Check Cargo
Write-Host "`n[2/7] Checking Cargo..." -ForegroundColor Yellow
if (Test-Command cargo) {
    $cargoVersion = cargo --version
    Write-Host "  ✓ $cargoVersion" -ForegroundColor Green
} else {
    Write-Host "  ✗ Cargo not found (should come with Rust)" -ForegroundColor Red
    $allGood = $false
}

# 3. Check Node.js
Write-Host "`n[3/7] Checking Node.js..." -ForegroundColor Yellow
if (Test-Command node) {
    $nodeVersion = node --version
    Write-Host "  ✓ Node.js $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "  ✗ Node.js not found. Install from https://nodejs.org" -ForegroundColor Red
    $allGood = $false
}

# 4. Check npm
Write-Host "`n[4/7] Checking npm..." -ForegroundColor Yellow
if (Test-Command npm) {
    $npmVersion = npm --version
    Write-Host "  ✓ npm $npmVersion" -ForegroundColor Green
    
    # Check if node_modules exists
    if (Test-Path ".\node_modules") {
        Write-Host "  ✓ node_modules installed" -ForegroundColor Green
    } else {
        Write-Host "  ⚠ node_modules not found. Run 'npm install'" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ✗ npm not found (should come with Node.js)" -ForegroundColor Red
    $allGood = $false
}

# 5. Check Tesseract
Write-Host "`n[5/7] Checking Tesseract OCR..." -ForegroundColor Yellow
$tesseractPaths = @(
    "C:\Program Files\Tesseract-OCR\tesseract.exe",
    "C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
    "D:\System\Tessaract\tesseract.exe"
)

$tesseractFound = $false
foreach ($path in $tesseractPaths) {
    if (Test-Path $path) {
        Write-Host "  ✓ Tesseract found at: $path" -ForegroundColor Green
        $tesseractFound = $true
        
        # Check for tessdata
        $tessdataPath = Join-Path (Split-Path $path) "tessdata"
        if (Test-Path $tessdataPath) {
            Write-Host "  ✓ tessdata folder found" -ForegroundColor Green
            
            # Check for English language data
            $engData = Join-Path $tessdataPath "eng.traineddata"
            if (Test-Path $engData) {
                Write-Host "  ✓ English language data found" -ForegroundColor Green
            } else {
                Write-Host "  ⚠ eng.traineddata not found in tessdata" -ForegroundColor Yellow
            }
        } else {
            Write-Host "  ⚠ tessdata folder not found" -ForegroundColor Yellow
        }
        break
    }
}

if (-not $tesseractFound) {
    if (Test-Command tesseract) {
        Write-Host "  ✓ Tesseract found in PATH" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Tesseract not found. Install from:" -ForegroundColor Red
        Write-Host "    https://github.com/UB-Mannheim/tesseract/wiki" -ForegroundColor Gray
        $allGood = $false
    }
}

# 6. Check LLVM/Clang (required for whisper-rs compilation)
Write-Host "`n[6/7] Checking LLVM/Clang..." -ForegroundColor Yellow
if ($env:LIBCLANG_PATH) {
    Write-Host "  ✓ LIBCLANG_PATH set to: $env:LIBCLANG_PATH" -ForegroundColor Green
} else {
    Write-Host "  ⚠ LIBCLANG_PATH not set (needed for first build)" -ForegroundColor Yellow
    Write-Host "    Install LLVM and set: `$env:LIBCLANG_PATH=`"C:\Program Files\LLVM\bin`"" -ForegroundColor Gray
}

if (Test-Command clang) {
    $clangVersion = clang --version | Select-Object -First 1
    Write-Host "  ✓ $clangVersion" -ForegroundColor Green
} else {
    Write-Host "  ⚠ clang not found in PATH" -ForegroundColor Yellow
    Write-Host "    Download from: https://github.com/llvm/llvm-project/releases" -ForegroundColor Gray
}

# 7. Check Whisper Model
Write-Host "`n[7/7] Checking Whisper Model..." -ForegroundColor Yellow
if (Test-Path ".\models\ggml-base.en.bin") {
    $modelSize = (Get-Item ".\models\ggml-base.en.bin").Length / 1MB
    Write-Host "  ✓ Whisper model found (${modelSize}MB)" -ForegroundColor Green
} else {
    Write-Host "  ✗ Whisper model not found at .\models\ggml-base.en.bin" -ForegroundColor Red
    Write-Host "    Download with:" -ForegroundColor Gray
    Write-Host "    mkdir models" -ForegroundColor Gray
    Write-Host "    curl -L https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin -o models/ggml-base.en.bin" -ForegroundColor Gray
    $allGood = $false
}

# Check Gemini API Key
Write-Host "`n[OPTIONAL] Checking Gemini API Key..." -ForegroundColor Yellow
if ($env:GEMINI_API_KEY) {
    $keyLength = $env:GEMINI_API_KEY.Length
    Write-Host "  ✓ GEMINI_API_KEY set (length: $keyLength)" -ForegroundColor Green
} else {
    Write-Host "  ⚠ GEMINI_API_KEY not set (required for AI responses)" -ForegroundColor Yellow
    Write-Host "    Get a key from: https://aistudio.google.com/app/apikey" -ForegroundColor Gray
    Write-Host "    Set with: `$env:GEMINI_API_KEY=`"your-key-here`"" -ForegroundColor Gray
}

# Summary
Write-Host "`n=== Summary ===" -ForegroundColor Cyan
if ($allGood) {
    Write-Host "✓ All required dependencies are installed!" -ForegroundColor Green
    Write-Host "`nYou can now build the project with:" -ForegroundColor Green
    Write-Host "  cargo build --release" -ForegroundColor Gray
} else {
    Write-Host "✗ Some dependencies are missing. Please install them and run this script again." -ForegroundColor Red
}

Write-Host "`nFor detailed setup instructions, see QUICKSTART.md" -ForegroundColor Gray
Write-Host ""

