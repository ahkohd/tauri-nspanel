# Key Types

This document covers the main types and enums used with tauri-nspanel.

## PanelLevel

Predefined window levels for panels. Higher levels appear above lower levels.

```rust
use tauri_nspanel::PanelLevel;

panel.set_level(PanelLevel::Normal);      // Standard window level
panel.set_level(PanelLevel::Floating);    // Floating above normal windows
panel.set_level(PanelLevel::ModalPanel);  // Modal panel level
panel.set_level(PanelLevel::Utility);     // Utility window level
panel.set_level(PanelLevel::Status);      // Status/menu bar level
panel.set_level(PanelLevel::PopUpMenu);   // Pop-up menu level
panel.set_level(PanelLevel::ScreenSaver); // Screen saver level
panel.set_level(PanelLevel::Custom(25));  // Custom level value
```

### Level Hierarchy (Lowest to Highest)
1. `Normal` - Regular application windows
2. `Floating` - Floating palettes and inspectors  
3. `ModalPanel` - Modal dialogs and sheets
4. `Utility` - Utility windows and panels
5. `Status` - Menu bar and status items
6. `PopUpMenu` - Pop-up menus and tooltips
7. `ScreenSaver` - Screen saver windows

## CollectionBehavior

Builder pattern for NSWindow collection behaviors. Controls how windows behave with Spaces, Mission Control, and fullscreen apps.

```rust
use tauri_nspanel::CollectionBehavior;

// Basic behaviors
let behavior = CollectionBehavior::new()
    .can_join_all_spaces()      // Show on all Spaces
    .stationary()               // Don't move between Spaces
    .ignores_cycle();           // Skip in Cmd+Tab cycling

panel.set_collection_behavior(behavior);
```

### Available Behaviors

#### Space Management
```rust
CollectionBehavior::new()
    .can_join_all_spaces()      // Window appears on all Spaces
    .move_to_active_space()     // Move window to active Space
    .stationary()               // Window doesn't move between Spaces
```

#### App Switching
```rust
CollectionBehavior::new()
    .ignores_cycle()            // Skip in Cmd+Tab and window cycling
    .participates_in_cycle()    // Include in Cmd+Tab (default)
```

#### Fullscreen Support
```rust
CollectionBehavior::new()
    .full_screen_primary()      // Can be primary fullscreen window
    .full_screen_auxiliary()    // Can coexist with fullscreen apps
    .full_screen_none()         // Hide when another app is fullscreen
    .full_screen_allows_tiling() // Allow window tiling
```

#### Advanced Options
```rust
CollectionBehavior::new()
    .managed()                  // Managed by Exposé and Spaces
    .transient()                // Transient window (tooltips, etc.)
    .disallow_tiling()          // Prevent window tiling
```

### Common Combinations

#### Floating Tool Palette
```rust
CollectionBehavior::new()
    .can_join_all_spaces()
    .stationary()
    .full_screen_auxiliary()
```

#### HUD Display  
```rust
CollectionBehavior::new()
    .can_join_all_spaces()
    .stationary()
    .ignores_cycle()
    .full_screen_auxiliary()
```

#### Temporary Panel
```rust
CollectionBehavior::new()
    .transient()
    .ignores_cycle()
    .full_screen_none()
```

## StyleMask

Builder pattern for NSWindow style masks. Controls window appearance and features.

```rust
use tauri_nspanel::StyleMask;

// Default panel with standard controls
let style = StyleMask::new();  // Titled, Closable, Miniaturizable, Resizable

// Minimal borderless panel
let style = StyleMask::empty().borderless();

// Custom styled panel
let style = StyleMask::empty()
    .titled()
    .closable()
    .utility_window()
    .nonactivating_panel();

panel.set_style_mask(style);
```

### Basic Styles
```rust
StyleMask::empty()
    .titled()                   // Window has a title bar
    .closable()                 // Window has a close button  
    .miniaturizable()           // Window has a minimize button
    .resizable()                // Window can be resized
```

### Special Window Types
```rust
StyleMask::empty()
    .utility_window()           // Utility window (smaller title bar)
    .hud_window()               // HUD window (dark translucent)
    .borderless()               // No title bar or border
```

### Panel-Specific Styles
```rust
StyleMask::empty()
    .nonactivating_panel()      // Panel doesn't activate the app
    .dock_modal()               // Modal relative to dock
```

### Advanced Styling
```rust
StyleMask::empty()
    .full_size_content_view()   // Content extends under title bar
    .unified_title_and_toolbar() // Unified title and toolbar
    .textured_background()      // Textured metal appearance
```

### Common Style Combinations

#### Clean Floating Panel
```rust
StyleMask::empty()
    .utility_window()
    .nonactivating_panel()
    .titled()
    .closable()
```

#### Borderless HUD
```rust
StyleMask::empty()
    .borderless()
    .nonactivating_panel()
```

#### Tool Palette
```rust
StyleMask::empty()
    .utility_window()
    .titled()
    .closable()
    .resizable()
```

## TrackingAreaOptions

Builder pattern for mouse tracking areas. Used in panel configurations with mouse tracking.

```rust
use tauri_nspanel::TrackingAreaOptions;

panel! {
    MyTrackingPanel {
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
    }
}
```

### Activation States
```rust
TrackingAreaOptions::new()
    .active_always()           // Track regardless of app state
    .active_when_first_responder() // Track when first responder
    .active_in_key_window()    // Track when in key window
    .active_in_active_app()    // Track when app is active
```

### Event Types  
```rust
TrackingAreaOptions::new()
    .mouse_entered_and_exited() // Track mouse enter/exit
    .mouse_moved()              // Track mouse movement
    .cursor_update()            // Update cursor on hover
```

### Behavior Options
```rust  
TrackingAreaOptions::new()
    .assume_inside()            // Start assuming mouse is inside
    .invert_state()            // Invert the tracking state
    .enable_mouse_moved_events() // Enable mouse moved events
```

### Common Tracking Configurations

#### Basic Hover Detection
```rust
TrackingAreaOptions::new()
    .active_always()
    .mouse_entered_and_exited()
```

#### Interactive Panel
```rust
TrackingAreaOptions::new()
    .active_always()
    .mouse_entered_and_exited()
    .mouse_moved()
    .cursor_update()
```

#### Precise Mouse Tracking
```rust
TrackingAreaOptions::new()
    .active_in_active_app()
    .mouse_moved()
    .enable_mouse_moved_events()
```

## Type Conversion

All builder types implement `Into` traits for seamless conversion:

```rust
// These are equivalent
panel.set_level(PanelLevel::Floating);
panel.set_level(PanelLevel::Floating.into());
panel.set_level(3i32);  // Raw NSWindowLevel value

// These are equivalent  
let style = StyleMask::empty().titled();
panel.set_style_mask(style);
panel.set_style_mask(style.into());
```

## Next Steps

- [Learn about Panel Methods](panel-methods.md)
- [Explore Event Handling](event-handling.md)
- [Check out Complete Examples](examples.md)