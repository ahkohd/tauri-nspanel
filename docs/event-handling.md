# Event Handling

Handle panel events using the `panel_event!` macro to create panel event handlers.

## Basic event handler

```rust
#![allow(clippy::unused_unit)]
use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel!(MyPanel {
        config: {
            can_become_key_window: true
        }
    })
    
    panel_event!(MyPanelEventHandler {
        window_did_become_key(notification: &NSNotification) -> (),
        window_did_resign_key(notification: &NSNotification) -> (),
        window_should_close(window: &NSWindow) -> Bool
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
panel.set_event_handler(Some(handler.as_ref()));
```

## Mouse tracking events

Enable mouse tracking in your panel configuration:

```rust
tauri_panel! {
    panel!(MouseTrackingPanel {
        config: {
            can_become_key_window: true
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
        window_did_become_key(notification: &NSNotification) -> ()
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

panel.set_event_handler(Some(handler.as_ref()));
```

## Selector generation rules

The macro automatically converts method signatures to Objective-C selectors:

- Single parameter: `method_name(param)` → `methodName:`
- Multiple parameters: `method_name(first, second)` → `methodName:second:`
- Parameter names convert from `snake_case` to `camelCase`: `to_size` → `toSize`
  - Example: `method_name(foo: Type1, bar_baz: Type2)` → `methodName:barBaz:`

## Return types

Methods must specify their return type explicitly:

```rust
panel_event!(CompleteEventHandler {
    // Void methods
    window_did_become_key(notification: &NSNotification) -> (),
    window_will_close(notification: &NSNotification) -> (),
    
    // Boolean returns
    window_should_close(window: &NSWindow) -> Bool,
    window_should_zoom(window: &NSWindow, to_frame: &NSRect) -> Bool,
    
    // Value returns
    window_will_resize(sender: &NSWindow, to_size: &NSSize) -> NSSize,
    window_will_use_standard_frame(window: &NSWindow, default_frame: &NSRect) -> NSRect,
    
    // Optional object returns
    window_will_return_field_editor(sender: &NSWindow, to_object: Option<&NSObject>) -> Option<&'static NSObject>
})
```

## Common event handler patterns

### Window lifecycle
```rust
panel_event!(LifecycleEventHandler {
    window_did_become_key(notification: &NSNotification) -> (),
    window_did_resign_key(notification: &NSNotification) -> (),
    window_will_close(notification: &NSNotification) -> (),
    window_did_miniaturize(notification: &NSNotification) -> (),
    window_did_deminiaturize(notification: &NSNotification) -> ()
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
    window_will_resize(sender: &NSWindow, to_size: &NSSize) -> NSSize,
    window_did_resize(notification: &NSNotification) -> ()
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
    window_should_close(window: &NSWindow) -> Bool
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
        config: { can_become_key_window: true }
    })
    
    panel!(UtilityPanel {
        config: { 
            is_floating_panel: true,
            can_become_key_window: false 
        }
    })
    
    panel_event!(MainPanelEventHandler {
        window_did_become_key(notification: &NSNotification) -> ()
    })
    
    panel_event!(UtilityPanelEventHandler {
        window_did_become_key(notification: &NSNotification) -> ()
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
