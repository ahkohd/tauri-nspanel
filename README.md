# tauri-nspanel

Create macOS panels for your Tauri app. Convert a regular window into a panel, or configure a new window with the panel builder.

> **Note**: For the previous version, see the [v2 branch](https://github.com/ahkohd/tauri-nspanel/tree/v2).

## What are panels?

Panels are a special type of window on macOS ([`NSPanel`](https://developer.apple.com/documentation/appkit/nspanel)) that float above other windows and provide auxiliary controls or information. They're commonly used for:
- Tool palettes
- Inspectors 
- Floating controls
- HUD displays

## Installation

Add the plugin to your `Cargo.toml`:

```toml
[dependencies]
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel", branch = "v2.1" }
```

## Usage

### 1. Register the plugin

In your `src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Define custom panel classes

Define custom panel classes using the `tauri_panel!` macro to control panel behavior. Use one `tauri_panel!` per file or scope:

```rust
tauri_panel! {
    panel!(MyPanel {
        config: {
            canBecomeKeyWindow: true
        }
    })
    
    // You can define multiple panels and event handlers in one block
    panel!(MyFloatingPanel {
        config: {
            isFloatingPanel: true,
            canBecomeKeyWindow: false
        }
    })
    
    panel_event!(MyPanelDelegate {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowShouldClose(window: &NSWindow) -> Bool
    })
}
```

### Understanding the macros

#### `panel!` macro
The `panel!` macro creates a custom NSPanel subclass with specified behaviors:
- **config**: Override NSPanel methods that return boolean values
- **with**: Optional configurations like tracking areas

#### `panel_event!` macro
The `panel_event!` macro creates an NSWindowDelegate to handle window events:
- Each method must specify parameter types and return type: `methodName(param: Type) -> ReturnType`
- Method names must match NSWindowDelegate protocol methods
- Parameters in parentheses become part of the Objective-C selector
- The macro automatically converts snake_case to camelCase
- All callbacks receive strongly typed parameters instead of raw pointers

### 3. Create panels using PanelBuilder

The `PanelBuilder` provides a flexible way to create panels with your custom panel classes:

```rust
use tauri::{LogicalPosition, LogicalSize};
use tauri_nspanel::{PanelBuilder, PanelLevel, WebviewUrl};

tauri::Builder::default()
    .setup(|app| {
        // Use the MyPanel class defined above
        let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "my-panel")
            .url(WebviewUrl::App("panel.html".into()))
            .title("My Panel")
            .position(LogicalPosition::new(100.0, 100.0))
            .size(LogicalSize::new(400.0, 300.0))
            .level(PanelLevel::Floating)
            .floating(true)
            .build()?;
        
        panel.show();
        Ok(())
    })
    // ...
```


#### Advanced PanelBuilder configuration

```rust
// Define a more advanced panel class
tauri_panel! {
    panel!(AdvancedPanel {
        config: {
            canBecomeKeyWindow: false,
            isFloatingPanel: true,
            becomesKeyOnlyIfNeeded: true
        }
    })
}

// Use it with advanced configuration
let panel = PanelBuilder::<_, AdvancedPanel>::new(app.handle(), "advanced-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .level(PanelLevel::Status)  // High window level
    .style_mask(
        StyleMask::empty()
            .nonactivating_panel()  // Doesn't activate app
            .utility_window()       // Smaller title bar
            .titled()
            .closable()
    )
    .collection_behavior(
        CollectionBehavior::new()
            .can_join_all_spaces()
            .stationary()
    )
    .alpha_value(0.95)
    .has_shadow(true)
    .with_window(|window| {
        // Access any Tauri window configuration
        window
            .decorations(false)
            .min_inner_size(300.0, 200.0)
            .max_inner_size(800.0, 600.0)
            .resizable(false)
    })
    .build()?;
```

### 4. Convert existing windows to panels

You can convert existing Tauri windows to panels. First define your panel class, then use it:

```rust
use tauri_nspanel::{tauri_panel, WebviewWindowExt};

// Define your custom panel type
tauri_panel! {
    panel!(ConvertedPanel {
        config: {
            canBecomeKeyWindow: false,
            isFloatingPanel: true
        }
    })
}

// Convert existing window to your custom panel type
let window = app.get_webview_window("main").unwrap();
let panel = window.to_panel::<ConvertedPanel>()?;
panel.show();
```

### 5. Create custom panel classes

For advanced use cases, you can define custom panel classes with specific behaviors. The `panel!` and `panel_event!` macros should be used inside the `tauri_panel!` macro.

#### Using `panel!` macro

The `panel!` macro creates a custom NSPanel subclass. Inside the config block, you can override any NSPanel method that returns a boolean value:

```rust
use tauri_nspanel::{tauri_panel, PanelBuilder, TrackingAreaOptions};

// Define custom panel class and event handler together
tauri_panel! {
    panel!(MyFloatingPanel {
        config: {
            // Override NSPanel methods that return boolean values
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            becomesKeyOnlyIfNeeded: true,
            isFloatingPanel: true
        }
        with: {
            tracking_area: {
                // Mouse tracking configuration using the builder pattern
                options: TrackingAreaOptions::new()
                    .active_always()
                    .mouse_entered_and_exited()
                    .mouse_moved(),
                auto_resize: true
            }
        }
    })
    
    panel_event!(MyFloatingPanelDelegate {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowDidResignKey(notification: &NSNotification) -> (),
        windowShouldClose(window: &NSWindow) -> Bool
    })
}

// Use with PanelBuilder
let panel = PanelBuilder::<_, MyFloatingPanel>::new(app.handle(), "my-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .level(PanelLevel::Floating)
    .build()?;

// Or convert existing window to custom panel type
let panel = window.to_panel::<MyFloatingPanel>()?;
```

### 6. Handle panel events with event handlers

Event handlers are typically defined together with panel classes in the `tauri_panel!` macro.

#### Using `panel_event!` macro

The `panel_event!` macro creates an NSWindowDelegate that handles window events. The macro:
- Generates type-safe event handler methods with strongly typed parameters
- Automatically converts method names to proper Objective-C selectors
- Requires explicit return type declarations for all methods
- Supports both void (`-> ()`) and value-returning delegate methods
- **Mouse tracking**: When you enable `tracking_area` in your panel configuration, mouse event callbacks become available

**Selector generation rules:**
- Single parameter: `methodName(param)` → `methodName:`
- Multiple parameters: `methodName(first, second)` → `methodName:second:`
- Snake_case is automatically converted to camelCase: `to_size` → `toSize`

See the [objc2-app-kit NSWindowDelegate documentation](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/trait.NSWindowDelegate.html) for the complete list of available delegate methods.

#### Mouse tracking events

When you enable `tracking_area` in your panel configuration, the following mouse event callbacks become available on your event handler:

- `on_mouse_entered()` - Called when the mouse enters the panel
- `on_mouse_exited()` - Called when the mouse exits the panel  
- `on_mouse_moved()` - Called when the mouse moves within the panel
- `on_cursor_update()` - Called when the cursor needs to be updated

Example with mouse tracking:

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
    
    panel_event!(MouseTrackingPanelDelegate {
        windowDidBecomeKey(notification: &NSNotification) -> ()
    })
}

// Create the event handler and set up mouse callbacks
let handler = MouseTrackingPanelDelegate::new();

// These methods are available when tracking_area is enabled
handler.on_mouse_entered(|event| {
    println!("Mouse entered the panel");
});

handler.on_mouse_exited(|event| {
    println!("Mouse exited the panel");
});

handler.on_mouse_moved(|event| {
    let location = unsafe { event.locationInWindow() };
    println!("Mouse moved to: x={}, y={}", location.x, location.y);
});

handler.on_cursor_update(|event| {
    println!("Cursor update requested");
    // You could change the cursor here based on hover state
});

// Attach the handler to your panel
panel.set_event_handler(Some(handler.as_protocol_object()));
```

Example usage:

```rust
use tauri_nspanel::{tauri_panel, PanelBuilder};

// Define panel class and event handler together
tauri_panel! {
    panel!(MyInteractivePanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false
        }
    })
    
    panel_event!(MyInteractivePanelDelegate {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowDidResignKey(notification: &NSNotification) -> (),
        windowShouldClose(window: &NSWindow) -> Bool,
        windowWillClose(notification: &NSNotification) -> (),
        windowWillResize(sender: &NSWindow, to_size: &NSSize) -> NSSize  // Multiple parameters example
    })
}

// Create and configure the event handler
let handler = MyInteractivePanelDelegate::new();

handler.window_did_become_key(|notification| {
    // notification is already typed as &NSNotification
    println!("Panel became key window: {:?}", notification);
});

handler.window_should_close(|window| {
    println!("Panel should close?: {:?}", window);
    // Return true to allow closing
    Bool::new(true)
});

handler.window_will_resize(|sender, to_size| {
    // Parameters are already typed: sender is &NSWindow, to_size is &NSSize
    println!("Window {:?} will resize to: {:?}", sender, to_size);
    
    // Enforce minimum size
    NSSize {
        width: to_size.width.max(400.0),
        height: to_size.height.max(300.0),
    }
});

let panel = PanelBuilder::<_, MyInteractivePanel>::new(app.handle(), "my-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .build()?;

panel.set_event_handler(Some(handler.as_protocol_object()));
```

**Return types in panel_event! macro:**

Methods must specify their return type explicitly:
- `-> ()` - For void methods (no return value)
- `-> Bool` - For BOOL returns (use `Bool::new(true/false)`)
- `-> NSSize` - For NSSize value returns
- `-> NSRect` - For NSRect value returns  
- `-> NSPoint` - For NSPoint value returns
- `-> Option<&'static NSObject>` - For nullable object returns
- Other types as needed by the delegate method

The macro handles conversion between Rust types and Objective-C types automatically.

### 7. Access panels from anywhere

```rust
use tauri_nspanel::ManagerExt;

let panel = app.get_webview_panel("my-panel")?;
panel.show(); // or `panel.show_and_make_key();`
```

### 8. Panel cleanup

Panels are not automatically released when closed. To ensure proper cleanup:

```rust
panel.set_released_when_closed(true);
// release the event handler if any
panel.set_event_handler(None);
panel.close();
```

## Available Panel Methods

Common panel control methods:
- Window visibility: `show()`, `hide()`, `close()`
- Window state: `make_key_window()`, `resign_key_window()`, `make_main_window()`
- Window level: `set_level()` (accepts `PanelLevel` enum or `i32`)
- Appearance: `set_alpha_value()`, `set_has_shadow()`, `set_opaque()`
- Size: `set_content_size()`
- Behavior: `set_floating_panel()`, `set_hides_on_deactivate()`, `set_works_when_modal()`
- Mouse events: `set_accepts_mouse_moved_events()`, `set_ignores_mouse_events()`
- Collection behavior: `set_collection_behavior()` (accepts `CollectionBehavior` or raw flags)
- And many more...

### Advanced: Accessing the underlying NSPanel

For functionality not directly exposed by the library, you can access the underlying `NSPanel` instance:

```rust
use tauri_nspanel::ManagerExt;
use objc2_app_kit::{NSWindowOcclusionState, NSWindowTabbingMode};

let panel = app.get_webview_panel("my-panel")?;

// Get the underlying NSPanel reference
let ns_panel = panel.as_panel();

// Use any NSPanel/NSWindow method from objc2-app-kit
unsafe {
    // Example: Get window information
    let frame = ns_panel.frame();
    let screen = ns_panel.screen();
    let backing_scale_factor = ns_panel.backingScaleFactor();
    
    // Example: Set tinting and appearance
    ns_panel.setTitlebarSeparatorStyle(objc2_app_kit::NSTitlebarSeparatorStyle::Shadow);
    ns_panel.setTitleVisibility(objc2_app_kit::NSWindowTitleVisibility::Hidden);
    
    // Example: Window tabbing
    ns_panel.setTabbingMode(NSWindowTabbingMode::Disallowed);
    
    // Example: Check occlusion state
    let occlusion_state = ns_panel.occlusionState();
    if occlusion_state.contains(NSWindowOcclusionState::Visible) {
        println!("Window is visible");
    }
}
```

This allows you to:
- Call any NSPanel or NSWindow method not wrapped by the library
- Access window properties like frame, screen, backing scale factor
- Configure advanced window features like tabbing, title visibility, etc.
- Integrate with other macOS APIs that expect NSPanel/NSWindow references

See the [objc2-app-kit NSPanel documentation](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/struct.NSPanel.html) for the complete list of available methods.

**Note**: The NSPanel methods are marked as `unsafe` in objc2-app-kit because they must be called on the main thread. Ensure you're on the main thread when using these methods.

## Key Types

### PanelLevel
Predefined window levels for panels:
```rust
PanelLevel::Normal 
PanelLevel::Floating
PanelLevel::ModalPanel
PanelLevel::Utility
PanelLevel::Status        
PanelLevel::PopUpMenu
PanelLevel::ScreenSaver
PanelLevel::Custom(i32)    // Any custom value
```

### CollectionBehavior
Builder pattern for window collection behaviors:
```rust
CollectionBehavior::new()
    .can_join_all_spaces()    // Show on all spaces
    .stationary()             // Don't move between spaces
    .ignores_cycle()          // Skip in Cmd+Tab
    .full_screen_auxiliary()  // Allow with fullscreen apps
```

### TrackingAreaOptions
Builder pattern for mouse tracking areas:
```rust
TrackingAreaOptions::new()
    .active_always()                // Track in any app state
    .mouse_entered_and_exited()     // Track enter/exit events
    .mouse_moved()                  // Track mouse movement
    .cursor_update()                // Update cursor
```

### StyleMask
Builder pattern for window style masks:
```rust
// Default panel with title bar and controls
StyleMask::new()  // Includes: Titled, Closable, Miniaturizable, Resizable

// Borderless panel
StyleMask::empty()
    .borderless()

// HUD-style panel
StyleMask::empty()
    .hud_window()
    .titled()
    .closable()

// Non-activating utility panel
StyleMask::empty()
    .utility_window()
    .nonactivating_panel()
    .titled()

// Full-featured panel with custom styling
StyleMask::new()
    .full_size_content_view()
    .unified_title_and_toolbar()
```

Available style mask options:
- `titled()` - Window has a title bar
- `closable()` - Window has a close button
- `miniaturizable()` - Window has a minimize button
- `resizable()` - Window can be resized
- `unified_title_and_toolbar()` - Unified title and toolbar appearance
- `full_size_content_view()` - Content extends under title bar
- `utility_window()` - Utility window style (smaller title bar)
- `hud_window()` - HUD window style (dark translucent)
- `nonactivating_panel()` - Panel doesn't activate the app
- `borderless()` - No title bar or border (replaces all other styles)

## Examples

Check out the [examples](/examples) directory for complete working examples:
- [`basic/`](/examples/basic/) - Basic panel setup in a vanilla JavaScript Tauri app
- [`panel_builder/`](/examples/panel_builder/) - Basic panel setup using `PanelBuilder`
- [`panel_macro`](/examples/panel_macro.rs) - Basic panel creation with the macro
- [`panel_builder`](/examples/panel_builder.rs) - Using the PanelBuilder API
- [`panel_levels`](/examples/panel_levels.rs) - Demonstrating different window levels
- [`panel_style_mask`](/examples/panel_style_mask.rs) - Different NSWindowStyleMask configurations
- [`collection_behavior`](/examples/collection_behavior.rs) - Combining collection behaviors
- [`builder_with_custom_panel`](/examples/builder_with_custom_panel.rs) - Using custom panel classes with PanelBuilder
- [`panel_event_macro`](/examples/panel_event_macro.rs) - Event handling with delegates
- [`fullscreen/`](/examples/fullscreen/) - Panel behavior with fullscreen windows (full Tauri app example)
- [`mouse_tracking/`](/examples/mouse_tracking/) - Mouse tracking events with panels (full Tauri app example with mouse enter/exit/move callbacks)
- [`hover_activate/`](/examples/hover_activate/) - Auto-activate panel on hover using mouse tracking (full Tauri app example)

## Thread Safety

The panel types implement `Send` and `Sync` to work with Tauri's command system. However, all actual panel operations must be performed on the main thread. The plugin handles this internally for the provided methods.

## Related Projects

- [tauri-plugin-spotlight](https://github.com/zzzze/tauri-plugin-spotlight)
- [tauri-macos-spotlight-example](https://github.com/ahkohd/tauri-macos-spotlight-example)
- [tauri-macos-menubar-example](https://github.com/ahkohd/tauri-macos-menubar-app-example)

## Showcase

Projects using `tauri-nspanel`:
- [Cap](https://github.com/CapSoftware/Cap)
- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste)
- [Overlayed](https://github.com/overlayeddev/overlayed)
- [Lume](https://github.com/lumehq/lume)
- [Verve](https://github.com/ParthJadhav/verve)
- [Buffer](https://buffer.md)

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License

MIT or MIT/Apache 2.0 where applicable.
