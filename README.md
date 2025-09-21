# tauri-nspanel

Create macOS panels for your Tauri app. Convert a regular window into a panel, or configure a new window with the panel builder.

> **Note**: For the previous version, see the [v2 branch](https://github.com/ahkohd/tauri-nspanel/tree/v2).

## What are panels?

Panels are a special type of window on macOS ([`NSPanel`](https://developer.apple.com/documentation/appkit/nspanel)) that float above other windows and provide auxiliary controls or information. They're commonly used for tool palettes, inspectors, floating controls, and HUD displays.

## Quick Start

### 1. Installation

```toml
[dependencies]
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel", branch = "v2.1" }
```

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Define and create a panel

```rust
use tauri::{Manager, WebviewUrl};
use tauri_nspanel::{tauri_panel, ManagerExt, PanelBuilder, PanelLevel};

// Define panel class
tauri_panel! {
    panel!(MyPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
}

// Create panel with builder
let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "my-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .level(PanelLevel::Floating)
    .build()?;

panel.show();
```

### 3. Access from commands

```rust
use tauri::Manager;
use tauri_nspanel::ManagerExt;

#[tauri::command]
fn show_panel(app: tauri::AppHandle) {
    if let Ok(panel) = app.get_webview_panel("my-panel") {
        panel.show_and_make_key();
    }
}

#[tauri::command]
fn close_panel(app_handle: tauri::AppHandle) {
    app_handle
        .get_webview_panel("my-panel")
        .ok()
        .and_then(|panel| panel.to_window())
        .map(|window| window.close());
}
```

## Documentation

See the [documentation](docs/) & [API Reference](https://docs.aremu.dev/tauri-nspanel/).

## Features

- Create panels with PanelBuilder API or convert existing windows
- Mouse tracking with enter, exit, and move events
- Handle panel events
- Works with existing Tauri windows and commands
- Thread-safe operations handled on main thread

## Examples

Check out the [examples](examples/) directory.

## Showcase

Some projects using `tauri-nspanel`:
- [Cap](https://github.com/CapSoftware/Cap) - Screen recording
- [Screenpipe](https://github.com/mediar-ai/screenpipe) - AI screen recording
- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Clipboard manager
- [Hyprnote](https://github.com/fastrepl/hyprnote) - Note-taking
- [BongoCat](https://github.com/ayangweb/BongoCat) - Desktop pet
- [Coco](https://github.com/infinilabs/coco-app) - AI Search and Assistant
- [Overlayed](https://github.com/overlayeddev/overlayed) - Screen overlay
- [Verve](https://github.com/ParthJadhav/verve) - Launcher
- [JET Pilot](https://github.com/unxsist/jet-pilot) - Kubernetes manager
- [Buffer](https://buffer.md) - AI-powered Markdown note app

## Contributing

PRs accepted. Please read the Contributing Guide before making a pull request.

## License

MIT or MIT/Apache 2.0 where applicable.
