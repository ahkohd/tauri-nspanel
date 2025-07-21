// Example: Panel Style Masks
// This example demonstrates different NSWindowStyleMask configurations for panels

use tauri_nspanel::{tauri_panel, StyleMask};

tauri_panel! {
    // Default panel with standard window chrome
    panel!(StandardPanel {
        config: {
            canBecomeKeyWindow: true
        }
    })

    // Borderless panel (no title bar)
    panel!(BorderlessPanel {
        config: {
            canBecomeKeyWindow: true,
            isFloatingPanel: true
        }
    })

    // HUD-style panel
    panel!(HUDPanel {
        config: {
            canBecomeKeyWindow: false,
            isFloatingPanel: true
        }
    })

    // Utility panel with small title bar
    panel!(UtilityPanel {
        config: {
            canBecomeKeyWindow: true,
            becomesKeyOnlyIfNeeded: true
        }
    })

    // Non-activating panel
    panel!(NonActivatingPanel {
        config: {
            canBecomeKeyWindow: false,
            isFloatingPanel: true
        }
    })
}

fn main() {
    println!("Panel Style Mask Examples:");
    println!("========================");

    // 1. Default panel with title bar and all controls
    println!("\n1. Standard Panel (default style):");
    println!("   - Has title bar with close, minimize, and maximize buttons");
    println!("   - User can resize the window");
    println!("   - Activates the application when clicked");
    let standard_style = StyleMask::new(); // Default includes: Titled, Closable, Miniaturizable, Resizable
    println!("   Style mask: {:?}", standard_style);

    // 2. Borderless panel
    println!("\n2. Borderless Panel:");
    println!("   - No title bar or window controls");
    println!("   - Cannot be resized by dragging edges");
    println!("   - Useful for splash screens or overlays");
    let borderless_style = StyleMask::empty().borderless();
    println!("   Style mask: {:?}", borderless_style);

    // 3. HUD-style panel
    println!("\n3. HUD Panel:");
    println!("   - Dark translucent appearance");
    println!("   - Small title bar");
    println!("   - Commonly used for floating controls");
    let hud_style = StyleMask::empty().hud_window().titled().closable();
    println!("   Style mask: {:?}", hud_style);

    // 4. Utility window
    println!("\n4. Utility Panel:");
    println!("   - Smaller title bar than normal windows");
    println!("   - Floats above normal windows");
    println!("   - Common for tool palettes");
    let utility_style = StyleMask::empty()
        .utility_window()
        .titled()
        .closable()
        .resizable();
    println!("   Style mask: {:?}", utility_style);

    // 5. Non-activating panel
    println!("\n5. Non-Activating Panel:");
    println!("   - Doesn't activate the app when clicked");
    println!("   - Useful for overlays that shouldn't steal focus");
    println!("   - Can still receive mouse events");
    let non_activating_style = StyleMask::empty().nonactivating_panel().titled().closable();
    println!("   Style mask: {:?}", non_activating_style);

    // 6. Full-featured panel with custom styling
    println!("\n6. Custom Full-Featured Panel:");
    println!("   - Content extends under title bar");
    println!("   - Unified title and toolbar appearance");
    println!("   - All window controls available");
    let full_featured_style = StyleMask::new()
        .full_size_content_view()
        .unified_title_and_toolbar();
    println!("   Style mask: {:?}", full_featured_style);

    // 7. Minimal floating panel
    println!("\n7. Minimal Floating Panel:");
    println!("   - Only close button, no minimize/maximize");
    println!("   - Cannot be resized");
    println!("   - Non-activating");
    let minimal_style = StyleMask::empty().nonactivating_panel().titled().closable();
    println!("   Style mask: {:?}", minimal_style);

    println!("\n========================");
    println!("Style Mask Methods:");
    println!("- titled() - Window has a title bar");
    println!("- closable() - Window has a close button");
    println!("- miniaturizable() - Window has a minimize button");
    println!("- resizable() - Window can be resized");
    println!("- unified_title_and_toolbar() - Unified appearance");
    println!("- full_size_content_view() - Content extends under title bar");
    println!("- utility_window() - Utility window style (smaller title bar)");
    println!("- hud_window() - HUD window style (dark translucent)");
    println!("- nonactivating_panel() - Panel doesn't activate the app");
    println!("- borderless() - No title bar or border");

    println!("\nUsage with PanelBuilder:");
    println!("```rust");
    println!("let panel = PanelBuilder::<_, MyPanel>::new(app.handle(), \"my-panel\")");
    println!("    .style_mask(");
    println!("        StyleMask::empty()");
    println!("            .nonactivating_panel()");
    println!("            .titled()");
    println!("            .closable()");
    println!("    )");
    println!("    .build()?;");
    println!("```");
}

