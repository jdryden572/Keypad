#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use nwg::NativeUi;

mod control_panel;
mod models;
mod profile_editor;
mod store;
mod tray;
mod watchdog;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    loop {
        let ui = tray::KeypadTray::build_ui(Default::default()).expect("Failed to build UI");
        nwg::dispatch_thread_events();

        if !ui.restart_tray.get() {
            break;
        }
    }
}
