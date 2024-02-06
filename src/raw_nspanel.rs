use bitflags::bitflags;
use cocoa::{
    appkit::{NSView, NSViewHeightSizable, NSViewWidthSizable, NSWindowCollectionBehavior},
    base::{id, nil, BOOL, NO, YES},
    foundation::NSRect,
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{self, Class, Object, Sel},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;
use objc_id::{Id, ShareId};
use tauri::{Runtime, Window};

bitflags! {
    struct NSTrackingAreaOptions: u32 {
        const NSTrackingActiveAlways = 0x80;
        const NSTrackingMouseEnteredAndExited = 0x01;
        const NSTrackingMouseMoved = 0x02;
        const NSTrackingCursorUpdate = 0x04;
    }
}

extern "C" {
    pub fn object_setClass(obj: id, cls: id) -> id;
}

const CLS_NAME: &str = "RawNSPanel";

pub struct RawNSPanel;

unsafe impl Sync for RawNSPanel {}
unsafe impl Send for RawNSPanel {}

impl INSObject for RawNSPanel {
    fn class() -> &'static runtime::Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }
}

impl RawNSPanel {
    /// Returns YES to ensure that RawNSPanel can become a key window
    extern "C" fn can_become_key_window(_: &Object, _: Sel) -> BOOL {
        YES
    }

    extern "C" fn dealloc(this: &mut Object, _cmd: Sel) {
        unsafe {
            let superclass = class!(NSObject);
            let dealloc: extern "C" fn(&mut Object, Sel) =
                msg_send![super(this, superclass), dealloc];
            dealloc(this, _cmd);
        }
    }

    fn define_class() -> &'static Class {
        let mut cls = ClassDecl::new(CLS_NAME, class!(NSPanel))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        unsafe {
            cls.add_method(
                sel!(canBecomeKeyWindow),
                Self::can_become_key_window as extern "C" fn(&Object, Sel) -> BOOL,
            );

            cls.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel),
            );
        }

        cls.register()
    }

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

    pub fn resign_key_window(&self) {
        let _: () = unsafe { msg_send![self, resignKeyWindow] };
    }

    pub fn make_key_and_order_front(&self, sender: Option<id>) {
        let _: () = unsafe { msg_send![self, makeKeyAndOrderFront: sender.unwrap_or(nil)] };
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

    pub fn set_alpha_value(&self, value: f64) {
        let _: () = unsafe { msg_send![self, setAlphaValue: value] };
    }

    pub fn set_content_size(&self, width: f64, height: f64) {
        let _: () = unsafe { msg_send![self, setContentSize: (width, height)] };
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

    pub fn released_when_closed(&self, flag: bool) {
        let _: () = unsafe { msg_send![self, setReleasedWhenClosed: if flag {YES} else {NO}] };
    }

    pub fn close(&self) {
        let _: () = unsafe { msg_send![self, close] };
    }

    pub fn handle(&mut self) -> ShareId<Self> {
        unsafe { ShareId::from_ptr(self as *mut Self) }
    }

    fn add_tracking_area(&self) {
        let view: id = self.content_view();
        let bounds: NSRect = unsafe { NSView::bounds(view) };
        let track_view: id = unsafe { msg_send![class!(NSTrackingArea), alloc] };
        let track_view: id = unsafe {
            msg_send![
            track_view,
            initWithRect: bounds
            options: NSTrackingAreaOptions::NSTrackingActiveAlways
            | NSTrackingAreaOptions::NSTrackingMouseEnteredAndExited
            | NSTrackingAreaOptions::NSTrackingMouseMoved
            | NSTrackingAreaOptions::NSTrackingCursorUpdate
            owner: view
            userInfo: nil
            ]
        };
        let autoresizing_mask = NSViewWidthSizable | NSViewHeightSizable;
        let () = unsafe { msg_send![view, setAutoresizingMask: autoresizing_mask] };
        let () = unsafe { msg_send![view, addTrackingArea: track_view] };
    }

    /// Create an NSPanel from a Tauri window
    pub fn from_window<R: Runtime>(window: Window<R>) -> Id<Self> {
        let nswindow: id = window.ns_window().unwrap() as _;
        let nspanel_class: id = unsafe { msg_send![Self::class(), class] };
        unsafe {
            object_setClass(nswindow, nspanel_class);
            let panel = Id::from_retained_ptr(nswindow as *mut RawNSPanel);

            // Add a tracking area to the panel's content view,
            // so that we can receive mouse events such as mouseEntered and mouseExited
            panel.add_tracking_area();

            panel
        }
    }
}

unsafe impl Message for RawNSPanel {}
