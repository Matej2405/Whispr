use anyhow::Result;
use tao::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    event::{Event as TaoEvent, WindowEvent},
    dpi::{LogicalSize, LogicalPosition},
};
use wry::webview::WebViewBuilder;

pub fn show_overlay(asr: &str, _ocr: &str, response: &str) -> Result<()> {
    let event_loop = EventLoop::new();
    
    let window = WindowBuilder::new()
        .with_title("Whispr AI")
        .with_inner_size(LogicalSize::new(500, 400))
        .with_resizable(true)
        .with_always_on_top(true)
        .with_decorations(false)
        .build(&event_loop)?;

    // Position in center of screen
    if let Some(monitor) = window.current_monitor() {
        let screen_size = monitor.size();
        let window_size = window.outer_size();
        window.set_outer_position(LogicalPosition::new(
            (screen_size.width as i32 - window_size.width as i32) / 2,
            (screen_size.height as i32 - window_size.height as i32) / 2,
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Helvetica Neue', Arial, sans-serif;
            background: #1e293b;
            color: #e2e8f0;
            overflow: hidden;
            height: 100vh;
            display: flex;
            flex-direction: column;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
        }}
        .header {{
            background: #0f172a;
            padding: 20px 24px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 16px 16px 0 0;
        }}
        .header-left {{
            display: flex;
            align-items: center;
            gap: 12px;
        }}
        .logo {{
            font-size: 24px;
            animation: pulse 2s ease-in-out infinite;
        }}
        @keyframes pulse {{
            0%, 100% {{ opacity: 1; }}
            50% {{ opacity: 0.7; }}
        }}
        .title {{
            font-size: 16px;
            font-weight: 600;
            color: #f1f5f9;
        }}
        .badge {{
            background: #3b82f6;
            color: white;
            font-size: 10px;
            padding: 3px 8px;
            border-radius: 12px;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }}
        .close-btn {{
            width: 28px;
            height: 28px;
            border-radius: 8px;
            background: rgba(255, 255, 255, 0.1);
            border: none;
            color: #94a3b8;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 18px;
            transition: all 0.2s;
        }}
        .close-btn:hover {{
            background: rgba(255, 255, 255, 0.2);
            color: #f1f5f9;
        }}
        .chat-container {{
            flex: 1;
            overflow-y: auto;
            padding: 24px;
            display: flex;
            flex-direction: column;
            gap: 20px;
            background: #1e293b;
        }}
        .chat-container::-webkit-scrollbar {{
            width: 6px;
        }}
        .chat-container::-webkit-scrollbar-track {{
            background: transparent;
        }}
        .chat-container::-webkit-scrollbar-thumb {{
            background: rgba(255, 255, 255, 0.2);
            border-radius: 3px;
        }}
        .user-section {{
            animation: slideUp 0.4s ease-out;
        }}
        .ai-section {{
            animation: slideUp 0.5s ease-out;
        }}
        @keyframes slideUp {{
            from {{
                opacity: 0;
                transform: translateY(20px);
            }}
            to {{
                opacity: 1;
                transform: translateY(0);
            }}
        }}
        .section-label {{
            font-size: 11px;
            font-weight: 600;
            color: #64748b;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-bottom: 12px;
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .bubble {{
            background: #0f172a;
            border-radius: 16px;
            padding: 16px 20px;
            line-height: 1.6;
            font-size: 14px;
            color: #cbd5e1;
            border: 1px solid rgba(255, 255, 255, 0.05);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        }}
        .ai-bubble {{
            background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
            color: white;
            font-size: 15px;
            font-weight: 500;
            border: none;
        }}
        .empty {{
            color: #64748b;
            font-style: italic;
            font-size: 13px;
        }}
        .input-hint {{
            padding: 16px 24px;
            text-align: center;
            font-size: 12px;
            color: #64748b;
            background: #0f172a;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 0 0 16px 16px;
        }}
        .input-hint kbd {{
            background: #1e293b;
            padding: 4px 8px;
            border-radius: 6px;
            font-family: 'Courier New', monospace;
            margin: 0 3px;
            font-size: 11px;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }}
    </style>
</head>
<body>
    <div class="header">
        <div class="header-left">
            <div class="logo">🎤</div>
            <div class="title">Whispr</div>
            <div class="badge">AI</div>
        </div>
        <button class="close-btn" onclick="window.close()">✕</button>
    </div>
    
    <div class="chat-container">
        <div class="user-section">
            <div class="section-label">💬 What you said</div>
            <div class="bubble">
                {}
            </div>
        </div>
        
        <div class="ai-section">
            <div class="section-label">✨ Whispr Response</div>
            <div class="bubble ai-bubble">
                {}
            </div>
        </div>
    </div>
    
    <div class="input-hint">
        Press <kbd>ESC</kbd> to close • <kbd>Ctrl+Shift+W</kbd> to capture again
    </div>
</body>
</html>"#,
        if asr.trim().is_empty() {
            "<span class='empty'>You didn't say anything, but I can see your screen...</span>".to_string()
        } else {
            html_escape(asr)
        },
        html_escape(response)
    );

    let _webview = WebViewBuilder::new(window)?
        .with_html(&html)?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            TaoEvent::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            TaoEvent::WindowEvent {
                event: WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _, .. },
                ..
            } => {
                if event.state == tao::event::ElementState::Pressed {
                    if let tao::keyboard::Key::Character(ref ch) = event.logical_key {
                        let ch_str: &str = ch;
                        if ch_str == "Escape" || ch_str == "\u{001b}" {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
            }
            _ => {}
        }
    });
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\n', "<br>")
}

