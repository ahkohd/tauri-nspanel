# Getting Started

## What are panels?

Panels are a special type of window on macOS ([`NSPanel`](https://developer.apple.com/documentation/appkit/nspanel)) that float above other windows and provide auxiliary controls or information. They're commonly used for:

- Tool palettes
- Inspectors 
- Floating controls
- HUD displays

## Quick start

### 1. Define a panel class

```rust
use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel!(MyPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
    })
}
```

### 2. Create the panel

```rust
use tauri::Manager;
use tauri_nspanel::{PanelBuilder, PanelLevel, WebviewUrl};

let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "my-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .level(PanelLevel::Floating)
    .build()?;

panel.show();
```

### 3. Access from anywhere

```rust
use tauri::Manager;
use tauri_nspanel::ManagerExt;

let panel = app.get_webview_panel("my-panel")?;
panel.show_and_make_key();
```

## Next steps

- [Learn about Panel Classes](panel-classes.md)
- [Explore the PanelBuilder API](panel-builder.md)
- [Handle Panel Events](event-handling.md)
- [Check out the Examples](examples.md)
