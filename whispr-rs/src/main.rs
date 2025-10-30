use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::PathBuf;
use std::process::Command;
use regex::Regex;

#[derive(Debug, Parser)]
#[command(name = "whispr-rs", version, about = "Record 5s audio and transcribe with Whisper")] 
struct Args {
    /// Path to Whisper model file (e.g., ./models/ggml-base.en.bin)
    #[arg(short = 'm', long = "model", default_value = "./models/ggml-base.en.bin")]
    model: String,

    /// Duration to record (seconds)
    #[arg(short = 'd', long = "duration", default_value_t = 5u64)]
    duration_secs: u64,

    /// Language hint (e.g., en)
    #[arg(short = 'l', long = "language", default_value = "en")]
    language: String,

    /// Run OCR on a screenshot instead of audio transcription
    #[arg(long = "ocr", default_value_t = false)]
    ocr: bool,

    /// Path to tesseract executable (optional override)
    #[arg(long = "tesseract")]
    tesseract: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    if args.ocr {
        let text = run_ocr(&args.language, args.tesseract.as_deref())?;
        if text.trim().is_empty() {
            println!("(no text detected)");
        } else {
            println!("{}", text);
        }
        return Ok(());
    }

    println!("Recording {}s of audio...", args.duration_secs);
    let recorded = record_audio(Duration::from_secs(args.duration_secs))
        .context("failed to record audio")?;

    // Downmix to mono if needed and convert to f32
    let mono_f32 = downmix_to_mono_f32(&recorded.data, recorded.channels);

    // Resample to 16kHz if needed (simple linear interpolation)
    let target_sr = 16_000u32;
    let audio_16k = if recorded.sample_rate != target_sr {
        println!(
            "Resampling from {} Hz to {} Hz (simple linear interpolation)...",
            recorded.sample_rate, target_sr
        );
        linear_resample(&mono_f32, recorded.sample_rate, target_sr)
    } else {
        mono_f32
    };

    // Transcribe
    println!("Loading Whisper model: {}", args.model);
    let ctx = WhisperContext::new_with_params(
        &args.model,
        WhisperContextParameters::default(),
    )
    .map_err(|e| anyhow!("failed to load model: {e}"))?;
    let mut state = ctx
        .create_state()
        .map_err(|e| anyhow!("failed to create whisper state: {e}"))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) as i32);
    params.set_language(Some(&args.language));
    params.set_translate(false);
    params.set_no_context(true);

    println!("Transcribing...");
    state
        .full(params, &audio_16k)
        .map_err(|e| anyhow!("whisper full failed: {e}"))?;

    // Print segments
    let num_segments = state.full_n_segments()?;
    for i in 0..num_segments {
        let text = state.full_get_segment_text(i)?;
        if !text.trim().is_empty() {
            println!("{}", text.trim());
        }
    }

    Ok(())
}

struct RecordedBuffer {
    data: Vec<f32>,
    sample_rate: u32,
    channels: u16,
}

fn record_audio(duration: Duration) -> Result<RecordedBuffer> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow!("no default input device available"))?;

    let supported_config = device
        .default_input_config()
        .context("failed to get default input config")?;

    let sample_format = supported_config.sample_format();
    let config: cpal::StreamConfig = supported_config.clone().into();
    let sample_rate = config.sample_rate.0;
    let channels = config.channels;

    let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let buffer_clone = Arc::clone(&buffer);

    let start = Instant::now();
    let err_fn = |err| eprintln!("stream error: {err}");

    let stream = match sample_format {
        SampleFormat::I16 => device.build_input_stream(
            &config,
            move |data: &[i16], _| {
                let mut buf = buffer_clone.lock().unwrap();
                for &s in data {
                    buf.push(s as f32 / 32768.0);
                }
            },
            err_fn,
            None,
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &config,
            move |data: &[u16], _| {
                let mut buf = buffer_clone.lock().unwrap();
                for &s in data {
                    // Map 0..65535 to -1.0..1.0
                    buf.push((s as f32 - 32768.0) / 32768.0);
                }
            },
            err_fn,
            None,
        )?,
        SampleFormat::F32 => device.build_input_stream(
            &config,
            move |data: &[f32], _| {
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow!("unsupported sample format: {sample_format:?}")),
    };

    stream.play()?;
    while start.elapsed() < duration {
        std::thread::sleep(Duration::from_millis(50));
    }
    // Dropping stream stops capture
    drop(stream);

    let data = buffer.lock().unwrap().clone();
    Ok(RecordedBuffer {
        data,
        sample_rate,
        channels,
    })
}

fn downmix_to_mono_f32(interleaved: &[f32], channels: u16) -> Vec<f32> {
    if channels <= 1 {
        return interleaved.to_vec();
    }
    let ch = channels as usize;
    let frames = interleaved.len() / ch;
    let mut mono = Vec::with_capacity(frames);
    for i in 0..frames {
        let mut sum = 0.0f32;
        for c in 0..ch {
            sum += interleaved[i * ch + c];
        }
        mono.push(sum / ch as f32);
    }
    mono
}

fn linear_resample(input: &[f32], src_sr: u32, dst_sr: u32) -> Vec<f32> {
    if src_sr == 0 || dst_sr == 0 || input.is_empty() {
        return Vec::new();
    }
    if src_sr == dst_sr {
        return input.to_vec();
    }
    let src_len = input.len() as f32;
    let ratio = dst_sr as f32 / src_sr as f32;
    let out_len = (src_len * ratio).ceil() as usize;
    let mut out = Vec::with_capacity(out_len);
    for n in 0..out_len {
        let pos = n as f32 / ratio;
        let i0 = pos.floor() as usize;
        let i1 = (i0 + 1).min(input.len() - 1);
        let frac = pos - i0 as f32;
        let y = input[i0] * (1.0 - frac) + input[i1] * frac;
        out.push(y);
    }
    out
}

fn run_ocr(language: &str, tesseract_cli: Option<&str>) -> Result<String> {
    // Capture primary screen
    let screens = screenshots::Screen::all()?;
    let screen = screens
        .get(0)
        .ok_or_else(|| anyhow!("no screens detected for screenshot"))?;

    let imgbuf = screen.capture()?; // ImageBuffer<Rgba<u8>, Vec<u8>>

    let out_dir = PathBuf::from("out");
    std::fs::create_dir_all(&out_dir)?;
    let img_path = out_dir.join("screenshot.png");

    image::DynamicImage::ImageRgba8(imgbuf.clone()).save(&img_path)?;

    // Prefer Tesseract CLI to avoid native linking issues
    let tess_path = find_tesseract(tesseract_cli)?;
    let output = Command::new(tess_path)
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
    // Clean: collapse whitespace
    let re = Regex::new(r"\s+").unwrap();
    Ok(re.replace_all(raw.trim(), " ").to_string())
}

fn find_tesseract(override_path: Option<&str>) -> Result<PathBuf> {
    if let Some(p) = override_path {
        let candidate = PathBuf::from(p);
        if candidate.exists() {
            return Ok(candidate);
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
        r"C:\\Program Files\\Tesseract-OCR\\tesseract.exe",
        r"C:\\Program Files (x86)\\Tesseract-OCR\\tesseract.exe",
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
