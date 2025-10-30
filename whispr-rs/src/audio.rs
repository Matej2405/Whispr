use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct AudioRecording {
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: SampleFormat,
}

pub fn record_audio(duration: Duration) -> Result<AudioRecording> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow!("no input device available"))?;

    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels();
    let sample_format = config.sample_format();

    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = buffer.clone();
    let start_time = Instant::now();
    let duration_clone = duration;

    let stream = match sample_format {
        SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &_| {
                if start_time.elapsed() < duration_clone {
                    let mut buf = buffer_clone.lock().unwrap();
                    for &sample in data {
                        buf.extend_from_slice(&sample.to_le_bytes());
                    }
                }
            },
            move |err| eprintln!("stream error: {err}"),
            None,
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            move |data: &[u16], _: &_| {
                if start_time.elapsed() < duration_clone {
                    let mut buf = buffer_clone.lock().unwrap();
                    for &sample in data {
                        buf.extend_from_slice(&sample.to_le_bytes());
                    }
                }
            },
            move |err| eprintln!("stream error: {err}"),
            None,
        )?,
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                if start_time.elapsed() < duration_clone {
                    let mut buf = buffer_clone.lock().unwrap();
                    for &sample in data {
                        buf.extend_from_slice(&sample.to_le_bytes());
                    }
                }
            },
            move |err| eprintln!("stream error: {err}"),
            None,
        )?,
        _ => return Err(anyhow!("unsupported sample format: {sample_format}")),
    };

    stream.play()?;
    std::thread::sleep(duration);
    drop(stream);

    let data = Arc::try_unwrap(buffer)
        .map_err(|_| anyhow!("failed to unwrap Arc"))?
        .into_inner()
        .unwrap();

    Ok(AudioRecording {
        data,
        sample_rate,
        channels,
        sample_format,
    })
}

pub fn downmix_to_mono_f32(data: &[u8], channels: u16, sample_format: SampleFormat) -> Vec<f32> {
    let ch = channels as usize;
    match sample_format {
        SampleFormat::I16 => {
            let samples: Vec<i16> = data
                .chunks_exact(2)
                .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            to_mono_f32_i16(&samples, ch)
        }
        SampleFormat::U16 => {
            let samples: Vec<u16> = data
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            to_mono_f32_u16(&samples, ch)
        }
        SampleFormat::F32 => {
            let samples: Vec<f32> = data
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();
            to_mono_f32_f32(&samples, ch)
        }
        _ => Vec::new(),
    }
}

fn to_mono_f32_i16(samples: &[i16], channels: usize) -> Vec<f32> {
    let mut mono = Vec::with_capacity(samples.len() / channels);
    for chunk in samples.chunks(channels) {
        let sum: f32 = chunk.iter().map(|&s| s as f32 / i16::MAX as f32).sum();
        mono.push(sum / channels as f32);
    }
    mono
}

fn to_mono_f32_u16(samples: &[u16], channels: usize) -> Vec<f32> {
    let mut mono = Vec::with_capacity(samples.len() / channels);
    for chunk in samples.chunks(channels) {
        let sum: f32 = chunk
            .iter()
            .map(|&s| (s as f32 - 32768.0) / 32768.0)
            .sum();
        mono.push(sum / channels as f32);
    }
    mono
}

fn to_mono_f32_f32(samples: &[f32], channels: usize) -> Vec<f32> {
    let mut mono = Vec::with_capacity(samples.len() / channels);
    for chunk in samples.chunks(channels) {
        let sum: f32 = chunk.iter().sum();
        mono.push(sum / channels as f32);
    }
    mono
}

pub fn linear_resample(input: &[f32], src_sr: u32, dst_sr: u32) -> Vec<f32> {
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

pub fn transcribe_audio(model_path: &str, audio_data: &[f32], language: &str) -> Result<String> {
    let ctx = WhisperContext::new_with_params(
        model_path,
        WhisperContextParameters::default(),
    )
    .map_err(|e| anyhow!("failed to load model: {e}"))?;
    
    let mut state = ctx
        .create_state()
        .map_err(|e| anyhow!("failed to create whisper state: {e}"))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4) as i32);
    params.set_language(Some(language));
    params.set_translate(false);
    params.set_no_context(true);

    state
        .full(params, audio_data)
        .map_err(|e| anyhow!("whisper full failed: {e}"))?;

    let mut text = String::new();
    let num_segments = state.full_n_segments()?;
    for i in 0..num_segments {
        let segment_text = state.full_get_segment_text(i)?;
        if !segment_text.trim().is_empty() {
            text.push_str(segment_text.trim());
            text.push(' ');
        }
    }

    Ok(text)
}

