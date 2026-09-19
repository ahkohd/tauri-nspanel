use std::sync::Arc;

use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSPanel};
use objc2_foundation::{MainThreadMarker, NSSize};
use tauri::{AppHandle, Position, Runtime, Size, WebviewUrl, WebviewWindowBuilder};

use crate::{FromWindow, Panel, WebviewWindowExt};

/// Type alias for window configuration function
type WindowConfigFn<'a, R> = Box<
    dyn FnOnce(
        WebviewWindowBuilder<'a, R, AppHandle<R>>,
    ) -> WebviewWindowBuilder<'a, R, AppHandle<R>>,
>;

/// Axes along which a resizable panel may be resized.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ResizeDirection {
    /// Allow the panel to be resized horizontally and vertically.
    #[default]
    Both,
    /// Allow the panel width to change while keeping its current height fixed.
    Horizontal,
    /// Allow the panel height to change while keeping its current width fixed.
    Vertical,
}

impl ResizeDirection {
    fn apply(self, panel: &NSPanel) {
        if self == Self::Both {
            return;
        }

        let Some(content_view) = panel.contentView() else {
            return;
        };

        let current_size = content_view.frame().size;
        let (min_size, max_size) = resize_constraints(
            self,
            current_size,
            panel.contentMinSize(),
            panel.contentMaxSize(),
        );

        panel.setContentMinSize(min_size);
        panel.setContentMaxSize(max_size);
    }
}

fn resize_constraints(
    direction: ResizeDirection,
    current_size: NSSize,
    mut min_size: NSSize,
    mut max_size: NSSize,
) -> (NSSize, NSSize) {
    match direction {
        ResizeDirection::Both => {}
        ResizeDirection::Horizontal => {
            min_size.height = current_size.height;
            max_size.height = current_size.height;
        }
        ResizeDirection::Vertical => {
            min_size.width = current_size.width;
            max_size.width = current_size.width;
        }
    }

    (min_size, max_size)
}

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
/// ```no_run
/// use tauri::AppHandle;
/// use tauri_nspanel::{tauri_panel, CollectionBehavior, PanelBuilder};
///
/// tauri_panel! {
///     panel!(SpacesPanel {})
/// }
///
/// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
///
/// // Create a panel that appears on all spaces and ignores Cmd+Tab cycling
/// let behavior = CollectionBehavior::new()
///     .can_join_all_spaces()
///     .ignores_cycle();
///
/// // Use with PanelBuilder
/// let _panel = PanelBuilder::<_, SpacesPanel>::new(app, "my-panel")
///     .collection_behavior(behavior)
///     .build()?;
/// # Ok(())
/// # }
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
/// ```
/// use tauri_nspanel::{tauri_panel, TrackingAreaOptions};
///
/// tauri_panel! {
///     panel!(TrackingPanel {
///         with: {
///             tracking_area: {
///                 options: TrackingAreaOptions::new()
///                     .active_always()
///                     .mouse_entered_and_exited()
///                     .mouse_moved(),
///                 auto_resize: true
///             }
///         }
///     })
/// }
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

    /// Keep the tracking area synchronized with the view's visible rectangle.
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
/// ```no_run
/// use tauri::AppHandle;
/// use tauri_nspanel::{tauri_panel, PanelBuilder, StyleMask};
///
/// tauri_panel! {
///     panel!(BorderlessPanel {})
/// }
///
/// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
///
/// // Create the borderless window through Tauri, then add non-activating panel behavior.
/// let _panel = PanelBuilder::<_, BorderlessPanel>::new(app, "my-panel")
///     .with_window(|window| window.decorations(false))
///     .add_style_mask(StyleMask::empty().nonactivating_panel())
///     .build()?;
/// # Ok(())
/// # }
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
    pub style_mask: Option<StyleMaskConfig>,
    pub collection_behavior: Option<CollectionBehavior>,
    pub no_activate: Option<bool>,
    pub corner_radius: Option<f64>,
    pub transparent: Option<bool>,
}

pub(crate) enum StyleMaskConfig {
    Replace(StyleMask),
    Add(StyleMask),
}

/// Builder for creating panels with Tauri-like API
///
/// PanelBuilder provides a fluent interface that creates a Tauri window,
/// converts it to an NSPanel, and applies panel-specific configurations.
/// It works with custom panel classes created with the `panel!` macro.
///
/// # Type Parameters
/// - `R`: The Tauri runtime type
/// - `T`: The panel type (must implement `FromWindow<R>`)
///
/// # Example
/// ```no_run
/// use tauri::{AppHandle, WebviewUrl};
/// use tauri_nspanel::{tauri_panel, PanelBuilder, PanelLevel};
///
/// tauri_panel! {
///     panel!(ToolPanel {
///         config: {
///             can_become_key_window: false
///         }
///     })
/// }
///
/// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
/// let _panel = PanelBuilder::<_, ToolPanel>::new(app, "my-panel")
///     .url(WebviewUrl::App("panel.html".into()))
///     .title("Tool Panel")
///     .level(PanelLevel::Floating)
///     .build()?;
/// # Ok(())
/// # }
/// ```
pub struct PanelBuilder<'a, R: Runtime, T: FromWindow<R> + 'static> {
    handle: &'a AppHandle<R>,
    label: String,
    url: Option<WebviewUrl>,
    title: Option<String>,
    position: Option<Position>,
    size: Option<Size>,
    min_size: Option<Size>,
    max_size: Option<Size>,
    resizable: Option<bool>,
    resize_direction: ResizeDirection,
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
            min_size: None,
            max_size: None,
            resizable: None,
            resize_direction: ResizeDirection::Both,
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

    /// Set the minimum content size of the panel.
    pub fn min_size(mut self, size: Size) -> Self {
        self.min_size = Some(size);
        self
    }

    /// Set the maximum content size of the panel.
    pub fn max_size(mut self, size: Size) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Set whether the panel can be resized by the user.
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    /// Limit resizing to one axis.
    ///
    /// The dimension on the disabled axis is fixed to the panel's content size
    /// after its minimum and maximum size constraints have been applied.
    pub fn resize_direction(mut self, direction: ResizeDirection) -> Self {
        self.resize_direction = direction;
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
    /// ```no_run
    /// use tauri::AppHandle;
    /// use tauri_nspanel::{tauri_panel, PanelBuilder, PanelLevel};
    ///
    /// tauri_panel! {
    ///     panel!(LevelPanel {})
    /// }
    ///
    /// # fn create_panels(app: &AppHandle) -> tauri::Result<()> {
    /// // Create a panel that floats above normal windows
    /// let _floating = PanelBuilder::<_, LevelPanel>::new(app, "floating")
    ///     .level(PanelLevel::Floating)
    ///     .build()?;
    ///
    /// // Create a status-level panel (appears above floating panels)
    /// let _status = PanelBuilder::<_, LevelPanel>::new(app, "status")
    ///     .level(PanelLevel::Status)
    ///     .build()?;
    /// # Ok(())
    /// # }
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

    /// Set whether AppKit releases the panel when it closes.
    ///
    /// This should normally remain `false` for Tauri-managed panels. [`Panel::to_window`]
    /// restores it to `false` before returning ownership to Tauri.
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

    /// Replace the window style mask.
    ///
    /// Style masks control the appearance and behavior of the window frame. AppKit may reject
    /// structural changes to a live window; [`Self::build`] returns that error instead of aborting
    /// the process. Prefer [`Self::add_style_mask`] when only enabling additional behavior.
    ///
    /// # Example
    /// ```no_run
    /// use tauri::AppHandle;
    /// use tauri_nspanel::{tauri_panel, PanelBuilder, StyleMask};
    ///
    /// tauri_panel! {
    ///     panel!(StyledPanel {})
    /// }
    ///
    /// # fn create_panels(app: &AppHandle) -> tauri::Result<()> {
    /// // Create the native window without decorations before applying the exact mask.
    /// let _borderless = PanelBuilder::<_, StyledPanel>::new(app, "borderless")
    ///     .with_window(|window| window.decorations(false))
    ///     .style_mask(StyleMask::empty().nonactivating_panel())
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn style_mask(mut self, style_mask: StyleMask) -> Self {
        self.panel_config.style_mask = Some(StyleMaskConfig::Replace(style_mask));
        self
    }

    /// Add flags to the window's existing style mask.
    ///
    /// This preserves the structural styles selected by Tauri and is the preferred way to enable
    /// panel behavior such as [`StyleMask::nonactivating_panel`].
    pub fn add_style_mask(mut self, style_mask: StyleMask) -> Self {
        self.panel_config.style_mask = Some(StyleMaskConfig::Add(style_mask));
        self
    }

    /// Set the collection behavior
    ///
    /// Collection behaviors control how the panel participates in Spaces, Exposé,
    /// and fullscreen mode on macOS.
    ///
    /// # Example
    /// ```no_run
    /// use tauri::AppHandle;
    /// use tauri_nspanel::{tauri_panel, CollectionBehavior, PanelBuilder};
    ///
    /// tauri_panel! {
    ///     panel!(SpacesPanel {})
    /// }
    ///
    /// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
    /// // Create a panel that appears on all spaces and doesn't participate in cycling
    /// let _panel = PanelBuilder::<_, SpacesPanel>::new(app, "tool-panel")
    ///     .collection_behavior(
    ///         CollectionBehavior::new()
    ///             .can_join_all_spaces()
    ///             .ignores_cycle()
    ///     )
    ///     .build()?;
    /// # Ok(())
    /// # }
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
    /// ```no_run
    /// use tauri::{AppHandle, WebviewUrl};
    /// use tauri_nspanel::{tauri_panel, PanelBuilder, PanelLevel};
    ///
    /// tauri_panel! {
    ///     panel!(UtilityPanel {})
    /// }
    ///
    /// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
    /// // Create a utility panel that doesn't steal focus
    /// let _panel = PanelBuilder::<_, UtilityPanel>::new(app, "utility")
    ///     .url(WebviewUrl::App("utility.html".into()))
    ///     .no_activate(true)
    ///     .level(PanelLevel::Floating)
    ///     .build()?;
    /// # Ok(())
    /// # }
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
    /// ```no_run
    /// use tauri::{AppHandle, WebviewUrl};
    /// use tauri_nspanel::{tauri_panel, PanelBuilder};
    ///
    /// tauri_panel! {
    ///     panel!(RoundedPanel {})
    /// }
    ///
    /// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
    /// let _panel = PanelBuilder::<_, RoundedPanel>::new(app, "rounded-panel")
    ///     .url(WebviewUrl::App("index.html".into()))
    ///     .corner_radius(10.0)  // 10pt corner radius
    ///     .build()?;
    /// # Ok(())
    /// # }
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
    /// ```no_run
    /// use tauri::{AppHandle, WebviewUrl};
    /// use tauri_nspanel::{tauri_panel, PanelBuilder};
    ///
    /// tauri_panel! {
    ///     panel!(TransparentPanel {})
    /// }
    ///
    /// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
    /// let _panel = PanelBuilder::<_, TransparentPanel>::new(app, "transparent-panel")
    ///     .url(WebviewUrl::App("index.html".into()))
    ///     .transparent(true)  // Transparent background
    ///     .build()?;
    /// # Ok(())
    /// # }
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
    /// ```no_run
    /// use tauri::{AppHandle, WebviewUrl};
    /// use tauri_nspanel::{tauri_panel, PanelBuilder};
    ///
    /// tauri_panel! {
    ///     panel!(ConfiguredPanel {})
    /// }
    ///
    /// # fn create_panel(app: &AppHandle) -> tauri::Result<()> {
    /// let _panel = PanelBuilder::<_, ConfiguredPanel>::new(app, "my-panel")
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
    ///     .build()?;
    /// # Ok(())
    /// # }
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
            MainThreadMarker::new().map(|mtm| {
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

        if let Some(resizable) = self.resizable {
            window_builder = window_builder.resizable(resizable);
        }

        // Apply custom configuration if provided
        if let Some(window_fn) = self.window_fn {
            window_builder = window_fn(window_builder);
        }

        // Build the window while the temporary activation policy is in effect.
        let window = window_builder.build();

        // The temporary policy is only needed while Tauri creates the native window. Restore it
        // before applying fallible panel configuration so errors cannot leave the app prohibited.
        if let Some(policy) = original_policy {
            if let Some(mtm) = MainThreadMarker::new() {
                let app = NSApplication::sharedApplication(mtm);
                let _success = app.setActivationPolicy(policy);
            }
        }

        let window = window?;

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
            let result = match style_mask {
                StyleMaskConfig::Replace(style_mask) => panel.set_style_mask(style_mask.0),
                StyleMaskConfig::Add(style_mask) => panel.add_style_mask(style_mask.0),
            };

            if let Err(error) = result {
                if let Some(window) = panel.to_window() {
                    let _ = window.close();
                }

                return Err(tauri::Error::Anyhow(error.into()));
            }
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

        // Apply size constraints after the final style mask so they describe
        // the panel's final content area rather than its temporary window frame.
        if let Some(min_size) = self.min_size {
            window.set_min_size(Some(min_size))?;
        }
        if let Some(max_size) = self.max_size {
            window.set_max_size(Some(max_size))?;
        }
        self.resize_direction.apply(panel.as_panel());

        Ok(panel)
    }
}

#[cfg(test)]
mod tests {
    use super::{resize_constraints, ResizeDirection};
    use objc2_foundation::NSSize;

    #[test]
    fn horizontal_resize_fixes_only_the_height() {
        let (min_size, max_size) = resize_constraints(
            ResizeDirection::Horizontal,
            NSSize::new(500.0, 300.0),
            NSSize::new(200.0, 100.0),
            NSSize::new(800.0, 600.0),
        );

        assert_eq!(min_size, NSSize::new(200.0, 300.0));
        assert_eq!(max_size, NSSize::new(800.0, 300.0));
    }

    #[test]
    fn vertical_resize_fixes_only_the_width() {
        let (min_size, max_size) = resize_constraints(
            ResizeDirection::Vertical,
            NSSize::new(500.0, 300.0),
            NSSize::new(200.0, 100.0),
            NSSize::new(800.0, 600.0),
        );

        assert_eq!(min_size, NSSize::new(500.0, 100.0));
        assert_eq!(max_size, NSSize::new(500.0, 600.0));
    }

    #[test]
    fn both_resize_axes_preserve_existing_constraints() {
        let min = NSSize::new(200.0, 100.0);
        let max = NSSize::new(800.0, 600.0);

        let constraints =
            resize_constraints(ResizeDirection::Both, NSSize::new(500.0, 300.0), min, max);

        assert_eq!(constraints, (min, max));
    }
}
