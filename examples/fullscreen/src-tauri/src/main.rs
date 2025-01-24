#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{AppHandle, Manager, Window, Wry};
use tauri_nspanel::{panel_delegate, ManagerExt, WindowExt, cocoa::appkit::NSWindowCollectionBehavior};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![
            show_panel,
            hide_panel,
            close_panel
        ])
        .setup(|app| {
            // Set activation poicy to Accessory to prevent the app icon from showing on the dock
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.handle().get_window("main").unwrap();
            init(window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init(window: Window<Wry>) {
    let panel = window.to_panel().unwrap();

    let delegate = panel_delegate!(MyPanelDelegate {
        window_did_become_key,
        window_did_resign_key
    });

    let handle = window.app_handle();
    delegate.set_listener(Box::new(move |delegate_name: String| {
        match delegate_name.as_str() {
            "window_did_become_key" => {
                let app_name = handle.package_info().name.to_owned();
                println!("[info]: {:?} panel becomes key window!", app_name);
            }
            "window_did_resign_key" => {
                println!("[info]: panel resigned from key window!");
            }
            _ => (),
        }
    }));

    // Set the window to float level
    #[allow(non_upper_case_globals)]
    const NSFloatWindowLevel: i32 = 4;

    panel.set_level(NSFloatWindowLevel);

    #[allow(non_upper_case_globals)]
    const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
    // Ensures the panel cannot activate the app
    panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);

    // Allows the panel to:
    // - display on the same space as the full screen window
    // - join all spaces
    panel.set_collection_behaviour(
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary |
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
    );

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
    panel.set_released_when_closed(true);
    panel.close();
}
