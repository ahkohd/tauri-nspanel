# Panel Methods

All panels implement the `Panel` trait with a comprehensive set of methods for controlling panel behavior.

## Window visibility

### Basic visibility
```rust
panel.show();                    // Show the panel
panel.hide();                    // Hide the panel
panel.is_visible();              // Check if visible
```

### Advanced visibility
```rust
panel.show_and_make_key();       // Show and make key window
panel.make_key_and_order_front(); // Make key and bring to front
panel.order_front_regardless();   // Bring to front without making key
```

## Window state

### Key window state
```rust
panel.make_key_window();         // Make this the key window
panel.resign_key_window();       // Give up key window status
panel.can_become_key_window();   // Check if can become key
```

### Main window state
```rust
panel.make_main_window();        // Make this the main window
panel.resign_main_window();      // Give up main window status
panel.can_become_main_window();  // Check if can become main
```

### Panel state queries
```rust
panel.is_floating_panel();                 // Check if floating
panel.becomes_key_only_if_needed();        // Check key behavior
```

## Window level

```rust
use tauri_nspanel::PanelLevel;

panel.set_level(PanelLevel::Floating.into());  // Use enum
panel.set_level(5);                             // Use raw value
```

## Appearance

### Transparency and opacity
```rust
panel.set_alpha_value(0.9);      // Set opacity (0.0-1.0)
panel.set_opaque(false);          // Make non-opaque
panel.set_transparent(true);      // Transparent background
```

### Visual effects
```rust
panel.set_has_shadow(true);       // Enable drop shadow
panel.set_corner_radius(12.0);    // Rounded corners
```

### Panel behavior
```rust
panel.set_floating_panel(true);             // Float above other windows
panel.set_becomes_key_only_if_needed(true); // Key only when needed
panel.set_hides_on_deactivate(true);        // Hide when app deactivates
panel.set_works_when_modal(true);           // Work with modal dialogs
```

## Size and content

```rust
panel.set_content_size(400.0, 300.0);      // Set content area size
panel.content_view();                        // Get content view
```

## Mouse events

```rust
panel.set_accepts_mouse_moved_events(true);  // Enable mouse move events
panel.set_ignores_mouse_events(false);       // Don't ignore mouse events
panel.set_movable_by_window_background(true); // Allow dragging by background
```

## Window management

```rust
panel.set_released_when_closed(true);       // Release when closed
panel.make_first_responder(Some(&responder)); // Set first responder
```

## Style and collection behavior

```rust
use tauri_nspanel::{StyleMask, CollectionBehavior};

// Set style mask
let style = StyleMask::empty().titled().closable();
panel.set_style_mask(style.into());

// Set collection behavior
let behavior = CollectionBehavior::new().can_join_all_spaces();
panel.set_collection_behavior(behavior.into());
```

## Event handlers

```rust
// Set event handler
panel.set_event_handler(Some(handler.as_ref()));

// Clear event handler
panel.set_event_handler(None);
```

## Panel conversion

```rust
// Convert back to regular window
if let Some(window) = panel.to_window() {
    // Now it's a regular Tauri window
    window.close();
}

// Get underlying NSPanel for advanced operations
let ns_panel = panel.as_panel();
```

## Advanced NSPanel Access

For functionality not directly exposed, access the underlying NSPanel. See the [objc2-app-kit NSPanel documentation](https://docs.rs/objc2-app-kit/latest/i686-unknown-linux-gnu/objc2_app_kit/struct.NSPanel.html) for complete reference.

```rust
use objc2_app_kit::{NSWindowOcclusionState, NSWindowTabbingMode};

let ns_panel = panel.as_panel();

unsafe {
    // Window information
    let frame = ns_panel.frame();
    let screen = ns_panel.screen();
    let backing_scale_factor = ns_panel.backingScaleFactor();
    
    // Advanced styling
    ns_panel.setTitlebarSeparatorStyle(
        objc2_app_kit::NSTitlebarSeparatorStyle::Shadow
    );
    ns_panel.setTitleVisibility(
        objc2_app_kit::NSWindowTitleVisibility::Hidden
    );
    
    // Window tabbing
    ns_panel.setTabbingMode(NSWindowTabbingMode::Disallowed);
    
    // Occlusion state
    let occlusion_state = ns_panel.occlusionState();
    if occlusion_state.contains(NSWindowOcclusionState::Visible) {
        println!("Panel is visible");
    }
}
```

## Method categories

### Window visibility
- `show()`, `hide()`, `is_visible()`
- `show_and_make_key()`, `make_key_and_order_front()`, `order_front_regardless()`

### Window state  
- `make_key_window()`, `resign_key_window()`, `can_become_key_window()`
- `make_main_window()`, `resign_main_window()`, `can_become_main_window()`

### Window level
- `set_level()` (accepts `PanelLevel` enum or `i32`)

### Appearance
- `set_alpha_value()`, `set_has_shadow()`, `set_opaque()` 
- `set_corner_radius()`, `set_transparent()`

### Size
- `set_content_size()`, `content_view()`

### Behavior
- `set_floating_panel()`, `set_hides_on_deactivate()`, `set_works_when_modal()`
- `set_becomes_key_only_if_needed()`, `set_released_when_closed()`

### Mouse events
- `set_accepts_mouse_moved_events()`, `set_ignores_mouse_events()`
- `set_movable_by_window_background()`

### Collection behavior
- `set_collection_behavior()` (accepts `CollectionBehavior` or raw flags)
- `set_style_mask()` (accepts `StyleMask` or raw flags)

### Advanced
- `set_event_handler()`, `make_first_responder()`
- `to_window()`, `as_panel()`, `as_any()`

## Thread safety note

All panel methods must be called on the main thread. The library implements `Send` and `Sync` for compatibility with Tauri's command system, but actual operations are performed on the main thread.

## Next steps

- [Learn about Key Types](key-types.md)  
- [Explore Event Handling](event-handling.md)
- [Check out Complete Examples](examples.md)
