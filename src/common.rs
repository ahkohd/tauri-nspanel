/// Top-level macro that wraps panel and event handler declarations
#[macro_export]
macro_rules! tauri_panel {
    // Pattern for panel class definition
    ($panel_name:ident {
        $(
            config: {
                $($method_name:ident: $method_value:expr),* $(,)?
            }
        )?
        $(
            with: {
                $(tracking_area: {
                    $($tracking_key:ident: $tracking_value:expr),* $(,)?
                })?
            }
        )?
    }) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSPanel, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $crate::panel!($panel_name {
            $(
                config: {
                    $($method_name: $method_value),*
                }
            )?
            $(
                with: {
                    $(tracking_area: {
                        $($tracking_key: $tracking_value),*
                    })?
                }
            )?
        });
    };

    // Pattern for event handler declarations
    (
        $(
            panel_event!($handler_name:ident {
                $(
                    $method:ident ( $first_param:ident : $first_type:ty $(, $param:ident : $param_type:ty)* $(,)? ) -> $return_type:ty
                ),* $(,)?
            })
        )*
    ) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSPanel, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $(
            $crate::panel_event!($handler_name {
                $(
                    $method ( $first_param : $first_type $(, $param : $param_type)* ) -> $return_type
                ),*
            });
        )*
    };

    // Pattern for mixed panel and event handler declarations
    (
        $(
            panel!($panel_name:ident {
                $(
                    config: {
                        $($method_name:ident: $method_value:expr),* $(,)?
                    }
                )?
                $(
                    with: {
                        $(tracking_area: {
                            $($tracking_key:ident: $tracking_value:expr),* $(,)?
                        })?
                    }
                )?
            })
        )*
        $(
            panel_event!($handler_name:ident {
                $(
                    $event_method:ident ( $first_param:ident : $first_type:ty $(, $param:ident : $param_type:ty)* $(,)? ) -> $return_type:ty
                ),* $(,)?
            })
        )*
    ) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSPanel, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $(
            $crate::panel!($panel_name {
                $(
                    config: {
                        $($method_name: $method_value),*
                    }
                )?
                $(
                    with: {
                        $(tracking_area: {
                            $($tracking_key: $tracking_value),*
                        })?
                    }
                )?
            });
        )*

        $(
            $crate::panel_event!($handler_name {
                $(
                    $event_method ( $first_param : $first_type $(, $param : $param_type)* ) -> $return_type
                ),*
            });
        )*
    };
}
