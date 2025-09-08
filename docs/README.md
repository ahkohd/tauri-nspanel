# Documentation

Complete documentation for creating macOS panels in your Tauri applications.

## Getting started

- [Installation](installation.md): Setup requirements and plugin registration
- [Getting Started](getting-started.md): Quick start guide and basic concepts

## Core concepts

- [Panel Classes](panel-classes.md): Define custom panel behavior with `tauri_panel!` macro
- [PanelBuilder API](panel-builder.md): Create panels with fluent builder pattern
- [Event Handling](event-handling.md): Handle window events and mouse tracking
- [Panel Methods](panel-methods.md): Control panel appearance and behavior

## Reference

- [Key Types](key-types.md) 
- [Thread Safety](thread-safety.md) 
- [Examples](examples.md) 
[API Reference](https://docs.aremu.dev/tauri-nspanel/)

## Quick navigation

### Basic usage
1. [Install and setup](installation.md) the plugin
2. [Define a panel class](panel-classes.md) with custom behavior
3. [Create the panel](panel-builder.md) using PanelBuilder or convert existing window
4. [Handle events](event-handling.md) if needed
5. [Control the panel](panel-methods.md) with available methods

### Common tasks

- Create a basic floating panel: [Getting Started](getting-started.md)
- Handle window events: [Event Handling](event-handling.md)
- Set up mouse tracking: [Event Handling](event-handling.md#mouse-tracking-events)
- Configure panel appearance: [PanelBuilder API](panel-builder.md#appearance)
- Control window levels: [Key Types](key-types.md#panellevel)
- Style window appearance: [Key Types](key-types.md#stylemask)
- Work with Spaces and fullscreen: [Key Types](key-types.md#collectionbehavior)

### Examples by use case

- Tool palette: [Panel Classes](panel-classes.md#tool-palette)
- Inspector panel: [Panel Classes](panel-classes.md#inspector-panel) 
- HUD display: [Panel Classes](panel-classes.md#hud-display)
- Interactive panel with mouse tracking: [Event Handling](event-handling.md#mouse-tracking-events)
- Floating panel that ignores Cmd+Tab: [Key Types](key-types.md#hud-display)

## API overview

### Main types
- `Panel<R>` - Core trait implemented by all panels
- `PanelBuilder<R, P>` - Builder for creating panels
- `ManagerExt<R>` - Extension trait for accessing panels from AppHandle

### Key enums
- `PanelLevel` - Window layering levels
- `StyleMask` - Window appearance styles  
- `CollectionBehavior` - Spaces and fullscreen behavior
- `TrackingAreaOptions` - Mouse tracking configuration

### Macros
- `tauri_panel!` - Define panel classes and event handlers
- `panel!` - Define individual panel class
- `panel_event!` - Define event handler delegate

## Related resources

- [Tauri Documentation](https://v2.tauri.app/)
- [NSPanel Documentation](https://developer.apple.com/documentation/appkit/nspanel)
- [NSWindowDelegate Documentation](https://developer.apple.com/documentation/appkit/nswindowdelegate)
- [Examples Repository](../examples/)
- [API Reference](https://docs.aremu.dev/tauri-nspanel/)
