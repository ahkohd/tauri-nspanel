# Thread Safety

Panel wrappers implement `Send` and `Sync` for compatibility with Tauri's command and state types. These trait implementations do **not** make AppKit objects thread-safe and do not dispatch calls to the main thread.

## Main-thread requirement

`NSPanel` and its related AppKit APIs must be used on the main thread. Panel methods call AppKit directly; callers must arrange for every operation to run on the main thread. Creating event handlers also requires the main thread. The `Send`/`Sync` implementations are an interoperability boundary, not permission to call panel APIs from arbitrary threads.

For example, a Tauri command or async task may run outside the main thread. Do not call a panel method there unless that code is already executing on the main thread. Use Tauri's main-thread dispatch mechanism to schedule the operation, and pass only the data needed by that closure.

## Usage implications

- You may hold or pass a panel wrapper through APIs that require `Send` or `Sync`, but you must still respect AppKit's main-thread requirement when accessing it.
- Do not call panel methods from a background thread or assume that async context is safe.
- Keep event-handler creation, configuration, and panel operations on the main thread.

The library does not provide automatic main-thread dispatch or synchronization for panel APIs.

See [Tauri's `AppHandle::run_on_main_thread` API](https://docs.rs/tauri/latest/tauri/struct.AppHandle.html#method.run_on_main_thread) when an operation must be scheduled from another thread. Ensure any panel handle captured by the closure remains valid for the operation.
