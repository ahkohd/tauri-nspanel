#![allow(clippy::unused_unit)]

use tauri_nspanel::tauri_panel;

tauri_panel! {
    panel_event!(MyPanelEventHandler {
        window_did_become_key(notification: &NSNotification) -> (),
        window_will_return_field_editor(sender: &NSWindow, to_object: Option<&AnyObject>) -> Option<&'static AnyObject>
    })

    panel_event!(MyOtherPanelEventHandler {
        window_should_close(window: &NSWindow) -> Bool,
        window_will_resize(window: &NSWindow, to_size: &NSSize) -> NSSize
    })
}

fn main() {
    let handler1 = MyPanelEventHandler::new();
    let handler2 = MyOtherPanelEventHandler::new();

    // Simple callback - just know the event happened
    handler1.window_did_become_key(|notification| {
        println!("Window became key! Notification: {:?}", notification);
    });
    //
    // Field editor request
    handler1.window_will_return_field_editor(|sender, client| {
        println!("Field editor requested by {:?} for {:?}", sender, client);
        None // Use default field editor
    });
    //
    // Window should close
    handler2.window_should_close(|window| {
        println!("Should close window: {:?}?", window);
        Bool::new(true)
    });
    //
    // Window resize
    handler2.window_will_resize(|window, proposed_size| {
        println!("Window {:?} will resize to: {:?}", window, proposed_size);

        // Enforce minimum size
        NSSize {
            width: proposed_size.width.max(400.0),
            height: proposed_size.height.max(300.0),
        }
    });

    println!("Event handlers created and configured!");

    // Use with panel: panel.set_event_handler(Some(handler1.as_ref()));
}
