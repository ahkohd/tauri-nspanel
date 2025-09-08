# Thread Safety

Panel types implement `Send` and `Sync` for Tauri compatibility. All actual panel operations are automatically performed on the main thread.

## How it works

The tauri-nspanel library ensures thread safety by:

1. **Trait Implementation**: All panel types implement `Send` and `Sync` traits, making them safe to pass between threads
2. **Main Thread Execution**: Despite being thread-safe, all NSPanel operations are automatically dispatched to the main thread
3. **Tauri Integration**: This design allows seamless integration with Tauri's multi-threaded architecture

## Usage implications

You can safely:
- Pass panels between threads
- Store panels in static variables or global state
- Use panels from async contexts and background tasks
- Call panel methods from any thread

The library handles the thread synchronization internally, so you don't need to worry about main thread dispatch or thread safety when using panel APIs.

## Example

```rust
use tauri_nspanel::ManagerExt;
use std::sync::Arc;
use tokio::task;

#[tauri::command]
async fn background_panel_operation(app: tauri::AppHandle) {
    // Safe to use from async context
    let panel = app.get_webview_panel("my-panel").unwrap();
    
    // Safe to move into async task
    let panel_clone = Arc::clone(&panel);
    task::spawn(async move {
        // Panel operations automatically dispatch to main thread
        panel_clone.show();
    });
}
```

This approach provides the safety and convenience of thread-safe APIs while maintaining the requirements of macOS's Cocoa framework.