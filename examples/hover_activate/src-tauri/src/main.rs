#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager};
use tauri_nspanel::{
  tauri_panel, CollectionBehavior, ManagerExt, PanelLevel, StyleMask, TrackingAreaOptions,
  WebviewWindowExt,
};

tauri_panel! {
    panel!(HoverActivatePanel {
        config: {
            canBecomeMainWindow: false,
            canBecomeKeyWindow: true,
            becomesKeyOnlyIfNeeded: true,
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

    panel_event!(MyPanelEventHandler {})
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_nspanel::init())
    .invoke_handler(tauri::generate_handler![
      show_panel,
      hide_panel,
      close_panel,
    ])
    .setup(|app| {
      // Set activation policy to Accessory to prevent the app icon from showing on the dock
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);

      init(&app.app_handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init(app_handle: &AppHandle) {
  let window = app_handle.get_webview_window("main").unwrap();

  let panel = window.to_panel::<HoverActivatePanel>().unwrap();

  let handler = MyPanelEventHandler::new();

  // Set up mouse event callbacks on the handler
  let handle = app_handle.clone();

  handler.on_mouse_entered(move |_event| {
    println!("🐭 Mouse entered the panel, make it key!");

    let panel = handle.get_webview_panel("main").unwrap();

    panel.make_key_window();
  });

  let handle = app_handle.to_owned();

  handler.on_mouse_exited(move |_event| {
    println!("👋 Mouse exited the panel!");

    let panel = handle.get_webview_panel("main").unwrap();

    panel.resign_key_window();
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

  // Sets the panel to be resizable
  panel.set_hides_on_deactivate(false);

  // Receive keyboard and mouse events even
  // when another window in the application is running modally
  panel.set_works_when_modal(true);

  panel.set_event_handler(Some(handler.as_protocol_object()));
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
