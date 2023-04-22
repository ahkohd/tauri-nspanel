use std::ffi::c_void;

use cocoa::{
    appkit::NSWindowCollectionBehavior,
    base::{id, nil, BOOL, NO, YES},
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
use tauri::{AppHandle, Manager, Runtime, Window};

extern "C" {
    pub fn object_setClass(obj: id, cls: id) -> id;
}

const CLS_NAME: &str = "RawNSPanel";

pub struct RawNSPanel;

unsafe impl Sync for RawNSPanel {}
unsafe impl Send for RawNSPanel {}

impl RawNSPanel {
    fn get_class() -> &'static Class {
        Class::get(CLS_NAME).unwrap_or_else(Self::define_class)
    }

    fn define_class() -> &'static Class {
        let mut cls = ClassDecl::new(CLS_NAME, class!(NSPanel))
            .unwrap_or_else(|| panic!("Unable to register {} class", CLS_NAME));

        unsafe {
            cls.add_ivar::<*mut c_void>("_app_handle");

            cls.add_method(
                sel!(setAppHandle:),
                Self::handle_set_app_handle as extern "C" fn(&mut Object, Sel, *mut c_void),
            );

            cls.add_method(
                sel!(appHandle),
                Self::handle_get_app_handle as extern "C" fn(&Object, Sel) -> *mut c_void,
            );

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

    extern "C" fn handle_set_app_handle(this: &mut Object, _: Sel, app_handle: *mut c_void) {
        unsafe { this.set_ivar("_app_handle", app_handle) };
    }

    extern "C" fn handle_get_app_handle(this: &Object, _: Sel) -> *mut c_void {
        unsafe { *this.get_ivar("_app_handle") }
    }

    /// Returns YES to ensure that RawNSPanel can become a key window
    extern "C" fn can_become_key_window(_: &Object, _: Sel) -> BOOL {
        YES
    }

    extern "C" fn dealloc(this: &mut Object, _cmd: Sel) {
        let app_handle_wrapper_ptr: *mut c_void = unsafe { *this.get_ivar("_app_handle") };

        if !app_handle_wrapper_ptr.is_null() {
            let app_handle_wrapper = unsafe { Box::from_raw(app_handle_wrapper_ptr) };
            drop(app_handle_wrapper);
        }

        unsafe {
            let superclass = class!(NSObject);
            let dealloc: extern "C" fn(&mut Object, Sel) =
                msg_send![super(this, superclass), dealloc];
            dealloc(this, _cmd);
        }
    }
}

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

    pub fn released_when_closed(&self, flag: bool) {
        let _: () = unsafe { msg_send![self, setReleasedWhenClosed: if flag {YES} else {NO}] };
    }

    pub fn close(&self) {
        let _: () = unsafe { msg_send![self, close] };
    }

    pub fn handle(&mut self) -> ShareId<Self> {
        unsafe { ShareId::from_ptr(self as *mut Self) }
    }

    fn set_app_handle<R: Runtime>(&self, app_handle: AppHandle<R>) {
        let handle = app_handle as _;
        let app_handle_wrapper = AppHandleWrapper::new(handle);
        let _: () = unsafe { msg_send![self, setAppHandle: app_handle_wrapper.into_raw()] };
    }

    pub fn app_handle<R: Runtime>(&self) -> Option<AppHandle<R>> {
        let wrapper_ptr: *mut c_void = unsafe { msg_send![self, appHandle] };

        if wrapper_ptr.is_null() {
            return None;
        }

        let wrapper = unsafe { &*(wrapper_ptr as *const AppHandleWrapper<R>) };
        Some(wrapper.app_handle.clone())
    }

    /// Create an NSPanel from a Tauri window
    pub fn from_window<R: Runtime>(window: Window<R>) -> Id<Self> {
        let app_handle = window.app_handle();
        let ns_window: id = window.ns_window().unwrap() as _;
        let ns_panel: id = unsafe { msg_send![Self::class(), class] };
        let raw_panel = unsafe {
            object_setClass(ns_window, ns_panel);
            Id::from_retained_ptr(ns_window as *mut RawNSPanel)
        };
        raw_panel.set_app_handle(app_handle);
        raw_panel
    }
}

unsafe impl Message for RawNSPanel {}

impl INSObject for RawNSPanel {
    fn class() -> &'static runtime::Class {
        RawNSPanel::get_class()
    }
}

pub struct AppHandleWrapper<R: Runtime> {
    app_handle: AppHandle<R>,
}

impl<R: Runtime> AppHandleWrapper<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self { app_handle }
    }

    pub fn into_raw(self) -> *mut c_void {
        Box::into_raw(Box::new(self)) as *mut c_void
    }
}
