#![allow(dead_code, clippy::unused_unit)]

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tauri::{AppHandle, Manager, State};
use tauri_nspanel::{tauri_panel, ManagerExt};

const PANEL_LABEL: &str = "main";

#[derive(Clone, Default)]
struct PanelPinState(Arc<AtomicBool>);

tauri_panel! {
    panel_event!(PinPanelEventHandler {
        window_did_resign_key(notification: &NSNotification) -> ()
    })
}

/// Install the event handler once while setting up the panel.
fn configure_pin_behavior(app: &AppHandle) {
    let pin_state = PanelPinState::default();
    app.manage(pin_state.clone());

    let panel = app.get_webview_panel(PANEL_LABEL).unwrap();
    let weak_panel = Arc::downgrade(&panel);
    let handler = PinPanelEventHandler::new();

    handler.window_did_resign_key(move |_| {
        if !pin_state.0.load(Ordering::Relaxed) {
            if let Some(panel) = weak_panel.upgrade() {
                panel.hide();
            }
        }
    });

    // The panel retains the handler until it is replaced or cleared.
    panel.set_event_handler(Some(handler.as_ref()));
}

#[tauri::command]
fn pin_panel(state: State<'_, PanelPinState>) {
    state.0.store(true, Ordering::Relaxed);
}

#[tauri::command]
fn unpin_panel(state: State<'_, PanelPinState>) {
    state.0.store(false, Ordering::Relaxed);
}

fn main() {
    println!("Configure pin behavior during Tauri setup and register the pin/unpin commands.");
}
