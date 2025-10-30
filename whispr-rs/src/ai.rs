use anyhow::{anyhow, Context, Result};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GeminiPart {
    Text { text: String },
    InlineData { inline_data: InlineData },
}

#[derive(Serialize)]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContentResponse,
}

#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

#[derive(Deserialize)]
struct GeminiPartResponse {
    text: String,
}

pub fn generate_response(asr: &str, screenshot_path: &Path, api_key: &str) -> Result<String> {
    let prompt = if asr.trim().is_empty() {
        "You are Whispr, a helpful AI assistant. The user didn't say anything, but here's what they're looking at. Provide a brief, helpful comment or insight about what you see on their screen (1-2 sentences). Be natural and friendly, like a smart colleague glancing over.".to_string()
    } else {
        format!(
            "You are Whispr, a helpful AI assistant. The user said: \"{}\"\n\nYou can see what's on their screen in the image. Respond naturally and briefly (1-2 sentences) as if you're a smart friend. Provide helpful insight, advice, or a relevant comment based on what they said AND what you see on the screen. Don't just describe the screen - they can already see it. Be conversational and helpful.",
            asr
        )
    };

    // Read and encode the screenshot as base64
    let img_bytes = std::fs::read(screenshot_path)
        .context("Failed to read screenshot file")?;
    let img_base64 = general_purpose::STANDARD.encode(&img_bytes);

    let req = GeminiRequest {
        contents: vec![GeminiContent {
            parts: vec![
                GeminiPart::Text { text: prompt },
                GeminiPart::InlineData {
                    inline_data: InlineData {
                        mime_type: "image/png".to_string(),
                        data: img_base64,
                    },
                },
            ],
        }],
    };

    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent?key={}",
        api_key
    );

    let resp = client
        .post(&url)
        .json(&req)
        .timeout(Duration::from_secs(30))
        .send()
        .context("failed to call Gemini API")?;

    let status = resp.status();
    if !status.is_success() {
        let error_body = resp.text().unwrap_or_else(|_| "Unable to read error body".to_string());
        return Err(anyhow!("Gemini API error: {} - {}", status, error_body));
    }

    let body: GeminiResponse = resp.json().context("failed to parse Gemini response")?;
    
    Ok(body.candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .unwrap_or_else(|| "No response from Gemini".to_string()))
}

pub fn get_api_key_from_env_or_arg(cli_key: Option<&str>) -> Result<String> {
    let gemini_env = std::env::var("GEMINI_API_KEY").ok();
    let gemini_key = cli_key.or_else(|| gemini_env.as_deref());
    
    match gemini_key {
        Some(key) => Ok(key.to_string()),
        None => Err(anyhow!(
            "❌ GEMINI_API_KEY not set!\n\nGet your free API key:\n→ https://aistudio.google.com/app/apikey\n\nThen set it:\n→ setx GEMINI_API_KEY \"your_key_here\"\n\nRestart your terminal and try again."
        ))
    }
}

