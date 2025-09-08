use tauri::{AppHandle, Manager};
use tauri_nspanel::{tauri_panel, PanelBuilder, PanelLevel};

// Define a simple panel class
tauri_panel! {
    panel!(DemoPanel {
        config: {
            canBecomeKeyWindow: true
        }
    })
}

/// Example demonstrating different panel levels
#[allow(dead_code)]
fn create_panels_with_levels(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Normal level panel (behind other panels)
    let normal_panel = PanelBuilder::<_, DemoPanel>::new(app, "normal-panel")
        .title("Normal Level Panel")
        .level(PanelLevel::Normal)
        .size(tauri::Size::Logical(tauri::LogicalSize::new(300.0, 200.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            100.0, 100.0,
        )))
        .build()?;

    // Floating level panel (above normal windows)
    let floating_panel = PanelBuilder::<_, DemoPanel>::new(app, "floating-panel")
        .title("Floating Panel")
        .level(PanelLevel::Floating)
        .size(tauri::Size::Logical(tauri::LogicalSize::new(300.0, 200.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            150.0, 150.0,
        )))
        .build()?;

    // Status level panel (above floating panels)
    let status_panel = PanelBuilder::<_, DemoPanel>::new(app, "status-panel")
        .title("Status Panel")
        .level(PanelLevel::Status)
        .size(tauri::Size::Logical(tauri::LogicalSize::new(300.0, 200.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            200.0, 200.0,
        )))
        .build()?;

    // Custom level panel
    let custom_panel = PanelBuilder::<_, DemoPanel>::new(app, "custom-panel")
        .title("Custom Level Panel")
        .level(PanelLevel::Custom(100))
        .size(tauri::Size::Logical(tauri::LogicalSize::new(300.0, 200.0)))
        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
            250.0, 250.0,
        )))
        .build()?;

    // Show all panels to demonstrate layering
    normal_panel.show();
    floating_panel.show();
    status_panel.show();
    custom_panel.show();

    println!("Created panels with different levels:");
    println!("- Normal (0): Behind other panels");
    println!("- Floating (3): Above normal windows");
    println!("- Status (25): Above floating panels");
    println!("- Custom (100): Very high level");

    Ok(())
}

fn main() {
    println!("This example demonstrates different panel levels.");
    println!("To run this in a real app, use the create_panels_with_levels function in your Tauri setup.");
}
