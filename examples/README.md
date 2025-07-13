# Examples

These examples demonstrate how to use [tauri-nspanel](https://github.com/ahkohd/tauri-nspanel) with the new v2 API:

- [vanilla](./vanilla/): Basic panel setup using the PanelBuilder API
- [fullscreen](./fullscreen/): Create a panel that can display over fullscreen windows
- [panel_builder](./panel_builder.rs): Complete example showing custom panel classes, delegates, and both creation methods

## Running the examples

To run an example:

```bash
cd examples/vanilla
cargo tauri dev
```

For the standalone Rust example:

```bash
cargo run --example panel_builder
```