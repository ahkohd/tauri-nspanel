# PanelBuilder API

The `PanelBuilder` provides a flexible way to create panels with your custom panel classes.

## Basic usage

```rust
use tauri::Manager;
use tauri_nspanel::{PanelBuilder, PanelLevel, WebviewUrl};

let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "my-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .title("My Panel")
    .level(PanelLevel::Floating)
    .build()?;
```

## Configuration methods

### Content and appearance
```rust
let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "styled-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .title("Styled Panel")
    .corner_radius(12.0)        // Rounded corners
    .transparent(true)          // Transparent background
    .alpha_value(0.95)          // Semi-transparent
    .has_shadow(true)           // Drop shadow
    .build()?;
```

### Window positioning and size
```rust
use tauri::{Manager, LogicalPosition, LogicalSize, Position, Size};

let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "positioned-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .position(Position::Logical(LogicalPosition::new(100.0, 100.0)))
    .size(Size::Logical(LogicalSize::new(400.0, 300.0)))
    .build()?;
```

### Panel behavior
```rust
let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "behavior-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .level(PanelLevel::Floating)
    .floating(true)
    .hides_on_deactivate(false)
    .works_when_modal(true)
    .accepts_mouse_moved_events(true)
    .build()?;
```

### Style masks
```rust
use tauri_nspanel::StyleMask;

let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "styled-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .style_mask(
        StyleMask::empty()
            .nonactivating_panel()  // Doesn't activate app
            .utility_window()       // Smaller title bar
            .titled()
            .closable()
    )
    .build()?;
```

### Collection behavior
```rust
use tauri_nspanel::CollectionBehavior;

let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "spaces-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .collection_behavior(
        CollectionBehavior::new()
            .can_join_all_spaces()  // Show on all spaces
            .stationary()           // Don't move between spaces
            .ignores_cycle()        // Skip in Cmd+Tab
    )
    .build()?;
```

### Advanced window configuration
```rust
let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "advanced-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .with_window(|window| {
        // Access any Tauri window configuration
        window
            .decorations(false)
            .min_inner_size(300.0, 200.0)
            .max_inner_size(800.0, 600.0)
            .resizable(false)
            .always_on_top(true)
    })
    .build()?;
```

### No activation
```rust
let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), "background-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .no_activate(true)  // Prevent panel from stealing focus when created
    .build()?;
```

## Complete example

```rust
use tauri::{Manager, LogicalPosition, LogicalSize, Position, Size};
use tauri_nspanel::{
    CollectionBehavior, PanelBuilder, PanelLevel, StyleMask, WebviewUrl
};

let panel = PanelBuilder::<_, AdvancedPanel>::new(app.handle(), "complete-panel")
    .url(WebviewUrl::App("panel.html".into()))
    .title("Complete Panel")
    .position(Position::Logical(LogicalPosition::new(100.0, 100.0)))
    .size(Size::Logical(LogicalSize::new(400.0, 300.0)))
    .level(PanelLevel::Floating)
    .corner_radius(12.0)
    .transparent(true)
    .alpha_value(0.95)
    .has_shadow(true)
    .style_mask(
        StyleMask::empty()
            .nonactivating_panel()
            .utility_window()
            .titled()
            .closable()
    )
    .collection_behavior(
        CollectionBehavior::new()
            .can_join_all_spaces()
            .stationary()
    )
    .with_window(|w| w.decorations(false))
    .no_activate(true)
    .build()?;

panel.show_and_make_key();
```

## Method reference

### Essential methods
- `new(app_handle, label)` - Create new builder
- `url(WebviewUrl)` - Set panel content URL
- `build()` - Build the panel

### Appearance
- `title(String)` - Set window title
- `corner_radius(f64)` - Set rounded corners
- `transparent(bool)` - Set transparent background
- `alpha_value(f64)` - Set opacity (0.0-1.0)
- `has_shadow(bool)` - Enable/disable drop shadow
- `opaque(bool)` - Set window opacity

### Positioning
- `position(Position)` - Set initial position
- `size(Size)` - Set initial size
- `content_size(Size)` - Set content area size

### Behavior
- `level(PanelLevel)` - Set window level
- `floating(bool)` - Set floating behavior
- `hides_on_deactivate(bool)` - Hide when app deactivates
- `works_when_modal(bool)` - Work with modal dialogs
- `no_activate(bool)` - Prevent focus stealing

### Style and collection
- `style_mask(StyleMask)` - Set window style
- `collection_behavior(CollectionBehavior)` - Set collection behavior

### Advanced
- `with_window(closure)` - Access Tauri window builder

## Next steps

- [Learn about Panel Events](event-handling.md)
- [Explore Panel Methods](panel-methods.md)
- [Check out Complete Examples](examples.md)
