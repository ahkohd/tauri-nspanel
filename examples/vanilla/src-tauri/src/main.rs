#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, Window, Wry};
use tauri_nspanel::{objc_id::Id, panel_delegate, ManagerExt, Panel, WindowExt};

fn main() {
  tauri::Builder::default()
    .plugin(tauri_nspanel::init())
    .invoke_handler(tauri::generate_handler![
      show_panel,
      hide_panel,
      close_panel
    ])
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

  fn window_did_become_key(panel: Panel<Wry>) {
    let app_name = panel.app_handle().unwrap().package_info().name.to_owned();
    println!("[info]: {:?} panel becomes key window!", app_name);
  }

  fn window_did_resign_key(_: Panel<Wry>) {
    println!("[info]: panel resigned from key window!");
  }

  let delegate = panel_delegate!(MyPanelDelegate<Wry> {
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

#[tauri::command]
fn close_panel(handle: AppHandle<Wry>) {
  let panel = handle.get_panel("main").unwrap();
  panel.released_when_closed(true);
  panel.close();
}
