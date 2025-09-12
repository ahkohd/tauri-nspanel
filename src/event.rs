/// Macro to create a custom event handler for panels
///
/// This creates an NSWindowDelegate that responds to window events with type-safe callbacks.
///
/// This macro generates a type-safe delegate class with strongly typed callbacks.
/// Users specify parameter types directly in the macro, and callbacks receive
/// properly typed arguments instead of raw pointers.
///
/// # References
///
/// - [objc2 NSWindowDelegate trait documentation](https://docs.rs/objc2-app-kit/0.3.1/objc2_app_kit/trait.NSWindowDelegate.html)
/// - [Apple NSWindowDelegate documentation](https://developer.apple.com/documentation/appkit/nswindowdelegate)
///
/// # Selector Generation Rules
///
/// The macro generates Objective-C selectors based on how you declare the methods:
///
/// - **Single parameter**: `method_name(param: Type)` → `methodName:`
///   - Example: `window_did_become_key(notification: &NSNotification)` → `windowDidBecomeKey:`
///
/// - **Multiple parameters**: `method_name(first: Type1, second: Type2)` → `methodName:second:`
///   - Example: `window_will_resize(window: &NSWindow, to_size: &NSSize)` → `windowWillResize:toSize:`
///   - The first parameter is always anonymous (`:`) in the selector
///   - Subsequent parameters use their names as selector parts
///
/// **Parameter Naming**: The macro automatically converts snake_case to camelCase:
/// - `to_size` → `toSize`
/// - `to_object` → `toObject`
/// - `for_client` → `forClient`
///
/// You can use Rust's conventional snake_case naming and the macro will generate
/// the correct camelCase selectors to match Apple's NSWindowDelegate signatures.
///
/// **Important**: Always check the references above to ensure your method names and
/// parameter names match the exact NSWindowDelegate protocol signatures.
///
/// Usage:
/// ```
/// use tauri_nspanel::tauri_panel;
///
/// tauri_panel! {
///     panel_event!(MyPanelEventHandler {
///         window_did_become_key(notification: &NSNotification) -> (),
///         window_should_close(window: &NSWindow) -> Bool,
///         window_will_resize(window: &NSWindow, to_size: &NSSize) -> NSSize,
///         window_will_return_field_editor(sender: &NSWindow, client: Option<&AnyObject>) -> Option<Retained<NSObject>>
///     })
/// }
///
/// let handler = MyPanelEventHandler::new();
///
/// handler.window_did_become_key(|notification| {
///     // notification is already typed as &NSNotification
///     println!("Window became key: {:?}", notification);
/// });
///
/// handler.window_should_close(|window| {
///     // window is already typed as &NSWindow
///     println!("Should close window: {:?}?", window);
///     Bool::new(true) // Allow closing
/// });
/// ```
///
/// # Type Specification
///
/// You must specify the exact types for parameters, including references:
/// - Object types usually take references: `&NSWindow`, `&NSNotification`
/// - Value types also take references: `&NSSize`, `&NSRect`, `&NSPoint`
/// - Optional parameters: `Option<&AnyObject>`
/// - The macro does NOT automatically add references
///
/// ## Return Values
/// Methods must specify their return type explicitly:
/// - `-> ()` for void methods (no return value)
/// - `-> Bool` for BOOL returns (objc2 Bool type)
/// - `-> Option<Retained<NSObject>>` for nullable object returns
/// - `-> NSSize` for NSSize value returns
/// - `-> NSRect` for NSRect value returns
/// - `-> NSPoint` for NSPoint value returns
/// - Other types as needed by the delegate method
///
/// The macro handles conversion between Rust types and Objective-C types automatically.
#[macro_export]
macro_rules! panel_event {
    (
        $handler_name:ident {
            $(
                $method:ident ( $first_param:ident : $first_type:ty $(, $param:ident : $param_type:ty)* $(,)? ) -> $return_type:ty
            ),* $(,)?
        }
    ) => {
        $crate::pastey::paste! {
                // Generate typed callback signatures for each method
                $(
                    pub type [<$handler_name $method:camel Callback>] = std::boxed::Box<
                        dyn Fn($first_type $(, $param_type)*) -> $return_type
                    >;
                )*

                struct [<$handler_name Ivars>] {
                   $(
                       [<$method:snake>]: std::cell::Cell<Option<[<$handler_name $method:camel Callback>]>>,
                   )*
                   // Mouse event callbacks
                   mouse_entered_callback: std::cell::Cell<Option<Box<dyn Fn(&$crate::objc2_app_kit::NSEvent)>>>,
                   mouse_exited_callback: std::cell::Cell<Option<Box<dyn Fn(&$crate::objc2_app_kit::NSEvent)>>>,
                   mouse_moved_callback: std::cell::Cell<Option<Box<dyn Fn(&$crate::objc2_app_kit::NSEvent)>>>,
                   cursor_update_callback: std::cell::Cell<Option<Box<dyn Fn(&$crate::objc2_app_kit::NSEvent)>>>,
                }

                define_class!(
                    #[unsafe(super(NSObject))]
                    #[name = stringify!($handler_name)]
                    #[thread_kind = MainThreadOnly]

                    #[ivars = [<$handler_name Ivars>]]
                    struct $handler_name;

                    unsafe impl NSObjectProtocol for $handler_name {}

                    unsafe impl NSWindowDelegate for $handler_name {
                        $(
                            #[doc = concat!(" Objective-C delegate method: ", stringify!($method), "_:", $(stringify!([<$param:lower_camel>]), ":"),*)]
                            #[allow(non_snake_case)]
                            #[unsafe(method([<$method:lower_camel>]:$([<$param:lower_camel>]:)*))]
                            fn [<__ $method:snake>](&self, [<$first_param:lower_camel>]: $first_type $(, [<$param:lower_camel>]: $param_type )* ) -> $return_type {
                                // Take the callback from the cell temporarily
                                let callback = self.ivars().[<$method:snake>].take();
                                if let Some(callback) = callback {
                                    // Call the callback with typed parameters
                                    let result = callback([<$first_param:lower_camel>] $(, [<$param:lower_camel>])*);

                                    // Put the callback back
                                    self.ivars().[<$method:snake>].set(Some(callback));

                                    result
                                } else {
                                    // Return default value for the type
                                    Default::default()
                                }
                            }
                        )*
                    }

                    impl $handler_name {
                        // Mouse event methods
                        #[unsafe(method(mouseEntered:))]
                        fn mouse_entered(&self, event: &$crate::objc2_app_kit::NSEvent) {
                            let ivars = self.ivars();
                            if let Some(callback) = ivars.mouse_entered_callback.take() {
                                callback(event);
                                ivars.mouse_entered_callback.set(Some(callback));
                            }
                        }

                        #[unsafe(method(mouseExited:))]
                        fn mouse_exited(&self, event: &$crate::objc2_app_kit::NSEvent) {
                            let ivars = self.ivars();
                            if let Some(callback) = ivars.mouse_exited_callback.take() {
                                callback(event);
                                ivars.mouse_exited_callback.set(Some(callback));
                            }
                        }

                        #[unsafe(method(mouseMoved:))]
                        fn mouse_moved(&self, event: &$crate::objc2_app_kit::NSEvent) {
                            let ivars = self.ivars();
                            if let Some(callback) = ivars.mouse_moved_callback.take() {
                                callback(event);
                                ivars.mouse_moved_callback.set(Some(callback));
                            }
                        }

                        #[unsafe(method(cursorUpdate:))]
                        fn cursor_update(&self, event: &$crate::objc2_app_kit::NSEvent) {
                            let ivars = self.ivars();
                            if let Some(callback) = ivars.cursor_update_callback.take() {
                                callback(event);
                                ivars.cursor_update_callback.set(Some(callback));
                            }
                        }
                    }
                );

                impl $handler_name {
                    /// Create a new event handler instance
                    pub fn new() -> Retained<Self> {
                        unsafe {
                            // Get main thread marker
                            let mtm = MainThreadMarker::new().expect("Must be on main thread");

                            // Allocate instance
                            let this = Self::alloc(mtm);
                            // Set ivars
                            let this = this.set_ivars([<$handler_name Ivars>] {
                                $(
                                    [<$method:snake>]: std::cell::Cell::new(None),
                                )*
                                mouse_entered_callback: std::cell::Cell::new(None),
                                mouse_exited_callback: std::cell::Cell::new(None),
                                mouse_moved_callback: std::cell::Cell::new(None),
                                cursor_update_callback: std::cell::Cell::new(None),
                            });
                            // Initialize
                            msg_send![super(this), init]
                        }
                    }

                    $(
                        #[doc = " A callback for the `" $method "` event"]
                        pub fn [<$method:snake>]<F>(&self, callback: F)
                        where
                            F: Fn($first_type $(, $param_type)*) -> $return_type + 'static
                        {
                            let boxed_callback: [<$handler_name $method:camel Callback>] = std::boxed::Box::new(callback);

                            // Store new callback
                            self.ivars().[<$method:snake>].set(Some(boxed_callback));
                        }
                    )*

                    // Mouse event handlers
                    /// Set the mouse entered callback
                    pub fn on_mouse_entered<F>(&self, callback: F)
                    where
                        F: Fn(&$crate::objc2_app_kit::NSEvent) + 'static
                    {
                        self.ivars().mouse_entered_callback.set(Some(Box::new(callback)));
                    }

                    /// Set the mouse exited callback
                    pub fn on_mouse_exited<F>(&self, callback: F)
                    where
                        F: Fn(&$crate::objc2_app_kit::NSEvent) + 'static
                    {
                        self.ivars().mouse_exited_callback.set(Some(Box::new(callback)));
                    }

                    /// Set the mouse moved callback
                    pub fn on_mouse_moved<F>(&self, callback: F)
                    where
                        F: Fn(&$crate::objc2_app_kit::NSEvent) + 'static
                    {
                        self.ivars().mouse_moved_callback.set(Some(Box::new(callback)));
                    }

                    /// Set the cursor update callback
                    pub fn on_cursor_update<F>(&self, callback: F)
                    where
                        F: Fn(&$crate::objc2_app_kit::NSEvent) + 'static
                    {
                        self.ivars().cursor_update_callback.set(Some(Box::new(callback)));
                    }
                }

                /// Implement AsRef for idiomatic conversion to ProtocolObject
                impl std::convert::AsRef<ProtocolObject<dyn NSWindowDelegate>> for $handler_name {
                    fn as_ref(&self) -> &ProtocolObject<dyn NSWindowDelegate> {
                        unsafe {
                            ProtocolObject::from_ref(self)
                        }
                    }
                }
        }
    };
}

// Example usage:
//
// use tauri_nspanel::tauri_panel;
//
// tauri_panel! {
//     panel_event!(MyPanelEventHandler {
//         window_did_become_key(notification: &NSNotification) -> (),
//         window_will_close(window: &NSWindow) -> (),
//         window_should_close(window: &NSWindow) -> Bool,
//         window_will_resize(window: &NSWindow, to_size: &NSSize) -> NSSize
//     })
// }
//
// let handler = MyPanelEventHandler::new();
//
// // Example: Handle window_did_become_key notification
// handler.window_did_become_key(|notification| {
//     println!("Window became key with notification: {:?}", notification);
// });
//
// // Example: Handle window_should_close with bool return
// handler.window_should_close(|window| {
//     println!("Should close window?");
//     Bool::new(true) // Allow closing
// });
//
// // Example: Handle window_will_resize with NSSize return
// handler.window_will_resize(|window, proposed_size| {
//     // Enforce minimum size
//     NSSize {
//         width: proposed_size.width.max(200.0),
//         height: proposed_size.height.max(100.0),
//     }
// });
//
// // Use with panel
// panel.set_event_handler(Some(handler.as_ref()));
