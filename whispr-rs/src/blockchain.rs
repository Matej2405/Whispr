use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug)]
pub struct SolanaLogResult {
    pub success: bool,
    pub signature: String,
    #[serde(rename = "explorerUrl")]
    pub explorer_url: String,
    pub memo: String,
    #[allow(dead_code)]
    hash: String,
    #[allow(dead_code)]
    pubkey: String,
}

pub fn log_to_solana(summary: &str) -> Result<SolanaLogResult> {
    let output = Command::new("node")
        .arg("postMemo.js")
        .arg(summary)
        .output()
        .context("failed to execute node postMemo.js")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("postMemo.js failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: SolanaLogResult = serde_json::from_str(&stdout)
        .context("failed to parse JSON response from postMemo.js")?;

    if !result.success {
        return Err(anyhow!("Solana transaction failed"));
    }

    Ok(result)
}

