#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, Window, Wry};
use tauri_nspanel::{objc_id::Id, panel_delegate, ManagerExt, Panel, WindowExt};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_nspanel::init())
    .invoke_handler(tauri::generate_handler![show_panel, hide_panel])
    .setup(|app| {
      let window = app.handle().get_window("main").unwrap();
      init(window);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn init(window: Window<Wry>) {
  let panel = window.to_panel().unwrap();

  fn window_did_become_key(_: Panel) {
    println!("[info]: panel becomes key window!");
  }

  fn window_did_resign_key(_: Panel) {
    println!("[info]: panel resigned from key window!");
  }

  let delegate = panel_delegate!(MyPanelDelegate {
    window_did_become_key,
    window_did_resign_key
  });

  delegate.set_panel(panel.to_owned());
  panel.set_delegate(delegate);
}

#[tauri::command]
fn show_panel(handle: AppHandle<Wry>) {
  let panel = handle.get_panel("main").unwrap();
  panel.show();
}

#[tauri::command]
fn hide_panel(handle: AppHandle<Wry>) {
  let panel = handle.get_panel("main").unwrap();
  panel.order_out(None);
}
