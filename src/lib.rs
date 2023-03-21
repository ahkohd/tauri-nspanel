mod macros;
pub mod raw_nspanel;

use std::{collections::HashMap, sync::Mutex};

use cocoa::base::id;
use objc_id::{Id, ShareId};
use raw_nspanel::RawNSPanel;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, Window,
};

pub extern crate cocoa;
pub extern crate objc;
pub extern crate objc_foundation;
pub extern crate objc_id;

pub type Panel = Id<RawNSPanel>;

#[derive(Default)]
pub struct Store {
    panels: HashMap<String, ShareId<RawNSPanel>>,
}

#[derive(Default)]
pub struct PanelManager(pub Mutex<Store>);

pub trait ManagerExt<R: Runtime> {
    fn get_panel(&self, label: &str) -> Result<ShareId<RawNSPanel>, Error>;
}

#[derive(Debug)]
pub enum Error {
    PanelNotFound,
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn get_panel(&self, label: &str) -> Result<ShareId<RawNSPanel>, Error> {
        let manager = self.state::<self::PanelManager>();
        let manager = manager.0.lock().unwrap();

        match manager.panels.get(label) {
            Some(panel) => Ok(panel.to_owned()),
            None => Err(Error::PanelNotFound),
        }
    }
}

#[derive(Default)]
pub struct PanelConfig {
    pub delegate: Option<id>,
}

pub trait WindowExt {
    fn to_panel(&self) -> tauri::Result<ShareId<RawNSPanel>>;
}

impl<R: Runtime> WindowExt for Window<R> {
    fn to_panel(&self) -> tauri::Result<ShareId<RawNSPanel>> {
        let panel = RawNSPanel::from(self);
        let panel = panel.share();

        let manager = self.state::<self::PanelManager>();
        manager
            .0
            .lock()
            .unwrap()
            .panels
            .insert(self.label().into(), panel.clone());

        Ok(panel)
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nspanel")
        .setup(|app| {
            app.manage(self::PanelManager::default());
            Ok(())
        })
        .build()
}
