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
    panel!(MouseTrackingPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            isFloatingPanel: true
        }
        with: {
            // Enable mouse tracking for the panel
            tracking_area: {
                options: TrackingAreaOptions::new()
                    .active_always()           // Track mouse even when app is not active
                    .mouse_entered_and_exited() // Get notified when mouse enters/exits
                    .mouse_moved()             // Track mouse movement
                    .cursor_update(),          // Track cursor updates
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

  let panel = window.to_panel::<MouseTrackingPanel>().unwrap();

  let handler = MyPanelEventHandler::new();

  // Set up mouse event callbacks on the handler
  handler.on_mouse_entered(|_event| {
    println!("🐭 Mouse entered the panel!");
    // In a real app, you could emit a Tauri event here to notify the frontend
  });

  handler.on_mouse_exited(|_event| {
    println!("👋 Mouse exited the panel!");
  });

  handler.on_mouse_moved(|event| {
    // Get the mouse location relative to the window
    let location = unsafe { event.locationInWindow() };
    println!("🏃 Mouse moved to: x={}, y={}", location.x, location.y);
  });

  handler.on_cursor_update(|_event| {
    println!("👆 Cursor update requested");
    // Here you could change the cursor based on what the mouse is hovering over
  });

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

  println!("Mouse tracking panel initialized! Move your mouse over the panel to see events.");
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

  panel.close();
}
