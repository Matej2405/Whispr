# Gemini API Setup (FREE!)

Whispr now uses **Google Gemini** as the default AI model - it's FREE and excellent!

## Get Your Free API Key

1. Go to **https://makersuite.google.com/app/apikey** or **https://aistudio.google.com/app/apikey**
2. Click **"Create API Key"**
3. Select a Google Cloud project (or create new)
4. Copy your API key

## Set Up Your Key

### Option 1: Environment Variable (Recommended)
```powershell
# Windows PowerShell
setx GEMINI_API_KEY "YOUR_API_KEY_HERE"
# Restart terminal
```

```bash
# macOS/Linux
export GEMINI_API_KEY="YOUR_API_KEY_HERE"
# Add to ~/.zshrc or ~/.bashrc to persist
```

### Option 2: Command Line Flag
```bash
cargo run --release -- --demo --overlay --gemini-key "YOUR_API_KEY_HERE"
```

## Run Whispr with Gemini

Once your key is set:

```bash
# Demo with Cluely-style overlay
cargo run --release -- --demo --overlay --language eng --tesseract "path/to/tesseract"

# Background listener (Ctrl+Shift+W to trigger)
cargo run --release -- --listen --overlay

# Quick test (no blockchain)
cargo run --release -- --demo --overlay --no-chain --duration 3
```

## Features

✅ **FREE** - Generous free tier from Google  
✅ **Fast** - Sub-second responses  
✅ **Smart** - Powered by Gemini Pro  
✅ **Context-aware** - Understands both speech + screen content  
✅ **Beautiful UI** - Cluely-style chat overlay  

## Fallback Chain

Whispr tries models in this order:
1. **Gemini** (if API key set) ✨ DEFAULT & FREE
2. **Ollama** (if `--ollama-model` specified)
3. **OpenAI** (if API key set)
4. **Rules-based** (always available as last resort)

## Rate Limits

Gemini Free Tier:
- **60 requests per minute**
- **1,500 requests per day**
- More than enough for personal use!

## Example Output

When you trigger Whispr (Ctrl+Shift+W), you'll see a sleek overlay window like Cluely:

```
╔══════════════════════════════════════╗
║  🎤 Whispr AI                        ║
║  Powered by Gemini                   ║
╠══════════════════════════════════════╣
║                                      ║
║  👤 You: "What's on my screen?"      ║
║                                      ║
║  ✨ Whispr: You're looking at a      ║
║     meeting schedule. You have an    ║
║     AT_JS Q&A session now until      ║
║     4:30 PM with EPAM team.          ║
║                                      ║
╚══════════════════════════════════════╝
```

## Troubleshooting

### "Gemini API error: 403"
- Check your API key is correct
- Verify API is enabled in Google Cloud Console
- Try regenerating the key

### "Gemini API error: 429"
- Rate limit reached (60/min or 1500/day)
- Wait a minute or use `--no-chain` to skip AI

### No response
- Check internet connection
- Verify `GEMINI_API_KEY` is set: `echo $env:GEMINI_API_KEY` (Windows) or `echo $GEMINI_API_KEY` (macOS/Linux)

## Privacy

- Gemini API calls are made directly to Google
- Your audio transcript and OCR text are sent to generate responses
- No data is stored by Whispr (only the hash on Solana blockchain)
- See Google's [Gemini API Terms](https://ai.google.dev/terms)

