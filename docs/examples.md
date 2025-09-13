# Examples

This document covers the available examples and their key concepts.

## Example categories

### Core examples (Rust files)
Located in `/examples/*.rs` - These are simple Rust examples showing specific features:

- [`panel_macro.rs`](/examples/panel_macro.rs) - Basic panel creation with the macro
- [`panel_builder.rs`](/examples/panel_builder.rs) - Using the PanelBuilder API
- [`panel_levels.rs`](/examples/panel_levels.rs) - Different window levels
- [`panel_style_mask.rs`](/examples/panel_style_mask.rs) - NSWindowStyleMask configurations
- [`collection_behavior.rs`](/examples/collection_behavior.rs) - Collection behaviors
- [`builder_with_custom_panel.rs`](/examples/builder_with_custom_panel.rs) - Custom panel classes with PanelBuilder
- [`panel_event_macro.rs`](/examples/panel_event_macro.rs) - Event handling with delegates

### Complete applications
Located in `/examples/*/` - These are full Tauri applications you can run:

- [`basic/`](/examples/basic/) - Basic panel setup in vanilla JavaScript
- [`panel_builder/`](/examples/panel_builder/) - Panel setup using PanelBuilder
- [`fullscreen/`](/examples/fullscreen/) - Panel behavior with fullscreen windows
- [`mouse_tracking/`](/examples/mouse_tracking/) - Mouse tracking events
- [`hover_activate/`](/examples/hover_activate/) - Auto-activate on hover

## Running examples

### Rust examples
```bash
# Run a specific Rust example
cargo run --example panel_macro

# List all available examples
cargo run --example
```

### Full applications
```bash
# Navigate to example directory
cd examples/basic

# Install dependencies and run
pnpm install
pnpm tauri dev
```

## Key concepts by example

### Basic panel creation
**Examples**: `panel_macro.rs`, `basic/`

Learn how to:
- Define panel classes with `tauri_panel!`
- Convert windows to panels
- Basic panel operations

```rust
tauri_panel! {
    panel!(BasicPanel {
        config: {
            can_become_key_window: true,
            can_become_main_window: false
        }
    })
}

let panel = window.to_panel::<BasicPanel>()?;
panel.show();
```

### Advanced panel builder
**Examples**: `panel_builder.rs`, `panel_builder/`

Learn how to:
- Use the PanelBuilder API
- Configure appearance and behavior
- Set up panel styling

```rust
let panel = PanelBuilder::<_, CustomPanel>::new(app.handle(), "panel")
    .url(WebviewUrl::App("index.html".into()))
    .level(PanelLevel::Floating)
    .corner_radius(12.0)
    .transparent(true)
    .build()?;
```

### Event handling
**Examples**: `panel_event_macro.rs`, `mouse_tracking/`

Learn how to:
- Create event handlers with `panel_event!`
- Handle window lifecycle events
- Set up mouse tracking

```rust
#![allow(clippy::unused_unit)]

tauri_panel! {
    panel_event!(PanelEventHandler {
        window_did_become_key(notification: &NSNotification) -> (),
        window_should_close(window: &NSWindow) -> Bool
    })
}

let handler = PanelEventHandler::new();
handler.window_did_become_key(|_| {
    println!("Panel became key!");
});
```

### Mouse tracking
**Examples**: `mouse_tracking/`, `hover_activate/`

Learn how to:
- Enable mouse tracking in panels
- Handle mouse enter/exit/move events
- Create interactive hover effects

```rust
panel!(TrackingPanel {
    config: { can_become_key_window: true }
    with: {
        tracking_area: {
            options: TrackingAreaOptions::new()
                .active_always()
                .mouse_entered_and_exited()
                .mouse_moved(),
            auto_resize: true
        }
    }
})

handler.on_mouse_entered(|_| {
    println!("Mouse entered!");
});
```

### Window levels and styling
**Examples**: `panel_levels.rs`, `panel_style_mask.rs`

Learn how to:
- Control panel layering with levels
- Configure window appearance
- Use different style masks

```rust
// Different levels
panel.set_level(PanelLevel::Floating);   // Above normal windows
panel.set_level(PanelLevel::Status);     // Menu bar level

// Different styles
let style = StyleMask::empty()
    .utility_window()
    .nonactivating_panel();
panel.set_style_mask(style);
```

### Collection behaviors
**Examples**: `collection_behavior.rs`

Learn how to:
- Control Spaces behavior
- Configure fullscreen interactions
- Set up window cycling behavior

```rust
let behavior = CollectionBehavior::new()
    .can_join_all_spaces()
    .stationary()
    .ignores_cycle();
panel.set_collection_behavior(behavior);
```

### Custom panel classes
**Examples**: `builder_with_custom_panel.rs`

Learn how to:
- Create multiple panel types
- Use different configurations
- Combine with PanelBuilder

```rust
tauri_panel! {
    panel!(FloatingPanel {
        config: {
            is_floating_panel: true,
            can_become_key_window: false
        }
    })
    
    panel!(UtilityPanel {
        config: {
            can_become_key_window: true
        }
    })
}
```

### Fullscreen integration
**Examples**: `fullscreen/`

Learn how to:
- Handle fullscreen app interactions
- Configure collection behaviors for fullscreen
- Maintain panel visibility

### Advanced interactions
**Examples**: `hover_activate/`

Learn how to:
- Combine mouse tracking with panel activation
- Create responsive UI behaviors
- Handle complex event sequences

## Example structure

### Rust examples structure
```
examples/
├── panel_macro.rs           # Basic panel macro usage
├── panel_builder.rs         # PanelBuilder API
├── panel_levels.rs          # Window level examples
└── ...
```

### Full app structure
```
examples/basic/
├── src-tauri/
│   ├── src/main.rs         # Rust backend with panels
│   └── Cargo.toml          # Dependencies
├── public/
│   └── index.html          # Frontend HTML
├── package.json            # Node dependencies
└── README.md              # Example-specific docs
```

## Testing examples

### Running rust examples
```bash
# Test compilation of all examples
cargo check --examples

# Run specific example
cargo run --example panel_macro
```

### Running full applications
```bash
# Test all full applications
for dir in examples/*/; do
    if [ -f "$dir/src-tauri/Cargo.toml" ]; then
        echo "Testing $dir"
        cd "$dir" && pnpm install && pnpm tauri build --debug
        cd - > /dev/null
    fi
done
```

## Common patterns

### Panel setup pattern
```rust
// 1. Define panel class
tauri_panel! {
    panel!(MyPanel { config: { can_become_key_window: true } })
}

// 2. Create panel
let panel = PanelBuilder::<_, MyPanel>::new(app, "label")
    .url(WebviewUrl::App("index.html".into()))
    .build()?;

// 3. Show panel
panel.show();
```

### Event handler pattern
```rust
// 1. Define handler in tauri_panel! block
panel_event!(MyHandler {
    window_did_become_key(notification: &NSNotification) -> ()
})

// 2. Create and configure handler
let handler = MyHandler::new();
handler.window_did_become_key(|_| { /* callback */ });

// 3. Attach to panel
panel.set_event_handler(Some(handler.as_ref()));
```

### Cleanup pattern
```rust
// Close panel properly
app.get_webview_panel("label")
    .ok()
    .and_then(|panel| panel.to_window())
    .map(|window| window.close());
```

## Next steps

- [Learn about Installation](installation.md)
- [Read the Getting Started Guide](getting-started.md) 
- [Explore Panel Classes](panel-classes.md)
- [Study the PanelBuilder API](panel-builder.md)
