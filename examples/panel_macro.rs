use tauri::Manager;
use tauri_nspanel::{tauri_panel, TrackingAreaOptions};

tauri_panel! {
    // Define a basic panel
    panel!(MyPanel {
        config: {
            can_become_key_window: true,
            can_become_main_window: false,
            become_key_if_only_needed: true,
            is_floating_panel: true
        }
    })
    // Define a panel with tracking area
    panel!(TrackingPanel {
        config: {
            can_become_key_window: true,
            can_become_main_window: false
        }
        with: {
            tracking_area: {
                options: TrackingAreaOptions::new()
                    .active_always()
                    .mouse_entered_and_exited()
                    .mouse_moved(),
                auto_resize: true
            }
        }
    })
}

fn main() {
    println!("Panel classes created!");
    println!("- MyPanel: Basic panel with floating behavior");
    println!("- TrackingPanel: Panel with mouse tracking area");
}
