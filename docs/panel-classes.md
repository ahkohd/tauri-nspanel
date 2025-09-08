# Panel Classes

Panel classes define the behavior and characteristics of your panels. They're created using the `tauri_panel!` macro.

## Basic panel definition

```rust
use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel!(MyPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            isFloatingPanel: true
        }
    })
}
```

## Understanding the `panel!` Macro

The `panel!` macro creates a custom NSPanel subclass with specified behaviors:

### Config block

The `config` block allows you to override NSPanel methods that return boolean values:

```rust
panel!(AdvancedPanel {
    config: {
        canBecomeKeyWindow: true,        // Can receive keyboard input
        canBecomeMainWindow: false,      // Can't be the main window
        becomesKeyOnlyIfNeeded: true,    // Only becomes key when needed
        isFloatingPanel: true,           // Floats above other windows
        worksWhenModal: true,            // Works with modal dialogs
        hidesOnDeactivate: false         // Doesn't hide when app deactivates
    }
})
```

### With block (optional)

The `with` block provides additional configurations like mouse tracking:

```rust
panel!(InteractivePanel {
    config: {
        canBecomeKeyWindow: true
    }
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
```

## Multiple panels in one block

You can define multiple panel classes and event handlers together:

```rust
#![allow(clippy::unused_unit)]

tauri_panel! {
    panel!(MainPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false
        }
    })
    
    panel!(FloatingPanel {
        config: {
            isFloatingPanel: true,
            canBecomeKeyWindow: false
        }
    })
    
    panel_event!(PanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowDidResignKey(notification: &NSNotification) -> ()
    })
}
```

## Converting existing windows

You can convert existing Tauri windows to your custom panel types:

```rust
use tauri::Manager;
use tauri_nspanel::WebviewWindowExt;

// Convert existing window to custom panel type
let window = app.get_webview_window("main").unwrap();
let panel = window.to_panel::<MyPanel>()?;
panel.show();
```

## Common panel configurations

### Tool palette
```rust
panel!(ToolPalette {
    config: {
        canBecomeKeyWindow: false,
        isFloatingPanel: true,
        becomesKeyOnlyIfNeeded: true,
        hidesOnDeactivate: false
    }
})
```

### Inspector panel
```rust
panel!(Inspector {
    config: {
        canBecomeKeyWindow: true,
        canBecomeMainWindow: false,
        isFloatingPanel: false,
        worksWhenModal: true
    }
})
```

### HUD display
```rust
panel!(HUD {
    config: {
        canBecomeKeyWindow: false,
        canBecomeMainWindow: false,
        isFloatingPanel: true,
        hidesOnDeactivate: false
    }
})
```

## Next steps

- [Learn about the PanelBuilder](panel-builder.md)
- [Handle Panel Events](event-handling.md)
- [Explore Panel Methods](panel-methods.md)
