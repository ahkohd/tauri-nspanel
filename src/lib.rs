mod macros;
pub mod raw_nspanel;

use std::{collections::HashMap, sync::Mutex};

use cocoa::base::id;
use objc_id::ShareId;
use raw_nspanel::RawNSPanel;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, Window,
};

pub extern crate cocoa;
pub extern crate objc;
pub extern crate objc_foundation;
pub extern crate objc_id;
pub extern crate tauri;

pub type Panel<R> = ShareId<RawNSPanel<R>>;

pub struct Store<R: Runtime> {
    panels: HashMap<String, ShareId<RawNSPanel<R>>>,
}

impl<R: Runtime> Default for Store<R> {
    fn default() -> Self {
        Self {
            panels: HashMap::new(),
        }
    }
}

pub struct PanelManager<R: Runtime>(pub Mutex<Store<R>>);

impl<R: Runtime> Default for PanelManager<R> {
    fn default() -> Self {
        Self(Mutex::new(Store::<R>::default()))
    }
}

pub trait ManagerExt<R: Runtime> {
    fn get_panel(&self, label: &str) -> Result<ShareId<RawNSPanel<R>>, Error>;
}

#[derive(Debug)]
pub enum Error {
    PanelNotFound,
}

impl<R: Runtime, T: Manager<R>> ManagerExt<R> for T {
    fn get_panel(&self, label: &str) -> Result<ShareId<RawNSPanel<R>>, Error> {
        let manager = self.state::<self::PanelManager<R>>();
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

pub trait WindowExt<R: Runtime> {
    fn to_panel(&self) -> tauri::Result<ShareId<RawNSPanel<R>>>;
}

impl<R: Runtime> WindowExt<R> for Window<R> {
    fn to_panel(&self) -> tauri::Result<ShareId<RawNSPanel<R>>> {
        let panel = RawNSPanel::from(self).share();

        let manager = self.state::<self::PanelManager<R>>();
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
            app.manage(self::PanelManager::<R>::default());
            Ok(())
        })
        .build()
}
