use cocoa::{
    appkit::NSWindowCollectionBehavior,
    base::{id, nil, BOOL, YES},
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{self, Class, Object, Protocol, Sel},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;
use objc_id::{Id, ShareId};
use tauri::{Runtime, Window};

extern "C" {
    pub fn object_setClass(obj: id, cls: id) -> id;
}

const CLS_NAME: &str = "RawNSPanel";

pub struct RawNSPanel;

impl RawNSPanel {
    fn get_class() -> &'static Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }

    fn define_class() -> &'static Class {
        let mut cls = ClassDecl::new(CLS_NAME, class!(NSPanel))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        unsafe {
            cls.add_method(
                sel!(canBecomeKeyWindow),
                Self::can_become_key_window as extern "C" fn(&Object, Sel) -> BOOL,
            );
        }

        cls.register()
    }

    /// Returns YES to ensure that RawNSPanel can become a key window
    extern "C" fn can_become_key_window(_: &Object, _: Sel) -> BOOL {
        YES
    }
}
unsafe impl Message for RawNSPanel {}

impl RawNSPanel {
    pub fn show(&self) {
        self.make_first_responder(Some(self.content_view()));
        self.order_front_regardless();
        self.make_key_window();
    }

    pub fn is_visible(&self) -> bool {
        let flag: BOOL = unsafe { msg_send![self, isVisible] };
        flag == YES
    }

    pub fn make_key_window(&self) {
        let _: () = unsafe { msg_send![self, makeKeyWindow] };
    }

    pub fn order_front_regardless(&self) {
        let _: () = unsafe { msg_send![self, orderFrontRegardless] };
    }

    pub fn order_out(&self, sender: Option<id>) {
        let _: () = unsafe { msg_send![self, orderOut: sender.unwrap_or(nil)] };
    }

    pub fn content_view(&self) -> id {
        unsafe { msg_send![self, contentView] }
    }

    pub fn make_first_responder(&self, sender: Option<id>) {
        if let Some(responder) = sender {
            let _: () = unsafe { msg_send![self, makeFirstResponder: responder] };
        } else {
            let _: () = unsafe { msg_send![self, makeFirstResponder: self] };
        }
    }

    pub fn set_level(&self, level: i32) {
        let _: () = unsafe { msg_send![self, setLevel: level] };
    }

    pub fn set_style_mask(&self, style_mask: i32) {
        let _: () = unsafe { msg_send![self, setStyleMask: style_mask] };
    }

    pub fn set_collection_behaviour(&self, behaviour: NSWindowCollectionBehavior) {
        let _: () = unsafe { msg_send![self, setCollectionBehavior: behaviour] };
    }

    pub fn set_delegate<T>(&self, delegate: Id<T>) {
        let _: () = unsafe { msg_send![self, setDelegate: delegate] };
    }

    /// Create an NSPanel from a Tauri window
    pub fn from<R: Runtime>(window: &Window<R>) -> Id<Self> {
        let ns_window: id = window.ns_window().unwrap() as _;
        let ns_panel: id = unsafe { msg_send![Self::class(), class] };
        unsafe {
            object_setClass(ns_window, ns_panel);
            Id::from_retained_ptr(ns_window as *mut Self)
        }
    }
}

impl INSObject for RawNSPanel {
    fn class() -> &'static runtime::Class {
        RawNSPanel::get_class()
    }
}

#[allow(dead_code)]
const DELEGATE_CLS_NAME: &str = "RawNSPanelDelegate";

#[allow(dead_code)]
struct RawNSPanelDelegate {}

impl RawNSPanelDelegate {
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

            cls.add_method(
                sel!(windowDidBecomeKey:),
                Self::window_did_become_key as extern "C" fn(&Object, Sel, id),
            );

            cls.add_method(
                sel!(windowDidResignKey:),
                Self::window_did_resign_key as extern "C" fn(&Object, Sel, id),
            );
        }

        cls.register()
    }

    #[allow(non_snake_case)]
    extern "C" fn setPanel(this: &mut Object, _: Sel, panel: id) {
        unsafe { this.set_ivar("panel", panel) };
    }

    extern "C" fn window_did_become_key(_: &Object, _: Sel, _: id) {}

    /// Hide panel when it's no longer the key window
    extern "C" fn window_did_resign_key(this: &Object, _: Sel, _: id) {
        let panel: id = unsafe { *this.get_ivar("panel") };
        let _: () = unsafe { msg_send![panel, orderOut: nil] };
    }
}

unsafe impl Message for RawNSPanelDelegate {}

impl INSObject for RawNSPanelDelegate {
    fn class() -> &'static runtime::Class {
        Self::get_class()
    }
}

impl RawNSPanelDelegate {
    #[allow(dead_code)]
    pub fn set_panel(&self, panel: ShareId<RawNSPanel>) {
        let _: () = unsafe { msg_send![self, setPanel: panel] };
    }
}
