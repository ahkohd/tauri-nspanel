use tauri::{AppHandle, Manager, WebviewUrl};
use tauri_nspanel::{tauri_panel, CollectionBehavior, PanelBuilder};

// Define a demo panel class
tauri_panel! {
    panel!(DemoPanel {
        config: {
            can_become_key_window: true
        }
    })
}

/// Example demonstrating collection behavior combinations
#[allow(dead_code)]
fn create_panels_with_behaviors(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Panel that appears on all spaces
    let all_spaces_panel = PanelBuilder::<_, DemoPanel>::new(app, "all-spaces")
        .url(WebviewUrl::App("index.html".into()))
        .title("All Spaces Panel")
        .collection_behavior(CollectionBehavior::new().can_join_all_spaces())
        .build()?;

    // Panel that doesn't participate in Cmd+Tab cycling
    let no_cycle_panel = PanelBuilder::<_, DemoPanel>::new(app, "no-cycle")
        .url(WebviewUrl::App("index.html".into()))
        .title("No Cycle Panel")
        .collection_behavior(CollectionBehavior::new().ignores_cycle())
        .build()?;

    // Utility panel that appears on all spaces and ignores cycling
    let utility_panel = PanelBuilder::<_, DemoPanel>::new(app, "utility")
        .url(WebviewUrl::App("index.html".into()))
        .title("Utility Panel")
        .collection_behavior(
            CollectionBehavior::new()
                .can_join_all_spaces()
                .ignores_cycle()
                .stationary(),
        )
        .build()?;

    // Panel that works with fullscreen apps
    let fullscreen_auxiliary = PanelBuilder::<_, DemoPanel>::new(app, "fullscreen-aux")
        .url(WebviewUrl::App("index.html".into()))
        .title("Fullscreen Auxiliary")
        .collection_behavior(
            CollectionBehavior::new()
                .full_screen_auxiliary()
                .can_join_all_spaces(),
        )
        .build()?;

    // Transient panel (for temporary UI)
    let transient_panel = PanelBuilder::<_, DemoPanel>::new(app, "transient")
        .url(WebviewUrl::App("index.html".into()))
        .title("Transient Panel")
        .collection_behavior(CollectionBehavior::new().transient())
        .build()?;

    all_spaces_panel.show();
    no_cycle_panel.show();
    utility_panel.show();
    fullscreen_auxiliary.show();
    transient_panel.show();

    println!("Created panels with different collection behaviors:");
    println!("- All Spaces: Visible on all desktop spaces");
    println!("- No Cycle: Skipped in Cmd+Tab");
    println!("- Utility: Combines multiple behaviors");
    println!("- Fullscreen Auxiliary: Works alongside fullscreen apps");
    println!("- Transient: For temporary UI elements");

    Ok(())
}

fn main() {
    println!("This example demonstrates different collection behaviors.");
    println!("To run this in a real app, use the create_panels_with_behaviors function in your Tauri setup.");
}
