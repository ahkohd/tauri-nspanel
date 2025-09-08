#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, LogicalPosition, LogicalSize, Manager, Position, Size, WebviewUrl};
use tauri_nspanel::{
  tauri_panel, CollectionBehavior, ManagerExt, PanelBuilder, PanelLevel, StyleMask,
};

// Define custom panel class and event handler
tauri_panel! {
    panel!(MiniPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            isFloatingPanel: true
        }
    })

    panel_event!(MiniPanelEventHandler {
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
  // Create a mini panel using PanelBuilder
  let panel = PanelBuilder::<_, MiniPanel>::new(app_handle, "mini-panel")
    .url(WebviewUrl::App("mini.html".into()))
    .title("Mini Panel")
    .position(Position::Logical(LogicalPosition::<f64> {
      x: 100.0,
      y: 100.0,
    }))
    .size(Size::Logical(LogicalSize::<f64> {
      width: 350.0,
      height: 350.0,
    }))
    .level(PanelLevel::Floating)
    .has_shadow(true)
    .collection_behavior(CollectionBehavior::new().can_join_all_spaces().stationary())
    .hides_on_deactivate(false)
    .works_when_modal(true)
    .with_window(|w| w.decorations(false))
    .style_mask(StyleMask::empty().nonactivating_panel().resizable())
    // Prevent the panel from stealing focus when created (works especially well with Accessory policy)
    .no_activate(true)
    .build()
    .expect("Failed to create mini panel");

  // Print panel info
  println!("Panel created with PanelBuilder!");
  println!("Panel class name: {:?}", panel.as_panel().class().name());
  println!("Panel can become key?: {}", panel.can_become_key_window());
  println!("Panel can become main?: {}", panel.can_become_main_window());
  println!("Panel is floating?: {}", panel.is_floating_panel());

  // Create and attach event handler
  let handler = MiniPanelEventHandler::new();

  let handle = app_handle.to_owned();

  handler.window_did_become_key(move |notification| {
    let app_name = handle.package_info().name.to_owned();

    unsafe { println!("[info]: Notification name: {:?}", notification.name()) };
    println!("[info]: {:?} mini panel becomes key window!", app_name);
  });

  handler.window_did_resign_key(|_notification| {
    println!("[info]: mini panel resigned from key window!");
  });

  panel.set_event_handler(Some(handler.as_protocol_object()));

  // Show the panel
  panel.show_and_make_key();
}

#[tauri::command]
fn show_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel("mini-panel").unwrap();
  panel.show_and_make_key();
}

#[tauri::command]
fn hide_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel("mini-panel").unwrap();
  panel.hide();
}

#[tauri::command]
fn close_panel(app_handle: AppHandle) {
    app_handle
        .get_webview_panel("mini-panel")
        .ok()
        .and_then(|panel| panel.to_window())
        .map(|window| window.close());
}
