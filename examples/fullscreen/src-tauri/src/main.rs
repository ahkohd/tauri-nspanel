#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_nspanel::{
  tauri_panel, CollectionBehavior, ManagerExt, PanelLevel, StyleMask, TrackingAreaOptions,
  WebviewWindowExt,
};

tauri_panel! {
    panel!(BasicPanel {
        config: {
            canBecomeKeyWindow: true,
            isFloatingPanel: true
        }
        with: {
            // Enable mouse tracking for the panel's content view
            // This allows the panel to receive mouse events even when not key/active
            tracking_area: {
                options: TrackingAreaOptions::new()
                    .active_always()           // Track mouse even when app is not active
                    .mouse_entered_and_exited() // Get notified when mouse enters/exits
                    .mouse_moved(),             // Track mouse movement
                auto_resize: true               // Resize tracking area with window
            }
        }
    })

    panel_event!(MyPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowDidResignKey(notification: &NSNotification) -> ()
    })
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_nspanel::init())
    .invoke_handler(tauri::generate_handler![
      show_panel,
      hide_panel,
      close_panel
    ])
    .setup(|app| {
      // Set activation policy to Accessory to prevent the app icon from showing on the dock
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      init(app.app_handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init(app_handle: &AppHandle) {
  let window: WebviewWindow = app_handle.get_webview_window("main").unwrap();

  let panel = window.to_panel::<BasicPanel>().unwrap();

  let handler = MyPanelEventHandler::new();

  let handle = app_handle.to_owned();

  handler.window_did_become_key(move |_notification| {
    let app_name = handle.package_info().name.to_owned();
    println!("[info]: {:?} panel becomes key window!", app_name);
  });

  handler.window_did_resign_key(|_notification| {
    println!("[info]: panel resigned from key window!");
  });

  // Set the window to float level
  panel.set_level(PanelLevel::Floating.value());

  // Ensures the panel cannot activate the app
  panel.set_style_mask(StyleMask::empty().nonactivating_panel().into());

  // Allows the panel to:
  // - display on the same space as the full screen window
  // - join all spaces
  panel.set_collection_behavior(
    CollectionBehavior::new()
      .full_screen_auxiliary()
      .can_join_all_spaces()
      .into(),
  );

  panel.set_event_handler(Some(handler.as_protocol_object()));

  // Note: The tracking area is configured in the panel definition above.
  // Mouse events (mouseEntered, mouseExited, mouseMoved) will be sent to the
  // panel's content view. To handle these events, you would need to:
  // 1. Create a custom NSView subclass that overrides these methods
  // 2. Use JavaScript in your webview to listen for mouse events
  // 3. Or use Tauri's event system to communicate mouse positions
}

#[tauri::command]
fn show_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel("main").unwrap();

  panel.show();
}

#[tauri::command]
fn hide_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel("main").unwrap();

  panel.hide();
}

#[tauri::command]
fn close_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel("main").unwrap();
  panel.set_released_when_closed(true);
  // release the event handler if any
  panel.set_event_handler(None);
  panel.close();
}
