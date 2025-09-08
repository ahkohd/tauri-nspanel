#![allow(clippy::unused_unit)]

use tauri::{AppHandle, Manager, WebviewUrl};
use tauri_nspanel::{tauri_panel, PanelBuilder, WebviewWindowExt};

// Define custom panel class and event handler together
tauri_panel! {
    panel!(MyFloatingPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            isFloatingPanel: true
        }
    })

    panel_event!(MyPanelEventHandler {
        windowDidBecomeKey(notification: &NSNotification) -> (),
        windowShouldClose(window: &NSWindow) -> Bool
    })
}

#[allow(dead_code)]
fn create_panels_example(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Method 1: Using PanelBuilder (recommended)
    let panel = PanelBuilder::<_, MyFloatingPanel>::new(app, "my-panel")
        .url(WebviewUrl::App("panel.html".into()))
        .title("My Floating Panel")
        .size(tauri::Size::Logical(tauri::LogicalSize::new(400.0, 300.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            100.0, 100.0,
        )))
        .floating(true)
        .has_shadow(true)
        .alpha_value(0.95)
        .build()?;

    panel.show();
    println!("Panel created with label: {}", panel.label());

    // Method 2: Convert existing window
    let window = tauri::WebviewWindowBuilder::new(
        app,
        "another-panel",
        WebviewUrl::App("index.html".into()),
    )
    .build()?;

    let panel2 = window.to_panel::<MyFloatingPanel>()?;
    panel2.show();

    // Set up event handler
    let handler = MyPanelEventHandler::new();
    handler.window_did_become_key(|_notification| {
        println!("Panel became key window");
    });

    handler.window_should_close(|_window| {
        println!("Panel should close?");
        Bool::new(true)
    });

    // Create another panel
    let panel_with_events = PanelBuilder::<_, MyFloatingPanel>::new(app, "panel-with-events")
        .url(WebviewUrl::App("panel.html".into()))
        .title("Panel with Events")
        .size(tauri::Size::Logical(tauri::LogicalSize::new(400.0, 300.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            550.0, 100.0,
        )))
        .build()?;

    panel_with_events.show();

    // Set event handler on panels
    // Note: The event handler needs to be passed as a protocol object wrapped in Some()
    panel_with_events.set_event_handler(Some(handler.as_protocol_object()));
    panel2.set_event_handler(Some(handler.as_protocol_object()));

    Ok(())
}

fn main() {
    println!("This example demonstrates using PanelBuilder with custom panels.");
    println!(
        "To run this in a real app, use the create_panels_example function in your Tauri setup."
    );
}
