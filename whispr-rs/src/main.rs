mod audio;
mod ocr;
mod ai;
mod ui;
mod blockchain;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rdev::{listen, Event, EventType, Key};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use audio::{record_audio, downmix_to_mono_f32, linear_resample, transcribe_audio};
use ocr::capture_and_ocr;
use ai::{generate_response, get_api_key_from_env_or_arg};
use ui::show_overlay;
use blockchain::log_to_solana;
use utils::truncate;

#[derive(Parser, Debug, Clone)]
#[command(name = "whispr-rs")]
#[command(about = "Whispr - Discrete AI assistant that listens when you can't", long_about = None)]
struct Args {
    /// Duration in seconds to record audio (default: 5)
    #[arg(short = 'd', long = "duration", default_value_t = 5)]
    duration_secs: u64,

    /// Path to whisper model (default: ./models/ggml-base.en.bin)
    #[arg(short = 'm', long = "model", default_value = "./models/ggml-base.en.bin")]
    model: String,

    /// Language code for transcription (default: en)
    #[arg(short = 'l', long = "language", default_value = "en")]
    language: String,

    /// Run OCR only (screenshot + OCR, no ASR)
    #[arg(long = "ocr", default_value_t = false)]
    ocr: bool,

    /// Run combined mode (ASR + OCR + response)
    #[arg(long = "combined", default_value_t = false)]
    combined: bool,

    /// Path to tesseract executable (optional override)
    #[arg(long = "tesseract")]
    tesseract: Option<String>,

    /// Gemini API key (or set GEMINI_API_KEY env var)
    #[arg(long = "gemini-key")]
    gemini_key: Option<String>,

    /// Log summary to Solana devnet
    #[arg(long = "solana-log", default_value_t = false)]
    solana_log: bool,

    /// Run demo mode with styled output
    #[arg(long = "demo", default_value_t = false)]
    demo: bool,

    /// Skip Solana logging in demo mode
    #[arg(long = "no-chain", default_value_t = false)]
    no_chain: bool,

    /// Listen for global hotkey (Ctrl+Shift+W) to trigger demo
    #[arg(long = "listen", default_value_t = false)]
    listen: bool,

    /// Show overlay window with results
    #[arg(long = "overlay", default_value_t = false)]
    overlay: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    if args.listen {
        return run_hotkey_listener(args);
    }

    if args.demo {
        return run_demo_mode(&args);
    }

    if args.combined {
        return run_combined_mode(&args);
    }

    if args.ocr {
        let (text, _img_path) = capture_and_ocr(&args.language, args.tesseract.as_deref())?;
        if text.trim().is_empty() {
            println!("(no text detected)");
        } else {
            println!("{}", text);
        }
        return Ok(());
    }

    // Default mode: ASR only
    println!("Recording {}s of audio...", args.duration_secs);
    let recorded = record_audio(Duration::from_secs(args.duration_secs))
        .context("failed to record audio")?;

    let mono_f32 = downmix_to_mono_f32(&recorded.data, recorded.channels, recorded.sample_format);

    let target_sr = 16_000u32;
    let audio_16k = if recorded.sample_rate != target_sr {
        println!(
            "Resampling from {} Hz to {} Hz...",
            recorded.sample_rate, target_sr
        );
        linear_resample(&mono_f32, recorded.sample_rate, target_sr)
    } else {
        mono_f32
    };

    println!("Transcribing...");
    let asr_text = transcribe_audio(&args.model, &audio_16k, &args.language)?;

    if asr_text.trim().is_empty() {
        println!("(no speech detected)");
    } else {
        println!("{}", asr_text.trim());
    }

    Ok(())
}

fn run_demo_mode(args: &Args) -> Result<()> {
    println!();
    println!("{}", "╔═══════════════════════════════════════════════════════════════╗".bright_cyan().bold());
    println!("{}", "║          🎤 WHISPR - AI Assistant Demo Mode                ║".bright_cyan().bold());
    println!("{}", "║  Diskretan AI asistent koji sluša kada ti ne možeš         ║".bright_cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan().bold());
    println!();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );

    // Step 1: ASR
    println!("{}", "┌─ Step 1: Audio Input & Transcription (ASR)".bright_yellow().bold());
    spinner.set_message("Recording audio...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let recorded = record_audio(Duration::from_secs(args.duration_secs))
        .context("failed to record audio")?;
    
    spinner.set_message("Processing audio...");
    let mono_f32 = downmix_to_mono_f32(&recorded.data, recorded.channels, recorded.sample_format);
    let target_sr = 16_000u32;
    let audio_16k = if recorded.sample_rate != target_sr {
        linear_resample(&mono_f32, recorded.sample_rate, target_sr)
    } else {
        mono_f32
    };

    spinner.finish_and_clear();
    println!("  {} Audio captured", "✓".green().bold());

    spinner.set_message("Transcribing speech...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let asr_text = transcribe_audio(&args.model, &audio_16k, &args.language)?;

    spinner.finish_and_clear();
    println!("  {} Transcription complete", "✓".green().bold());
    println!("  {} {}", "→".bright_blue(), if asr_text.trim().is_empty() { 
        "(no speech detected)".dimmed().to_string() 
    } else { 
        format!("\"{}\"", asr_text.trim().bright_white()) 
    });
    println!();

    // Step 2: OCR
    println!("{}", "┌─ Step 2: Screen Capture & OCR".bright_yellow().bold());
    spinner.set_message("Capturing screenshot...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let (ocr_text, screenshot_path) = capture_and_ocr(&args.language, args.tesseract.as_deref())?;

    spinner.finish_and_clear();
    println!("  {} Screenshot captured & processed", "✓".green().bold());
    let preview = if ocr_text.trim().is_empty() { 
        "(no text detected)".dimmed().to_string()
    } else {
        let truncated = truncate(&ocr_text, 80);
        format!("\"{}\"", truncated.bright_white())
    };
    println!("  {} {}", "→".bright_blue(), preview);
    println!();

    // Step 3: AI Response
    println!("{}", "┌─ Step 3: AI Response Generation".bright_yellow().bold());
    spinner.set_message("Generating intelligent response with vision...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let api_key = get_api_key_from_env_or_arg(args.gemini_key.as_deref())?;
    let response = generate_response(&asr_text, &screenshot_path, &api_key)
        .context("Gemini API call failed. Check your API key and internet connection.")?;

    spinner.finish_and_clear();
    println!("  {} Response generated", "✓".green().bold());
    println!();
    println!("{}", "┌─────────────────────────────────────────────────────────────┐".bright_cyan());
    println!("{} {}", "│".bright_cyan(), "📝 Summary:".bright_white().bold());
    println!("{} {}", "│".bright_cyan(), response.bright_white());
    println!("{}", "└─────────────────────────────────────────────────────────────┘".bright_cyan());
    println!();

    // Step 4: Solana Logging (optional)
    if args.solana_log && !args.no_chain {
        println!("{}", "┌─ Step 4: Blockchain Logging (Solana Devnet)".bright_yellow().bold());
        spinner.set_message("Posting to Solana...");
        spinner.enable_steady_tick(Duration::from_millis(100));

        match log_to_solana(&response) {
            Ok(result) => {
                spinner.finish_and_clear();
                println!("  {} Transaction confirmed", "✓".green().bold());
                println!("  {} {}", "→".bright_blue(), result.signature.bright_white());
                println!("  {} {}", "🔗".bright_blue(), result.explorer_url.bright_white());
            }
            Err(e) => {
                spinner.finish_and_clear();
                println!("  {} Solana logging failed: {}", "✗".red().bold(), e);
            }
        }
        println!();
    }

    // Show overlay if requested
    if args.overlay {
        show_overlay(&asr_text, &ocr_text, &response)?;
    }

    Ok(())
}

fn run_combined_mode(args: &Args) -> Result<()> {
    println!("=== Combined Mode: ASR + OCR + Response ===\n");

    // Collect ASR
    println!("Recording {}s of audio...", args.duration_secs);
    let recorded = record_audio(Duration::from_secs(args.duration_secs))
        .context("failed to record audio")?;
    let mono_f32 = downmix_to_mono_f32(&recorded.data, recorded.channels, recorded.sample_format);
    let target_sr = 16_000u32;
    let audio_16k = if recorded.sample_rate != target_sr {
        linear_resample(&mono_f32, recorded.sample_rate, target_sr)
    } else {
        mono_f32
    };

    let asr_text = transcribe_audio(&args.model, &audio_16k, &args.language)?;
    println!("ASR: {}\n", if asr_text.trim().is_empty() { "(no speech)" } else { asr_text.trim() });

    // Collect OCR
    println!("Capturing screenshot...");
    let (ocr_text, screenshot_path) = capture_and_ocr(&args.language, args.tesseract.as_deref())?;
    println!("OCR: {}\n", if ocr_text.trim().is_empty() { "(no text)" } else { &ocr_text });

    // Generate response
    let api_key = get_api_key_from_env_or_arg(args.gemini_key.as_deref())?;
    let response = generate_response(&asr_text, &screenshot_path, &api_key)
        .context("Gemini API call failed.")?;
    println!("=== Response ===\n{}", response);

    // Log to Solana if requested
    if args.solana_log {
        println!("\n--- Logging to Solana Devnet ---");
        match log_to_solana(&response) {
            Ok(result) => {
                println!("✅ Transaction confirmed!");
                println!("   Signature: {}", result.signature);
                println!("   Explorer: {}", result.explorer_url);
                println!("   Memo: {}", result.memo);
            }
            Err(e) => {
                eprintln!("⚠️  Solana logging failed: {}", e);
            }
        }
    }

    Ok(())
}

fn run_hotkey_listener(args: Args) -> Result<()> {
    println!("{}", "\n╔═══════════════════════════════════════════════════════════════╗".bright_cyan());
    println!("{}", "║          🎧 WHISPR - Hotkey Listener Mode                  ║".bright_cyan().bold());
    println!("{}", "║  Press Ctrl+Shift+W to trigger capture                     ║".bright_cyan());
    println!("{}", "║  Press Ctrl+C to exit                                       ║".bright_cyan());
    println!("{}", "╚═══════════════════════════════════════════════════════════════╝".bright_cyan());
    println!();
    println!("{}", "Listening for hotkey...".bright_green());

    let args = Arc::new(args);
    let ctrl_pressed = Arc::new(Mutex::new(false));
    let shift_pressed = Arc::new(Mutex::new(false));

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let mut ctrl = ctrl_pressed.lock().unwrap();
                let mut shift = shift_pressed.lock().unwrap();

                match key {
                    Key::ControlLeft | Key::ControlRight => *ctrl = true,
                    Key::ShiftLeft | Key::ShiftRight => *shift = true,
                    Key::KeyW => {
                        if *ctrl && *shift {
                            println!("\n{}", "🔥 Hotkey triggered! Running demo...".bright_yellow().bold());
                            
                            if let Err(e) = run_demo_mode(&args) {
                                eprintln!("{} {}", "Error:".red().bold(), e);
                            }
                            
                            println!("\n{}", "Listening for hotkey...".bright_green());
                        }
                    }
                    _ => {}
                }
            }
            EventType::KeyRelease(key) => {
                let mut ctrl = ctrl_pressed.lock().unwrap();
                let mut shift = shift_pressed.lock().unwrap();

                match key {
                    Key::ControlLeft | Key::ControlRight => *ctrl = false,
                    Key::ShiftLeft | Key::ShiftRight => *shift = false,
                    _ => {}
                }
            }
            _ => {}
        }
    };

    if let Err(error) = listen(callback) {
        eprintln!("{} {:?}", "Hotkey listener error:".red().bold(), error);
    }

    Ok(())
}
