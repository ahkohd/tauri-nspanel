use tauri::{AppHandle, Runtime, WebviewUrl};
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, PanelBuilder, PanelLevel, TrackingAreaOptions,
};

// Define all custom panel classes in a single tauri_panel! block
tauri_panel! {
    panel!(MyCustomPanel {
        config: {
            canBecomeKeyWindow: true,
            canBecomeMainWindow: false,
            becomesKeyOnlyIfNeeded: true
        }
    })

    panel!(NotificationPanel {
        config: {
            canBecomeKeyWindow: false,
            canBecomeMainWindow: false,
            isFloatingPanel: true
        }
    })

    panel!(InteractivePanel {
        config: {
            canBecomeKeyWindow: true
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

#[allow(dead_code)]
fn create_custom_panels<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Create a panel using MyCustomPanel class with additional builder configuration
    let custom_panel = PanelBuilder::<R, MyCustomPanel>::new(app, "custom-panel")
        .url(WebviewUrl::App("index.html".into()))
        .title("My Custom Panel")
        .level(PanelLevel::Floating)
        .collection_behavior(CollectionBehavior::new().can_join_all_spaces().stationary())
        .size(tauri::Size::Logical(tauri::LogicalSize::new(400.0, 300.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            100.0, 100.0,
        )))
        .alpha_value(0.98)
        .build()?;

    // Create a notification-style panel
    let notification = PanelBuilder::<R, NotificationPanel>::new(app, "notification")
        .url(WebviewUrl::App("notification.html".into()))
        .title("Notification")
        .level(PanelLevel::Status) // High level for notifications
        .collection_behavior(CollectionBehavior::new().ignores_cycle().transient())
        .size(tauri::Size::Logical(tauri::LogicalSize::new(350.0, 100.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            50.0, 50.0,
        )))
        .alpha_value(0.95)
        .has_shadow(true)
        .hides_on_deactivate(false)
        .build()?;

    // Create an interactive panel with tracking already configured
    let interactive = PanelBuilder::<R, InteractivePanel>::new(app, "interactive")
        .url(WebviewUrl::App("interactive.html".into()))
        .title("Interactive Panel")
        .level(PanelLevel::Utility)
        .size(tauri::Size::Logical(tauri::LogicalSize::new(500.0, 400.0)))
        .with_window(|window| {
            window
                .min_inner_size(300.0, 200.0)
                .max_inner_size(800.0, 600.0)
                .resizable(true)
        })
        .build()?;

    // The panels have their custom configurations from the macro
    // plus the additional configurations from the builder
    custom_panel.show();
    notification.show();
    interactive.show();

    println!("Created custom panels:");
    println!("- MyCustomPanel: Becomes key only if needed");
    println!("- NotificationPanel: Floating, non-key panel");
    println!("- InteractivePanel: Has built-in mouse tracking");

    Ok(())
}

fn main() {
    println!("This example demonstrates using custom panel classes with PanelBuilder.");
    println!(
        "To run this in a real app, use the create_custom_panels function in your Tauri setup."
    );
}
