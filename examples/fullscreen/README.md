# Demo
[tauri-nspanel](https://github.com/ahkohd/tauri-nspanel) converts a standard Tauri [WebviewWindow](https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindow.html) ([NSWindow](https://developer.apple.com/documentation/appkit/nswindow/)) to [NSPanel](https://developer.apple.com/documentation/appkit/nspanel/) that can display over fullscreen window.

To run the demo:
```bash
pnpm install

pnpm tauri dev
```

# What you should know

## Remove Window Decorations
Configure the window by setting `decorations` and `fullscreen` to false:

[tauri-config.json](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/tauri.conf.json#L58)

```json
{
    "decorations": false,
    "fullscreen": false
}
```

## Set Activation Policy (Optional)
Set the app's activation policy to auxiliary during startup; this prevents the app icon from appearing in the dock:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/src/main.rs#L19)

```rust
    .setup(|app| {
      // Set activation poicy to Accessory to prevent the app icon from showing on the dock
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      let window = app.handle().get_window("main").unwrap();
      init(window);
      Ok(())
    })
```

## Set Window Level
Raise the panel to floating window level:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/src/main.rs#L55)

```rust
  // Set the window to float level
  #[allow(non_upper_case_globals)]
  const NSFloatWindowLevel: i32 = 4;

  panel.set_level(NSFloatWindowLevel);
```
You can set to other levels as long as it is above the normal window level, for example, set the panel above the main menu window level:
```rust
  use cocoa::appkit::NSMainMenuWindowLevel;

  // this level is recommend for a spotlight panel
  panel.set_level(NSMainMenuWindowLevel + 1);
```

## Prevent Panel From Activating The Application
It's required to prevent the panel from activating the application to display over a fullscreen window:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/src/main.rs#L60)

```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  // Ensures the panel cannot activate the app
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
```
## Set Window Collection Behaviour
To display the panel over a fullscreen window, we need to ensure it can join all spaces and be in the same space as the fullscreen window:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/src/main.rs#L65)

```rust
  // Allows the panel to:
  // - display on the same space as the full screen window
  // - join all spaces
  panel.set_collection_behaviour(
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary |
    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
  );
```

## Make The Panel Resizeable
To make the panel resizable, append the appropriate window style mask to the panel:

[main.rs](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/src/main.rs#L60)

```rust
  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  #[allow(non_upper_case_globals)]
  const NSResizableWindowMask: i32 = 1 << 3;
  
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel + NSResizableWindowMask);
```
## Add A Drag Region (Optional)
To make the panel dragable, setup a drag region:

[index.html](https://github.com/ahkohd/tauri-nspanel/blob/2357e4d5181f85b0feada118172a1cd75ea5f34d/examples/fullscreen/public/index.html#L33)

```html
<div data-tauri-drag-region>drag me</div>
```

Add the permission to allow dragging: 

[tauri.config.json](https://github.com/ahkohd/tauri-nspanel/blob/0db9134dc58b308a44fd49579e1c731810c1ef50/examples/fullscreen/src-tauri/tauri.conf.json#L46C1-L50C8)

```json
{
  "allowlist": {
   "all": false,
   "window": {
     "startDragging": true
   }
}
```
Now that the panel can be displayed over fullscreen windows, it cannot become fullscreen or be maximised. Therefore, avoid calling `{panel, window}.maximize()` or `{panel, window}.fullscreen()` on this panel, as it will result in a crash.
