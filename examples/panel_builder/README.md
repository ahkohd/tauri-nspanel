# Panel Builder Example

This example demonstrates how to use the `PanelBuilder` API to create NSPanel windows programmatically in a Tauri application.

---

![A panel create using PanelBuilder](./demo.png)


## Features Demonstrated

- Creating panels programmatically using `PanelBuilder::new()`
- Configuring panel properties with the builder pattern
- Adding drag regions for window movement
- Setting up event handlers for panel lifecycle events
- Managing multiple windows (main window + floating panel)
- Preventing focus stealing - The app uses `ActivationPolicy::Accessory` (no dock icon) and `no_activate(true)` to ensure panels appear without stealing focus

## Running the Example

```bash
cd examples/panel_builder
npm install
npm run tauri dev
```

## Key Implementation Details

### 1. Preventing Focus Stealing at Window Creation

Since PanelBuilder creates a regular window first before converting it to a panel, the window creation can steal focus. The example demonstrates how to prevent this:

```rust
// In setup(), set the app to Accessory mode (no dock icon, doesn't activate)
app.set_activation_policy(tauri::ActivationPolicy::Accessory);

// When building the panel, use no_activate(true)
PanelBuilder::new(...)
    .no_activate(true)  // Temporarily sets activation policy to Prohibited during window creation
    .build()
```

This ensures the window is created silently before being converted to a panel, preventing any focus interruption.

### 2. Using PanelBuilder

PanelBuilder provides a convenient API that creates a window and converts it to a panel, applying all configurations in one fluent interface:

```rust
let panel = PanelBuilder::<_, MiniPanel>::new(app_handle, "mini-panel")
    .url(WebviewUrl::App("mini.html".into()))
    .title("Mini Panel")
    .position(Position::Logical(LogicalPosition::<f64> {
      x: 100.0,
      y: 100.0,
    }))
    .size(Size::Logical(LogicalSize::<f64> {
      width: 300.0,
      height: 200.0,
    }))
    .level(PanelLevel::Floating)
    .has_shadow(true)
    .collection_behavior(
      CollectionBehavior::new()
        .can_join_all_spaces()
        .stationary()
        .into(),
    )
    .hides_on_deactivate(false)
    .works_when_modal(true)
    .with_window(|w| w.decorations(false))
    .style_mask(StyleMask::empty().nonactivating_panel().resizable().into())
    .no_activate(true)  // Prevent focus stealing when created
    .build()
    .expect("Failed to create mini panel");
```

### 3. Custom Panel Class

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

### 4. Configuring Panel Behavior

The PanelBuilder API allows you to configure all panel properties directly in the builder chain, including style mask, collection behavior, and other settings. This is more convenient than setting them after creation.

### 5. Drag Region

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

### 6. Event Handlers

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

// Show the panel and make it key window
panel.show_and_make_key();
```

