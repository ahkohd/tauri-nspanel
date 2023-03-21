#[macro_export]
macro_rules! panel_delegate {
    ($delegate_name:ident { $($fn_name:ident),* $(,)* }) => {{
        use $crate::objc::{
            class,
            declare::ClassDecl,
            msg_send,
            runtime::{self, Class, Object, Protocol, Sel},
            sel, sel_impl, Message,
        };
        use $crate::cocoa::base::{id, nil};
        use $crate::objc_foundation::INSObject;
        use $crate::objc_id::ShareId;
        use $crate::raw_nspanel::RawNSPanel;

        macro_rules! snake_to_camel {
            ($input:ident) => {{
                let mut camel_case = String::new();
                let mut chars = $input.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '_' {
                        if let Some(next) = chars.peek() {
                            camel_case.push(next.to_ascii_uppercase());
                            chars.next();
                        }
                    } else {
                        camel_case.push(c);
                    }
                }
                camel_case
            }};
        }

        macro_rules! sel_from_func {
            ($func:ident) => ({
                let func_str = stringify!($func);
                let sel_str = snake_to_camel!(func_str);
                sel_impl!(format!("{}:{}", sel_str, '\0').as_str())
            });
        }

        #[allow(dead_code)]
        const DELEGATE_CLS_NAME: &str = stringify!($delegate_name);

        #[allow(dead_code)]
        struct $delegate_name {}

        impl $delegate_name {
             #[allow(dead_code)]
            fn get_class() -> &'static Class {
                Class::get(DELEGATE_CLS_NAME).unwrap_or_else(Self::define_class)
            }

            #[allow(dead_code)]
            fn define_class() -> &'static Class {
                let mut cls = ClassDecl::new(DELEGATE_CLS_NAME, class!(NSObject))
                    .unwrap_or_else(|| panic!("Unable to register {} class", DELEGATE_CLS_NAME));

                cls.add_protocol(
                    Protocol::get("NSWindowDelegate").expect("Failed to get NSWindowDelegate protocol"),
                );

                unsafe {
                    cls.add_ivar::<id>("panel");

                    cls.add_method(
                        sel!(setPanel:),
                        Self::setPanel as extern "C" fn(&mut Object, Sel, id),
                    );


                    $(
                        cls.add_method(
                            sel_from_func!($fn_name),
                            Self::$fn_name as extern "C" fn(&Object, Sel, id),
                        );
                    )*
                }

                cls.register()
            }

            extern "C" fn setPanel(this: &mut Object, _: Sel, panel: id) {
                unsafe { this.set_ivar("panel", panel) };
            }

            $(
                extern "C" fn $fn_name(this: &Object, _: Sel, _: id) {
                    let panel: id = unsafe { *this.get_ivar("panel") };
                    $fn_name(unsafe {Id::from_retained_ptr(panel as *mut RawNSPanel) });
                }
            )*
        }

        unsafe impl Message for $delegate_name {}

        impl INSObject for $delegate_name {
            fn class() -> &'static runtime::Class {
                Self::get_class()
            }
        }

        impl $delegate_name {
            pub fn set_panel(&self, panel: ShareId<RawNSPanel>) {
                let _: () = unsafe { msg_send![self, setPanel: panel] };
            }
        }


        $delegate_name::new()
    }};
}
