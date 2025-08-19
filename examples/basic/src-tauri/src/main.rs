#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_nspanel::{tauri_panel, ManagerExt, WebviewWindowExt};

// Define custom panel class and event handler
tauri_panel! {
    panel!(Panel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false
        }
    })

    panel_event!(PanelEventHandler {
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
      init(app.app_handle());

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init(app_handle: &AppHandle) {
  let window: WebviewWindow = app_handle.get_webview_window("main").unwrap();

  let panel = window.to_panel::<Panel>().unwrap();

  println!("panel class name: {:?}", panel.as_panel().class().name());
  println!("panel can become key?: {}", panel.can_become_key_window());
  println!("panel can become main?: {}", panel.can_become_main_window());

  let handler = PanelEventHandler::new();

  let handle = app_handle.to_owned();

  handler.window_did_become_key(move |notification| {
    let app_name = handle.package_info().name.to_owned();

    unsafe { println!("[info]: Notification name: {:?}", notification.name()) };
    println!("[info]: {:?} panel becomes key window!", app_name);
  });

  handler.window_did_resign_key(|_notification| {
    println!("[info]: panel resigned from key window!");
  });

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
  if let Ok(panel) = handle.get_webview_panel("main") {
    panel.set_released_when_closed(true);
    // release the event handler if any
    panel.set_event_handler(None);
    panel.close(&handle);
  }
}
