# Examples

These examples demonstrate various features and use cases of [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel):

## Full Tauri Applications

- [basic](./basic/): Basic panel setup in a vanilla JavaScript Tauri app
- [panel_builder](./panel_builder/): Basic panel setup using `PanelBuilder`
- [fullscreen](./fullscreen/): Panel that displays over fullscreen windows
- [mouse_tracking](./mouse_tracking/): Mouse tracking events with enter/exit/move callbacks
- [hover_activate](./hover_activate/): Auto-activate panel on mouse hover
- [`spotlight-app`](https://github.com/ahkohd/tauri-macos-spotlight-example) - An example macOS Spotlight app built with Tauri
- [`menubar-app`](https://github.com/ahkohd/tauri-macos-menubar-app-example) - An example macOS Menubar app built with Tauri 

## Standalone Rust Examples

- [panel_macro](./panel_macro.rs): Basic panel creation with the macro
- [panel_builder](./panel_builder.rs): Using the PanelBuilder API
- [panel_levels](./panel_levels.rs): Demonstrating different window levels
- [panel_style_mask](./panel_style_mask.rs): Different NSWindowStyleMask configurations for panels
- [collection_behavior](./collection_behavior.rs): Combining collection behaviors
- [builder_with_custom_panel](./builder_with_custom_panel.rs): Using custom panel classes with PanelBuilder
- [panel_event_macro](./panel_event_macro.rs): Event handling with delegates

## Running the Examples

### Full Tauri applications

```bash
# For any of the full Tauri app examples:
cd examples/mouse_tracking  # or basic, panel_builder, fullscreen, hover_activate
npm install
npm run tauri dev
```

### Standalone Rust examples

```bash
# From the root directory:
cargo run --example panel_macro
cargo run --example panel_builder
cargo run --example panel_levels
# etc.
```
