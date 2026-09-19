# Installation

## Requirements

- Tauri v2.0+
- macOS 10.15+
- Rust 1.75+

## Adding the plugin

Add the plugin to your `Cargo.toml`:

```toml
[dependencies]
tauri-nspanel = "2.1"
```

You can also install directly from the maintained `v2.1` branch. This follows the latest commit on
the branch rather than a published crates.io release:

```toml
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel", branch = "v2.1" }
```

For transparent panels and advanced styling, also add:

```toml
tauri = {version = "2.8.5", features = ["macos-private-api"] }
```

## Register the plugin

In your `src-tauri/src/main.rs`:

```rust
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

That's it! You're ready to create panels.
