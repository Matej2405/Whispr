use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;

pub fn capture_and_ocr(language: &str, tesseract_cli: Option<&str>) -> Result<(String, PathBuf)> {
    // Capture primary screen
    let screens = screenshots::Screen::all()?;
    let screen = screens
        .get(0)
        .ok_or_else(|| anyhow!("no screens detected for screenshot"))?;

    let imgbuf = screen.capture()?;

    let out_dir = PathBuf::from("out");
    std::fs::create_dir_all(&out_dir)?;
    let img_path = out_dir.join("screenshot.png");

    image::DynamicImage::ImageRgba8(imgbuf).save(&img_path)?;

    // Prefer Tesseract CLI to avoid native linking issues
    let tess_path = find_tesseract(tesseract_cli)?;
    
    // Auto-detect tessdata directory from tesseract path
    let mut cmd = Command::new(&tess_path);
    
    // Try to find tessdata directory
    let tessdata_set = std::env::var("TESSDATA_PREFIX").is_ok();
    if !tessdata_set {
        if let Some(parent) = tess_path.parent() {
            let tessdata = parent.join("tessdata");
            if tessdata.exists() {
                // Use --tessdata-dir flag which is more reliable
                cmd.arg("--tessdata-dir").arg(&tessdata);
            }
        }
    }
    
    let output = cmd
        .arg(&img_path)
        .arg("stdout")
        .arg("-l")
        .arg(language)
        .output()
        .map_err(|e| anyhow!("failed to run tesseract: {e}. Is Tesseract installed and on PATH?"))?;

    if !output.status.success() {
        return Err(anyhow!(
            "tesseract error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    // Clean: collapse whitespace using simple string operations
    let collapsed = raw
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    Ok((collapsed, img_path))
}

fn find_tesseract(override_path: Option<&str>) -> Result<PathBuf> {
    if let Some(p) = override_path {
        let candidate = PathBuf::from(p);
        if candidate.exists() {
            return Ok(candidate);
        } else {
            return Err(anyhow!("Explicitly provided Tesseract path '{}' does not exist", p));
        }
    }
    if let Ok(env_path) = std::env::var("TESSERACT_PATH") {
        let candidate = PathBuf::from(env_path);
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    // Try PATH first
    if let Ok(out) = Command::new("tesseract").arg("--version").output() {
        if out.status.success() {
            return Ok(PathBuf::from("tesseract"));
        }
    }
    // Common install locations
    let candidates = [
        // Windows
        r"C:\Program Files\Tesseract-OCR\tesseract.exe",
        r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
        // macOS Homebrew
        "/opt/homebrew/bin/tesseract",
        "/usr/local/bin/tesseract",
        "/usr/bin/tesseract",
    ];
    for c in candidates {
        let pb = PathBuf::from(c);
        if pb.exists() {
            return Ok(pb);
        }
    }
    Err(anyhow!(
        "Tesseract not found. Install it, add to PATH, or set --tesseract or TESSERACT_PATH"
    ))
}

