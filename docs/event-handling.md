# Event Handling

Handle panel events using the `panel_event!` macro to create panel event handlers.

## Basic event handler

```rust
#![allow(clippy::unused_unit)]
use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel!(MyPanel {
        config: {
            canBecomeKeyWindow: true
        }
    })
    
    panel_event!(MyPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowDidResignKey(notification: &NSNotification) -> (),
        windowShouldClose(window: &NSWindow) -> Bool
    })
}
```

## Setting up event handlers

```rust
// Create the event handler
let handler = MyPanelEventHandler::new();

// Set up callbacks
handler.window_did_become_key(|notification| {
    println!("Panel became key window");
});

handler.window_should_close(|window| {
    println!("Panel should close?");
    Bool::new(true) // Allow closing
});

// Attach to panel
panel.set_event_handler(Some(handler.as_protocol_object()));
```

## Mouse tracking events

Enable mouse tracking in your panel configuration:

```rust
tauri_panel! {
    panel!(MouseTrackingPanel {
        config: {
            canBecomeKeyWindow: true
        }
        with: {
            tracking_area: {
                options: TrackingAreaOptions::new()
                    .active_always()
                    .mouse_entered_and_exited()
                    .mouse_moved()
                    .cursor_update(),
                auto_resize: true
            }
        }
    })
    
    panel_event!(MouseTrackingPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> ()
    })
}
```

### Mouse event callbacks

When tracking is enabled, these methods become available:

```rust
let handler = MouseTrackingPanelEventHandler::new();

handler.on_mouse_entered(|event| {
    println!("Mouse entered the panel");
});

handler.on_mouse_exited(|event| {
    println!("Mouse exited the panel");
});

handler.on_mouse_moved(|event| {
    let location = unsafe { event.locationInWindow() };
    println!("Mouse at: x={}, y={}", location.x, location.y);
});

handler.on_cursor_update(|event| {
    println!("Cursor update requested");
    // Change cursor based on hover state
});

panel.set_event_handler(Some(handler.as_protocol_object()));
```

## Selector generation rules

The macro automatically converts method signatures to Objective-C selectors:

- Single parameter: `methodName(param)` → `methodName:`
- Multiple parameters: `methodName(first, second)` → `methodName:second:`
- Snake_case converts to camelCase: `to_size` → `toSize`

## Return types

Methods must specify their return type explicitly:

```rust
panel_event!(CompleteEventHandler {
    // Void methods
    windowDidBecomeKey(notification: &NSNotification) -> (),
    windowWillClose(notification: &NSNotification) -> (),
    
    // Boolean returns
    windowShouldClose(window: &NSWindow) -> Bool,
    windowShouldZoom(window: &NSWindow, to_frame: &NSRect) -> Bool,
    
    // Value returns
    windowWillResize(sender: &NSWindow, to_size: &NSSize) -> NSSize,
    windowWillUseStandardFrame(window: &NSWindow, default_frame: &NSRect) -> NSRect,
    
    // Optional object returns
    windowWillReturnFieldEditor(sender: &NSWindow, to_object: Option<&NSObject>) -> Option<&'static NSObject>
})
```

## Common event handler patterns

### Window lifecycle
```rust
panel_event!(LifecycleEventHandler {
    windowDidBecomeKey(notification: &NSNotification) -> (),
    windowDidResignKey(notification: &NSNotification) -> (),
    windowWillClose(notification: &NSNotification) -> (),
    windowDidMiniaturize(notification: &NSNotification) -> (),
    windowDidDeminiaturize(notification: &NSNotification) -> ()
})

let handler = LifecycleEventHandler::new();

handler.window_did_become_key(|_| {
    println!("Panel is now the key window");
});

handler.window_will_close(|_| {
    println!("Panel is about to close - cleanup time!");
});
```

### Window resizing
```rust
panel_event!(ResizeEventHandler {
    windowWillResize(sender: &NSWindow, to_size: &NSSize) -> NSSize,
    windowDidResize(notification: &NSNotification) -> ()
})

let handler = ResizeEventHandler::new();

handler.window_will_resize(|window, to_size| {
    // Enforce minimum size
    NSSize {
        width: to_size.width.max(300.0),
        height: to_size.height.max(200.0),
    }
});

handler.window_did_resize(|_| {
    println!("Panel was resized");
});
```

### Close confirmation
```rust
panel_event!(CloseEventHandler {
    windowShouldClose(window: &NSWindow) -> Bool
})

let handler = CloseEventHandler::new();

handler.window_should_close(|window| {
    // Show confirmation dialog or check unsaved changes
    let should_close = true; // Your logic here
    Bool::new(should_close)
});
```

## Advanced: multiple event handlers

You can have different event handlers for different panels:

```rust
tauri_panel! {
    panel!(MainPanel {
        config: { canBecomeKeyWindow: true }
    })
    
    panel!(UtilityPanel {
        config: { 
            isFloatingPanel: true,
            canBecomeKeyWindow: false 
        }
    })
    
    panel_event!(MainPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> ()
    })
    
    panel_event!(UtilityPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> ()
    })
}

// Different handlers for different panels
let main_handler = MainPanelEventHandler::new();
main_handler.window_did_become_key(|_| println!("Main panel active"));

let utility_handler = UtilityPanelEventHandler::new();
utility_handler.window_did_become_key(|_| println!("Utility panel active"));
```

## Event handler cleanup

Event handlers are automatically cleaned up when panels are closed or converted back to windows:

```rust
// This automatically cleans up the event handler
app_handle
    .get_webview_panel("my-panel")
    .ok()
    .and_then(|panel| panel.to_window())
    .map(|window| window.close());
```

## Understanding event handlers

Event handlers created with `panel_event!` are `NSWindowDelegate` implementations. They provide a Rust-friendly way to handle `NSPanel` delegate methods while maintaining type safety and memory management.

When you create an event handler:
- It implements the `NSWindowDelegate` protocol
- Methods are automatically bridged between Rust and Objective-C
- Memory management is handled automatically
- Type conversions are performed safely

## Available NSWindowDelegate methods

See the [objc2-app-kit NSWindowDelegate documentation](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/trait.NSWindowDelegate.html) for the complete list of available delegate methods.

## Next steps

- [Explore Panel Methods](panel-methods.md)
- [Learn about Key Types](key-types.md)
- [Check out Complete Examples](examples.md)
