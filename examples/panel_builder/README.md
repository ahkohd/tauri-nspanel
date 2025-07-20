# Panel Builder Example

This example demonstrates how to use the `PanelBuilder` API to create NSPanel windows programmatically in a Tauri application.

## Features Demonstrated

- **Creating panels programmatically** using `PanelBuilder::new()`
- **Configuring panel properties** with the builder pattern
- **Adding drag regions** for window movement
- **Setting up event handlers** for panel lifecycle events
- **Managing multiple windows** (main window + floating panel)

## Running the Example

```bash
cd examples/panel_builder
npm install
npm run tauri dev
```

## Key Implementation Details

### 1. Using PanelBuilder

Unlike converting an existing window with `window.to_panel()`, PanelBuilder creates panels from scratch:

```rust
let panel = PanelBuilder::<_, MiniPanel>::new(app_handle, "mini-panel")
    .url(WebviewUrl::App("mini.html".into()))
    .title("Mini Panel")
    .position(LogicalPosition::new(100.0, 100.0))
    .size(LogicalSize::new(300.0, 200.0))
    .level(PanelLevel::Floating)
    .has_shadow(true)
    .build()
    .expect("Failed to create mini panel");
```

### 2. Custom Panel Class

The example defines a custom panel class with specific configuration:

```rust
panel!(MiniPanel {
    config: {
        canBecomeKeyWindow: true,
        canBecomeMainWindow: false,
        isFloatingPanel: true
    }
})
```

### 3. Configuring Panel Behavior

After building, you can further configure the panel:

```rust
// Set style mask for non-activating, resizable panel
panel.set_style_mask(StyleMask::empty().nonactivating_panel().resizable().into());

// Configure collection behavior
panel.set_collection_behavior(
    CollectionBehavior::new()
        .can_join_all_spaces()
        .stationary()
        .into(),
);

// Other settings
panel.set_hides_on_deactivate(false);
panel.set_works_when_modal(true);
```

### 4. Drag Region

The mini panel includes a drag region for window movement:

```html
<div data-tauri-drag-region class="drag-region">
    ↔ Mini Panel - Drag to Move
</div>
```

Required permissions in `capabilities/default.json`:
```json
{
    "permissions": [
        "core:window:allow-start-dragging",
        "core:window:deny-internal-toggle-maximize"
    ]
}
```

### 5. Event Handlers

The example shows how to attach event handlers to the panel:

```rust
let handler = MiniPanelEventHandler::new();

handler.window_did_become_key(move |notification| {
    println!("Mini panel became key window!");
});

handler.window_did_resign_key(|_notification| {
    println!("Mini panel resigned from key window!");
});

panel.set_event_handler(Some(handler.as_protocol_object()));
```

## PanelBuilder API Reference

### Builder Methods

- `new(handle, label)` - Create a new builder
- `url(WebviewUrl)` - Set the webview URL
- `title(String)` - Set the window title
- `position(Position)` - Set initial position
- `size(Size)` - Set initial size
- `level(PanelLevel)` - Set window level (Floating, Status, etc.)
- `has_shadow(bool)` - Enable/disable window shadow
- `floating(bool)` - Set floating panel behavior
- `build()` - Create the panel

### Panel Levels

- `PanelLevel::Normal` - Standard window level
- `PanelLevel::Floating` - Floats above normal windows
- `PanelLevel::Status` - Status bar level (very high)
- `PanelLevel::MainMenu` - Main menu level
- `PanelLevel::PopUpMenu` - Pop-up menu level
- `PanelLevel::ScreenSaver` - Screen saver level

### Style Masks

- `nonactivating_panel()` - Panel doesn't activate the app
- `resizable()` - Allow window resizing
- `titled()` - Show title bar
- `closable()` - Show close button
- `miniaturizable()` - Show minimize button
- `borderless()` - Remove window chrome

### Collection Behaviors

- `can_join_all_spaces()` - Panel appears on all spaces
- `stationary()` - Panel doesn't move with spaces
- `full_screen_auxiliary()` - Can appear over fullscreen windows
- `ignores_cycle()` - Excluded from Cmd+Tab cycling

## Notes

- PanelBuilder is ideal for creating auxiliary windows like tool palettes, floating inspectors, or utility panels
- The panel is created during app initialization and persists throughout the app lifecycle
- Commands allow showing/hiding/closing the panel from the frontend
- The main window remains a standard window while the panel floats above it