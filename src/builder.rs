use std::sync::Arc;

use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
use objc2_foundation::MainThreadMarker;
use tauri::{AppHandle, Position, Runtime, Size, WebviewUrl, WebviewWindowBuilder};

use crate::{FromWindow, Panel, WebviewWindowExt};

/// Type alias for window configuration function
type WindowConfigFn<'a, R> = Box<
    dyn FnOnce(
        WebviewWindowBuilder<'a, R, AppHandle<R>>,
    ) -> WebviewWindowBuilder<'a, R, AppHandle<R>>,
>;

/// Window level constants for NSPanel
/// Based on NSWindow.Level constants from macOS
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PanelLevel {
    /// Normal window level (0)
    Normal,
    /// Submenu window level (3)
    Submenu,
    /// Torn-off menu window level (3)
    TornOffMenu,
    /// Floating window level (4)
    Floating,
    /// Modal panel window level (8)
    ModalPanel,
    /// Utility window level (19)
    Utility,
    /// Dock window level (20)
    Dock,
    /// Main menu window level (24)
    MainMenu,
    /// Status window level (25)
    Status,
    /// Pop-up menu window level (101)
    PopUpMenu,
    /// Screen saver window level (1000)
    ScreenSaver,
    /// Custom level value
    Custom(i32),
}

impl PanelLevel {
    /// Convert to the raw i64 value used by NSWindow
    pub fn value(&self) -> i64 {
        match self {
            PanelLevel::Normal => 0,
            PanelLevel::Submenu => 3,
            PanelLevel::TornOffMenu => 3,
            PanelLevel::Floating => 4,
            PanelLevel::ModalPanel => 8,
            PanelLevel::Utility => 19,
            PanelLevel::Dock => 20,
            PanelLevel::MainMenu => 24,
            PanelLevel::Status => 25,
            PanelLevel::PopUpMenu => 101,
            PanelLevel::ScreenSaver => 1000,
            PanelLevel::Custom(value) => *value as i64,
        }
    }
}

impl From<PanelLevel> for i64 {
    fn from(level: PanelLevel) -> Self {
        level.value()
    }
}

impl From<i32> for PanelLevel {
    fn from(value: i32) -> Self {
        PanelLevel::Custom(value)
    }
}

impl From<i64> for PanelLevel {
    fn from(value: i64) -> Self {
        PanelLevel::Custom(value as i32)
    }
}

/// Window collection behavior builder for NSPanel
///
/// Allows combining multiple collection behaviors using the builder pattern.
/// Collection behaviors control how a window participates in Spaces, Exposé, and fullscreen mode.
///
/// # Example
/// ```rust
/// use tauri_nspanel::{CollectionBehavior, PanelBuilder};
///
/// // Create a panel that appears on all spaces and ignores Cmd+Tab cycling
/// let behavior = CollectionBehavior::new()
///     .can_join_all_spaces()
///     .ignores_cycle();
///
/// // Use with PanelBuilder
/// PanelBuilder::new(&app, "my-panel")
///     .collection_behavior(behavior)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollectionBehavior(objc2_app_kit::NSWindowCollectionBehavior);

impl CollectionBehavior {
    /// Create an empty collection behavior
    pub fn new() -> Self {
        Self(objc2_app_kit::NSWindowCollectionBehavior::empty())
    }

    /// Window can be shown on another space
    pub fn can_join_all_spaces(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::CanJoinAllSpaces;
        self
    }

    /// Window appears in all spaces
    pub fn move_to_active_space(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::MoveToActiveSpace;
        self
    }

    /// Window is managed by Spaces
    pub fn managed(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::Managed;
        self
    }

    /// Window participates in Spaces and Expose
    pub fn transient(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::Transient;
        self
    }

    /// Window does not participate in Spaces or Expose
    pub fn stationary(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::Stationary;
        self
    }

    /// Window participates in cycling
    pub fn participates_in_cycle(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::ParticipatesInCycle;
        self
    }

    /// Window ignores cycling commands
    pub fn ignores_cycle(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::IgnoresCycle;
        self
    }

    /// Window can be shown in full screen
    pub fn full_screen_primary(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::FullScreenPrimary;
        self
    }

    /// Window can be shown alongside full screen window
    pub fn full_screen_auxiliary(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::FullScreenAuxiliary;
        self
    }

    /// Window does not allow full screen
    pub fn full_screen_none(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::FullScreenNone;
        self
    }

    /// Window can be shown in full screen for this space only
    pub fn full_screen_allows_tiling(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::FullScreenAllowsTiling;
        self
    }

    /// Window does not allow full screen and hides on app deactivation
    pub fn full_screen_disallows_tiling(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowCollectionBehavior::FullScreenDisallowsTiling;
        self
    }

    /// Create from raw NSWindowCollectionBehavior flags
    pub fn from_raw(flags: objc2_app_kit::NSWindowCollectionBehavior) -> Self {
        Self(flags)
    }

    /// Get the raw NSWindowCollectionBehavior flags
    pub fn value(&self) -> objc2_app_kit::NSWindowCollectionBehavior {
        self.0
    }
}

impl Default for CollectionBehavior {
    fn default() -> Self {
        Self::new()
    }
}

impl From<CollectionBehavior> for objc2_app_kit::NSWindowCollectionBehavior {
    fn from(behavior: CollectionBehavior) -> Self {
        behavior.0
    }
}

impl From<objc2_app_kit::NSWindowCollectionBehavior> for CollectionBehavior {
    fn from(value: objc2_app_kit::NSWindowCollectionBehavior) -> Self {
        CollectionBehavior(value)
    }
}

/// Tracking area options builder for NSPanel
///
/// Allows combining multiple tracking area options using the builder pattern.
/// Tracking areas enable mouse event tracking within a specific region of a view.
///
/// # Example
/// ```rust
/// use tauri_nspanel::{TrackingAreaOptions, PanelBuilder};
///
/// // Track mouse movement and enter/exit events, active in any application state
/// let options = TrackingAreaOptions::new()
///     .active_always()
///     .mouse_entered_and_exited()
///     .mouse_moved();
///
/// // Use with panel macro
/// panel!(MyPanel {
///     with: {
///         tracking_area: {
///             options: options,
///             auto_resize: true
///         }
///     }
/// });
///
/// // Or use with PanelBuilder
/// PanelBuilder::new(&app, "my-panel")
///     .tracking_area(options, true)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrackingAreaOptions(objc2_app_kit::NSTrackingAreaOptions);

impl TrackingAreaOptions {
    /// Create empty tracking area options
    pub fn new() -> Self {
        Self(objc2_app_kit::NSTrackingAreaOptions::empty())
    }

    /// Track mouse moved events
    pub fn mouse_moved(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::MouseMoved;
        self
    }

    /// Track mouse entered and exited events
    pub fn mouse_entered_and_exited(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::MouseEnteredAndExited;
        self
    }

    /// Track when mouse is active in any application
    pub fn active_always(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::ActiveAlways;
        self
    }

    /// Track when mouse is active in this application
    pub fn active_in_active_app(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::ActiveInActiveApp;
        self
    }

    /// Track when mouse is active in key window
    pub fn active_in_key_window(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::ActiveInKeyWindow;
        self
    }

    /// Track when window is key
    pub fn active_when_first_responder(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::ActiveWhenFirstResponder;
        self
    }

    /// Assumes tracking area is active
    pub fn assume_inside(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::AssumeInside;
        self
    }

    /// Tracking area is in visibleRect coordinates
    pub fn in_visible_rect(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::InVisibleRect;
        self
    }

    /// Enable cursor update events
    pub fn cursor_update(mut self) -> Self {
        self.0 |= objc2_app_kit::NSTrackingAreaOptions::CursorUpdate;
        self
    }

    /// Create from raw NSTrackingAreaOptions flags
    pub fn from_raw(flags: objc2_app_kit::NSTrackingAreaOptions) -> Self {
        Self(flags)
    }

    /// Get the raw NSTrackingAreaOptions flags
    pub fn value(&self) -> objc2_app_kit::NSTrackingAreaOptions {
        self.0
    }
}

impl Default for TrackingAreaOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TrackingAreaOptions> for objc2_app_kit::NSTrackingAreaOptions {
    fn from(options: TrackingAreaOptions) -> Self {
        options.0
    }
}

impl From<objc2_app_kit::NSTrackingAreaOptions> for TrackingAreaOptions {
    fn from(value: objc2_app_kit::NSTrackingAreaOptions) -> Self {
        TrackingAreaOptions(value)
    }
}

/// Window style mask builder for NSPanel
///
/// Allows combining multiple style masks using the builder pattern.
/// Style masks control the appearance and behavior of the window frame.
///
/// # Example
/// ```rust
/// use tauri_nspanel::{StyleMask, PanelBuilder};
///
/// // Create a borderless panel that doesn't activate the app
/// let style = StyleMask::new()
///     .borderless()
///     .nonactivating_panel();
///
/// // Use with PanelBuilder
/// PanelBuilder::new(&app, "my-panel")
///     .style_mask(style)
///     .build();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StyleMask(objc2_app_kit::NSWindowStyleMask);

impl StyleMask {
    /// Create with default style mask (Titled | Closable | Miniaturizable | Resizable)
    pub fn new() -> Self {
        Self(
            objc2_app_kit::NSWindowStyleMask::Titled
                | objc2_app_kit::NSWindowStyleMask::Closable
                | objc2_app_kit::NSWindowStyleMask::Miniaturizable
                | objc2_app_kit::NSWindowStyleMask::Resizable,
        )
    }

    /// Create an empty style mask
    pub fn empty() -> Self {
        Self(objc2_app_kit::NSWindowStyleMask::empty())
    }

    /// Window has a title bar
    pub fn titled(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::Titled;
        self
    }

    /// Window has a close button
    pub fn closable(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::Closable;
        self
    }

    /// Window has a minimize button
    pub fn miniaturizable(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::Miniaturizable;
        self
    }

    /// Window can be resized
    pub fn resizable(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::Resizable;
        self
    }

    /// Window uses unified title and toolbar
    pub fn unified_title_and_toolbar(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::UnifiedTitleAndToolbar;
        self
    }

    /// Window uses full size content view
    pub fn full_size_content_view(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::FullSizeContentView;
        self
    }

    /// Window is a utility window
    pub fn utility_window(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::UtilityWindow;
        self
    }

    /// Window is a HUD window
    pub fn hud_window(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::HUDWindow;
        self
    }

    /// Window is a non-activating panel
    pub fn nonactivating_panel(mut self) -> Self {
        self.0 |= objc2_app_kit::NSWindowStyleMask::NonactivatingPanel;
        self
    }

    /// Window has no title bar or border
    pub fn borderless(mut self) -> Self {
        self.0 = objc2_app_kit::NSWindowStyleMask::Borderless;
        self
    }

    /// Create from raw NSWindowStyleMask flags
    pub fn from_raw(flags: objc2_app_kit::NSWindowStyleMask) -> Self {
        Self(flags)
    }

    /// Get the raw NSWindowStyleMask flags
    pub fn value(&self) -> objc2_app_kit::NSWindowStyleMask {
        self.0
    }
}

impl Default for StyleMask {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StyleMask> for objc2_app_kit::NSWindowStyleMask {
    fn from(mask: StyleMask) -> Self {
        mask.0
    }
}

impl From<objc2_app_kit::NSWindowStyleMask> for StyleMask {
    fn from(value: objc2_app_kit::NSWindowStyleMask) -> Self {
        StyleMask(value)
    }
}

#[derive(Default)]
pub(crate) struct PanelConfig {
    pub floating: Option<bool>,
    pub level: Option<PanelLevel>,
    pub has_shadow: Option<bool>,
    pub opaque: Option<bool>,
    pub alpha_value: Option<f64>,
    pub hides_on_deactivate: Option<bool>,
    pub becomes_key_only_if_needed: Option<bool>,
    pub accepts_mouse_moved_events: Option<bool>,
    pub ignores_mouse_events: Option<bool>,
    pub movable_by_window_background: Option<bool>,
    pub released_when_closed: Option<bool>,
    pub works_when_modal: Option<bool>,
    pub content_size: Option<Size>,
    pub style_mask: Option<StyleMask>,
    pub collection_behavior: Option<CollectionBehavior>,
    pub no_activate: Option<bool>,
    pub corner_radius: Option<f64>,
    pub transparent: Option<bool>,
}

/// Builder for creating panels with Tauri-like API
///
/// PanelBuilder provides a fluent interface that creates a Tauri window,
/// converts it to an NSPanel, and applies panel-specific configurations.
/// It can work with both the default panel type or custom panel classes
/// created with the `panel!` macro.
///
/// # Type Parameters
/// - `R`: The Tauri runtime type
/// - `T`: The panel type (must implement `FromWindow<R>`)
///
/// # Example
/// ```rust
/// use tauri_nspanel::{panel, PanelBuilder, PanelLevel};
///
/// // Using default panel type
/// let panel = PanelBuilder::new(&app, "my-panel")
///     .url(WebviewUrl::App("panel.html".into()))
///     .title("Tool Panel")
///     .level(PanelLevel::Floating)
///     .build()?;
///
/// // Using custom panel type
/// panel!(CustomPanel {
///     config: {
///         can_become_key_window: false
///     }
/// });
///
/// let custom = PanelBuilder::<_, CustomPanel>::new(&app, "custom")
///     .url(WebviewUrl::App("custom.html".into()))
///     .build()?;
/// ```
pub struct PanelBuilder<'a, R: Runtime, T: FromWindow<R> + 'static> {
    handle: &'a AppHandle<R>,
    label: String,
    url: Option<WebviewUrl>,
    title: Option<String>,
    position: Option<Position>,
    size: Option<Size>,
    pub(crate) panel_config: PanelConfig,
    window_fn: Option<WindowConfigFn<'a, R>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, R: Runtime + 'a, T: FromWindow<R> + 'static> PanelBuilder<'a, R, T> {
    /// Create a new PanelBuilder
    pub fn new(handle: &'a AppHandle<R>, label: impl Into<String>) -> Self {
        Self {
            handle,
            label: label.into(),
            url: None,
            title: None,
            position: None,
            size: None,
            panel_config: PanelConfig::default(),
            window_fn: None,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Set the webview URL
    pub fn url(mut self, url: WebviewUrl) -> Self {
        self.url = Some(url);
        self
    }

    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the window position
    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    /// Set the window size
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Set whether the panel floats above other windows
    pub fn floating(mut self, floating: bool) -> Self {
        self.panel_config.floating = Some(floating);
        self
    }

    /// Set the window level
    ///
    /// The window level determines the panel's position in the window hierarchy.
    /// Higher levels appear above lower levels.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::{PanelBuilder, PanelLevel};
    /// // Create a panel that floats above normal windows
    /// PanelBuilder::new(&app, "floating")
    ///     .level(PanelLevel::Floating)
    ///     .build();
    ///
    /// // Create a status-level panel (appears above floating panels)
    /// PanelBuilder::new(&app, "status")
    ///     .level(PanelLevel::Status)
    ///     .build();
    /// ```
    pub fn level(mut self, level: PanelLevel) -> Self {
        self.panel_config.level = Some(level);
        self
    }

    /// Set whether the panel has a shadow
    pub fn has_shadow(mut self, has_shadow: bool) -> Self {
        self.panel_config.has_shadow = Some(has_shadow);
        self
    }

    /// Set whether the panel is opaque
    pub fn opaque(mut self, opaque: bool) -> Self {
        self.panel_config.opaque = Some(opaque);
        self
    }

    /// Set the alpha value (transparency)
    pub fn alpha_value(mut self, alpha: f64) -> Self {
        self.panel_config.alpha_value = Some(alpha);
        self
    }

    /// Set whether the panel hides when the app is deactivated
    pub fn hides_on_deactivate(mut self, hides: bool) -> Self {
        self.panel_config.hides_on_deactivate = Some(hides);
        self
    }

    /// Set whether the panel becomes key window only if needed
    pub fn becomes_key_only_if_needed(mut self, value: bool) -> Self {
        self.panel_config.becomes_key_only_if_needed = Some(value);
        self
    }

    /// Set whether the panel accepts mouse moved events
    pub fn accepts_mouse_moved_events(mut self, value: bool) -> Self {
        self.panel_config.accepts_mouse_moved_events = Some(value);
        self
    }

    /// Set whether the panel ignores mouse events
    pub fn ignores_mouse_events(mut self, value: bool) -> Self {
        self.panel_config.ignores_mouse_events = Some(value);
        self
    }

    /// Set whether the panel is movable by its background
    pub fn movable_by_window_background(mut self, value: bool) -> Self {
        self.panel_config.movable_by_window_background = Some(value);
        self
    }

    /// Set whether the panel is released when closed
    pub fn released_when_closed(mut self, value: bool) -> Self {
        self.panel_config.released_when_closed = Some(value);
        self
    }

    /// Set whether the panel works when modal dialogs are displayed
    pub fn works_when_modal(mut self, value: bool) -> Self {
        self.panel_config.works_when_modal = Some(value);
        self
    }

    /// Set the content size (inner size excluding window decorations)
    pub fn content_size(mut self, size: Size) -> Self {
        self.panel_config.content_size = Some(size);
        self
    }

    /// Set the window style mask
    ///
    /// Style masks control the appearance and behavior of the window frame.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::{PanelBuilder, StyleMask};
    /// // Create a borderless panel
    /// PanelBuilder::new(&app, "borderless")
    ///     .style_mask(StyleMask::empty().borderless())
    ///     .build();
    ///
    /// // Create a HUD-style panel
    /// PanelBuilder::new(&app, "hud")
    ///     .style_mask(
    ///         StyleMask::empty()
    ///             .hud_window()
    ///             .titled()
    ///             .closable()
    ///     )
    ///     .build();
    /// ```
    pub fn style_mask(mut self, style_mask: StyleMask) -> Self {
        self.panel_config.style_mask = Some(style_mask);
        self
    }

    /// Set the collection behavior
    ///
    /// Collection behaviors control how the panel participates in Spaces, Exposé,
    /// and fullscreen mode on macOS.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::{CollectionBehavior, PanelBuilder};
    /// // Create a panel that appears on all spaces and doesn't participate in cycling
    /// PanelBuilder::new(&app, "tool-panel")
    ///     .collection_behavior(
    ///         CollectionBehavior::new()
    ///             .can_join_all_spaces()
    ///             .ignores_cycle()
    ///     )
    ///     .build();
    /// ```
    pub fn collection_behavior(mut self, behavior: CollectionBehavior) -> Self {
        self.panel_config.collection_behavior = Some(behavior);
        self
    }

    /// Prevent focus stealing during window creation
    ///
    /// Since PanelBuilder creates a regular window before converting it to a panel,
    /// the window creation phase can steal focus. When set to true, the application's
    /// activation policy is temporarily set to Prohibited during window creation,
    /// preventing this focus interruption.
    ///
    /// This works particularly well with apps that use `ActivationPolicy::Accessory`,
    /// ensuring the window is created silently before being converted to a panel.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::{PanelBuilder, PanelLevel};
    /// use tauri::WebviewUrl;
    /// // Create a utility panel that doesn't steal focus
    /// PanelBuilder::new(&app, "utility")
    ///     .url(WebviewUrl::App("utility.html".into()))
    ///     .no_activate(true)
    ///     .level(PanelLevel::Floating)
    ///     .build();
    /// ```
    pub fn no_activate(mut self, no_activate: bool) -> Self {
        self.panel_config.no_activate = Some(no_activate);
        self
    }

    /// Set the corner radius for rounded corners
    ///
    /// This enables the layer-backed view and sets the corner radius on the panel's layer,
    /// giving the panel rounded corners with the specified radius.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::PanelBuilder;
    /// use tauri::WebviewUrl;
    /// PanelBuilder::new(&app, "rounded-panel")
    ///     .url(WebviewUrl::App("index.html".into()))
    ///     .corner_radius(10.0)  // 10pt corner radius
    ///     .build();
    /// ```
    pub fn corner_radius(mut self, radius: f64) -> Self {
        self.panel_config.corner_radius = Some(radius);
        self
    }

    /// Set the panel background to be transparent
    ///
    /// This sets the window background color to clear and makes the panel non-opaque,
    /// allowing content behind the panel to show through.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::PanelBuilder;
    /// use tauri::WebviewUrl;
    /// PanelBuilder::new(&app, "transparent-panel")
    ///     .url(WebviewUrl::App("index.html".into()))
    ///     .transparent(true)  // Transparent background
    ///     .build();
    /// ```
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.panel_config.transparent = Some(transparent);
        self
    }

    /// Apply a custom configuration function to the WebviewWindowBuilder
    ///
    /// This allows access to any Tauri window configuration not exposed by the panel builder.
    /// The closure receives the WebviewWindowBuilder and should return it after applying
    /// any desired configurations.
    ///
    /// # Example
    /// ```rust
    /// use tauri_nspanel::PanelBuilder;
    /// use tauri::WebviewUrl;
    /// PanelBuilder::new(&app, "my-panel")
    ///     .url(WebviewUrl::App("index.html".into()))
    ///     .with_window(|window| {
    ///         window
    ///             .min_inner_size(300.0, 200.0)
    ///             .max_inner_size(800.0, 600.0)
    ///             .resizable(false)
    ///             .decorations(false)
    ///             .always_on_top(true)
    ///             .skip_taskbar(true)
    ///     })
    ///     .build()
    /// ```
    pub fn with_window<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
                WebviewWindowBuilder<'a, R, AppHandle<R>>,
            ) -> WebviewWindowBuilder<'a, R, AppHandle<R>>
            + 'static,
    {
        self.window_fn = Some(Box::new(f) as WindowConfigFn<'a, R>);
        self
    }

    /// Build the panel
    ///
    /// Creates a Tauri window using the configured properties, converts it to
    /// an NSPanel, and applies all panel-specific settings.
    pub fn build(self) -> tauri::Result<Arc<dyn Panel<R>>> {
        // Handle no_activate option by temporarily changing activation policy
        let original_policy = if self.panel_config.no_activate.unwrap_or(false) {
            MainThreadMarker::new().map(|mtm| unsafe {
                let app = NSApplication::sharedApplication(mtm);
                let current_policy = app.activationPolicy();
                let _success = app.setActivationPolicy(NSApplicationActivationPolicy::Prohibited);
                current_policy
            })
        } else {
            None
        };

        // Create a window first
        let mut window_builder = WebviewWindowBuilder::new(
            self.handle,
            &self.label,
            self.url.unwrap_or(WebviewUrl::App("index.html".into())),
        );

        if let Some(title) = self.title {
            window_builder = window_builder.title(title);
        }

        if let Some(position) = self.position {
            match position {
                Position::Physical(pos) => {
                    window_builder = window_builder.position(pos.x as f64, pos.y as f64);
                }
                Position::Logical(pos) => {
                    window_builder = window_builder.position(pos.x, pos.y);
                }
            }
        }

        if let Some(size) = self.size {
            match size {
                Size::Physical(s) => {
                    window_builder = window_builder.inner_size(s.width as f64, s.height as f64);
                }
                Size::Logical(s) => {
                    window_builder = window_builder.inner_size(s.width, s.height);
                }
            }
        }

        // Apply custom configuration if provided
        if let Some(window_fn) = self.window_fn {
            window_builder = window_fn(window_builder);
        }

        // Build the window
        let window = window_builder.build()?;

        // Convert to panel
        let panel = window.to_panel::<T>().unwrap();

        // Apply panel configuration using the Panel trait methods
        if let Some(floating) = self.panel_config.floating {
            panel.set_floating_panel(floating);
        }
        if let Some(level) = self.panel_config.level {
            panel.set_level(level.value());
        }
        if let Some(has_shadow) = self.panel_config.has_shadow {
            panel.set_has_shadow(has_shadow);
        }
        if let Some(opaque) = self.panel_config.opaque {
            panel.set_opaque(opaque);
        }
        if let Some(alpha_value) = self.panel_config.alpha_value {
            panel.set_alpha_value(alpha_value);
        }
        if let Some(hides) = self.panel_config.hides_on_deactivate {
            panel.set_hides_on_deactivate(hides);
        }
        if let Some(value) = self.panel_config.becomes_key_only_if_needed {
            panel.set_becomes_key_only_if_needed(value);
        }
        if let Some(value) = self.panel_config.accepts_mouse_moved_events {
            panel.set_accepts_mouse_moved_events(value);
        }
        if let Some(value) = self.panel_config.ignores_mouse_events {
            panel.set_ignores_mouse_events(value);
        }
        if let Some(value) = self.panel_config.movable_by_window_background {
            panel.set_movable_by_window_background(value);
        }
        if let Some(value) = self.panel_config.released_when_closed {
            panel.set_released_when_closed(value);
        }
        if let Some(value) = self.panel_config.works_when_modal {
            panel.set_works_when_modal(value);
        }
        if let Some(style_mask) = self.panel_config.style_mask {
            panel.set_style_mask(style_mask.0);
        }
        if let Some(behavior) = self.panel_config.collection_behavior {
            panel.set_collection_behavior(behavior.0);
        }
        if let Some(radius) = self.panel_config.corner_radius {
            panel.set_corner_radius(radius);
        }
        if let Some(transparent) = self.panel_config.transparent {
            panel.set_transparent(transparent);
        }

        // Restore original activation policy if we changed it
        if let Some(policy) = original_policy {
            if let Some(mtm) = MainThreadMarker::new() {
                let app = NSApplication::sharedApplication(mtm);
                let _success = app.setActivationPolicy(policy);
            }
        }

        Ok(panel)
    }
}
