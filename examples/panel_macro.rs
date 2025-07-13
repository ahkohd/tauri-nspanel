use tauri_nspanel::{tauri_panel, TrackingAreaOptions};

tauri_panel! {
    // Define a basic panel
    panel!(MyPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            becomeKeyIfOnlyNeeded: true,
            isFloatingPanel: true
        }
    })
    // Define a panel with tracking area
    panel!(TrackingPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false
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
