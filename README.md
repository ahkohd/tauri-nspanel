Swizzle Tauri's [`NSWindow`](https://developer.apple.com/documentation/appkit/nswindow) to [`NSPanel`](https://developer.apple.com/documentation/appkit/nspanel)

# Install

There are three general methods of installation that we can recommend.

- Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
- Pull sources directly from Github using git tags / revision hashes (most secure)
- Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the plugin by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
tauri-nspanel = { git = "https://github.com/ahkohd/tauri-nspanel" }
```

# Usage

1. First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

2. To swizzle a window's `NSWindow` to `NSPanel`, use the `to_panel()` method:

```rust
use tauri_nspanel::WindowExt;

// ...
let panel = window.to_panel().unwrap();
```

The window will be swizzled to `NSPanel`.

> Only call the `to_panel()` method once a window.

3. To access your panels, use the `app_handle.get_panel("label")`:

```rust
use tauri_nspanel::ManagerExt;

// ...

let my_panel = app_handle.get_panel("main");
```

4. To respond to panel events, such as resizing, moving, exposing, and minimizing ([See the exhaustive list](https://developer.apple.com/documentation/appkit/nswindowdelegate?language=objc)), you need to setup a `NSWindowDelegate` for your panel.

Use the `panel_delegate!()` macro to do this:

```rust
use tauri_nspanel::{objc_id::Id, panel_delegate, ManagerExt, Panel, WindowExt};

// ...

// Define your handlers
// See, https://developer.apple.com/documentation/appkit/nswindowdelegate?language=objc
// for an exhaustive list of handlers.
//
// Example: to respond to windowWillBeginSheet:
// declare the function in snake case:
// fn window_will_begin_sheet(_: Panel) { }

fn window_did_become_key(_: Panel) {
  println!("[info]: panel becomes key window!");
}

fn window_did_resign_key(_: Panel) {
  println!("[info]: panel resigned from key window!");
}

// Use the `panel_delegate!()` macro to create your custom delegate
let delegate = panel_delegate!(MyPanelDelegate {
  window_did_become_key,
  window_did_resign_key
});

// Give the delegate a reference to your panel
delegate.set_panel(panel.to_owned());

// Set your panel's delegate
panel.set_delegate(delegate);
```

# Related

The following are projects related to this plugin:

- [tauri-plugin-spotlight](https://github.com/zzzze/tauri-plugin-spotlight)
- [tauri-nspanel-example](https://github.com/ahkohd/tauri-nspanel/tree/main/examples/vanilla)
- [tauri-macos-spotlight-example](https://github.com/ahkohd/tauri-macos-spotlight-example)

# Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

# License

MIT or MIT/Apache 2.0 where applicable.
