use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    thread,
};

use native_windows_derive as nwd;
use native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;

use crate::{control_panel::ControlPanel, models::Profile, store, watchdog::WatchDog};

use keypad::*;

const ICON: &[u8] = include_bytes!("../resources/keycap.ico");

#[derive(Default, NwgUi)]
pub struct KeypadTray {
    #[nwg_control]
    #[nwg_events(OnInit: [KeypadTray::on_init(RC_SELF)])]
    window: nwg::MessageWindow,

    #[nwg_resource(source_bin: Some(ICON))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("Keypad"))]
    #[nwg_events(MousePressLeftUp: [KeypadTray::show_menu], OnContextMenu: [KeypadTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Profiles")]
    profiles_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu)]
    top_separator: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Control Panel")]
    #[nwg_events(OnMenuItemSelected: [KeypadTray::open_control_panel])]
    control_panel: nwg::MenuItem,

    #[nwg_control(parent: tray_menu)]
    exit_separator: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [KeypadTray::exit])]
    exit: nwg::MenuItem,

    profile_menu_items: RefCell<Vec<nwg::MenuItem>>,

    profile_menu_handler: RefCell<Option<nwg::EventHandler>>,

    editor_data: RefCell<Option<thread::JoinHandle<Vec<Profile>>>>,

    #[nwg_control]
    #[nwg_events( OnNotice: [KeypadTray::save_updated_profiles] )]
    editor_notice: nwg::Notice,

    profiles: RefCell<Vec<Profile>>,

    watchdog: RefCell<Option<WatchDog>>,

    #[nwg_control]
    #[nwg_events( OnNotice: [KeypadTray::watchdog_notice_received] )]
    watchdog_notice: nwg::Notice,

    pub restart_tray: Cell<bool>,
}

impl KeypadTray {
    fn on_init(tray_rc: &Rc<KeypadTray>) {
        KeypadTray::setup_profile_handler(tray_rc);
        KeypadTray::load_profiles(tray_rc);
        KeypadTray::start_watchdog(tray_rc);
    }

    pub fn load_profiles(&self) {
        {
            let mut profiles = self.profiles.borrow_mut();
            *profiles = store::load_profiles().unwrap();

            for profile in profiles.iter() {
                self.add_profile_menu_item(profile);
            }
        }

        let idx = self.read_selected_profile();
        self.show_selected_profile(idx);
    }

    pub fn start_watchdog(&self) {
        let mut watchdog = self.watchdog.borrow_mut();
        let notice = self.watchdog_notice.sender();
        *watchdog = Some(WatchDog::start(store::load_profiles().unwrap(), notice));
    }

    fn watchdog_notice_received(&self) {
        let idx = self.read_selected_profile();
        if let Some(idx) = idx.clone() {
            let flags =
                nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            self.tray.show(
                &format!("{}", self.profiles.borrow()[idx]),
                Some("Keypad profile changed"),
                Some(flags),
                Some(&self.icon),
            );
        }

        self.show_selected_profile(idx);
    }

    fn read_selected_profile(&self) -> Option<usize> {
        let profiles = self.profiles.borrow();
        let result = Keypad::auto_detect().and_then(|mut k| k.get_combos_from_device());
        match result {
            Ok(combos) => {
                let matching_idx = profiles
                    .iter()
                    .enumerate()
                    .find(|(_, p)| p.combos == combos)
                    .map(|(i, _)| i);
                return matching_idx;
            }
            Err(e) => {
                let flags =
                    nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    &format!("{}", e),
                    Some("Unable to read from device"),
                    Some(flags),
                    Some(&self.icon),
                );
            }
        }
        return None;
    }

    fn setup_profile_handler(tray_rc: &Rc<KeypadTray>) {
        let tray = Rc::clone(tray_rc);
        let handler =
            nwg::full_bind_event_handler(&tray_rc.window.handle, move |evt, _evt_data, handle| {
                match evt {
                    nwg::Event::OnMenuItemSelected => {
                        let items = tray.profile_menu_items.borrow();
                        for (idx, item) in items.iter().enumerate() {
                            if item.handle == handle {
                                tray.apply_profile(idx);
                            }
                        }
                    }
                    _ => {}
                }
            });

        *tray_rc.profile_menu_handler.borrow_mut() = Some(handler);
    }

    fn add_profile_menu_item(&self, profile: &Profile) {
        let mut item = Default::default();
        nwg::MenuItem::builder()
            .text(&profile.name)
            .parent(&self.profiles_menu)
            .build(&mut item)
            .expect("Failed to build profile menu item");

        let mut items = self.profile_menu_items.borrow_mut();
        items.push(item);
    }

    fn apply_profile(&self, idx: usize) {
        let profiles = self.profiles.borrow();
        let profile = profiles[idx].clone();
        let result =
            Keypad::auto_detect().and_then(|mut k| k.send_combos_to_device(profile.combos));
        if let Err(e) = result {
            let flags =
                nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
            self.tray.show(
                &format!("{}", e),
                Some("Error applying profile"),
                Some(flags),
                Some(&self.icon),
            );
            return;
        }
        self.show_selected_profile(Some(idx));
    }

    fn show_selected_profile(&self, idx: Option<usize>) {
        let items = self.profile_menu_items.borrow();
        for (i, item) in items.iter().enumerate() {
            let checked = match idx {
                Some(idx) => idx == i,
                None => false,
            };
            item.set_checked(checked);
        }
    }

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn open_control_panel(&self) {
        let mut handle = self.editor_data.borrow_mut();
        if handle.is_some() {
            return;
        }

        let notice = self.editor_notice.sender();
        let profiles = self.profiles.borrow();
        let profiles: Vec<_> = profiles.iter().map(Profile::clone).collect();

        *handle = Some(thread::spawn(move || {
            nwg::init().unwrap();
            let panel = ControlPanel::new(profiles);
            let ui = ControlPanel::build_ui(panel).expect("Failed to build control panel UI");
            nwg::dispatch_thread_events();

            notice.notice();
            {
                let profiles = ui.profiles.borrow();
                profiles.clone()
            }
        }))
    }

    fn save_updated_profiles(&self) {
        let mut data = self.editor_data.borrow_mut();
        if let Some(handle) = data.take() {
            let updated_profiles = handle.join().unwrap();
            if let Err(e) = store::store_profiles(&updated_profiles) {
                let flags =
                    nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
                self.tray.show(
                    &format!("{}", e),
                    Some("Error saving profiles"),
                    Some(flags),
                    Some(&self.icon),
                );
                return;
            }

            self.update_profiles_or_restart_if_required(updated_profiles);
            let idx = self.read_selected_profile();
            self.show_selected_profile(idx);
        }
    }

    fn update_profiles_or_restart_if_required(&self, updated_profiles: Vec<Profile>) {
        let mut tray_profiles = self.profiles.borrow_mut();
        if tray_profiles.len() != updated_profiles.len() {
            self.restart();
            return;
        }
        for (idx, profile) in updated_profiles.iter().enumerate() {
            if Self::profile_updated(profile, &tray_profiles[idx]) {
                self.restart();
                return;
            }
        }

        *tray_profiles = updated_profiles;
    }

    fn profile_updated(lhs: &Profile, rhs: &Profile) -> bool {
        lhs.name != rhs.name || lhs.auto_launch_program != rhs.auto_launch_program
    }

    fn restart(&self) {
        self.restart_tray.set(true);
        self.exit();
    }

    fn exit(&self) {
        let mut maybe_handle = self.profile_menu_handler.borrow_mut();
        if let Some(handle) = maybe_handle.take() {
            nwg::unbind_event_handler(&handle);
        }
        self.stop_watchdog();
        nwg::stop_thread_dispatch();
    }

    fn stop_watchdog(&self) {
        let mut watchdog = self.watchdog.borrow_mut();
        if let Some(watchdog) = watchdog.take() {
            watchdog.stop();
        }
    }
}
