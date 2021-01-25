use std::{cell::RefCell, cmp::min, thread};

use native_windows_derive as nwd;
use native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

use keypad::Keypad;
use thread::JoinHandle;

use crate::{models::Profile, profile_editor::KeypadEditor};

const UP_PNG: &[u8] = include_bytes!("../resources/up.png");
const DOWN_PNG: &[u8] = include_bytes!("../resources/down.png");
const DELETE_PNG: &[u8] = include_bytes!("../resources/delete.png");
const SUCCESS_PNG: &[u8] = include_bytes!("../resources/success.png");

#[derive(Default, NwgUi)]
pub struct ControlPanel {
    #[nwg_control(size: (300, 500), position: (1150, 450), title: "Keypad Control Panel")]
    #[nwg_events( OnWindowClose: [ControlPanel::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, min_size: [300, 500])]
    layout: nwg::GridLayout,

    #[nwg_control(collection: data.profile_labels(), selected_index: data.selected_index())]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 5)]
    #[nwg_events(OnListBoxSelect: [ControlPanel::profile_selected],
        OnListBoxDoubleClick: [ControlPanel::edit_profile])]
    menu: nwg::ListBox<String>,

    #[nwg_resource(source_bin: Some(UP_PNG))]
    up_icon: nwg::Bitmap,

    #[nwg_resource(source_bin: Some(DOWN_PNG))]
    down_icon: nwg::Bitmap,

    #[nwg_resource(source_bin: Some(DELETE_PNG))]
    delete_icon: nwg::Bitmap,

    #[nwg_resource(source_bin: Some(SUCCESS_PNG))]
    success_icon: nwg::Bitmap,

    #[nwg_control(text: " ", bitmap: Some(&data.up_icon))]
    #[nwg_layout_item(layout: layout, col: 6, col_span: 1, row: 0)]
    #[nwg_events(OnButtonClick: [ControlPanel::move_up])]
    move_up: nwg::Button,

    #[nwg_control(text: " ", bitmap: Some(&data.down_icon))]
    #[nwg_layout_item(layout: layout, col: 6, col_span: 1, row: 1)]
    #[nwg_events(OnButtonClick: [ControlPanel::move_down])]
    move_down: nwg::Button,

    #[nwg_control(text: " ", bitmap: Some(&data.delete_icon))]
    #[nwg_layout_item(layout: layout, col: 6, col_span: 1, row: 2)]
    #[nwg_events(OnButtonClick: [ControlPanel::remove])]
    delete: nwg::Button,

    #[nwg_control(text: "New Profile")]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 3, row: 5)]
    #[nwg_events(OnButtonClick: [ControlPanel::new_profile])]
    new_profile: nwg::Button,

    #[nwg_control(text: "Edit Profile")]
    #[nwg_layout_item(layout: layout, col: 3, col_span: 3, row: 5)]
    #[nwg_events(OnButtonClick: [ControlPanel::edit_profile])]
    edit_profile: nwg::Button,

    #[nwg_control(text: "Apply Profile")]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 6)]
    #[nwg_events(OnButtonClick: [ControlPanel::apply_profile])]
    apply_profile: nwg::Button,

    #[nwg_control(text: &data.preview_text(), readonly: true)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 7, row: 7, row_span: 4)]
    preview: nwg::TextBox,

    #[nwg_control(text: "Read from Keypad")]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 11)]
    #[nwg_events(OnButtonClick: [ControlPanel::read_from_keypad])]
    read_profile: nwg::Button,

    #[nwg_control(size: (32, 32), bitmap: Some(&data.success_icon), flags: "DISABLED")]
    #[nwg_layout_item(layout: layout, col: 6, row: 11)]
    success_frame: nwg::ImageFrame,

    editor_data: RefCell<Option<JoinHandle<(Option<usize>, Option<Profile>)>>>,

    #[nwg_control]
    #[nwg_events( OnNotice: [ControlPanel::editor_closed] )]
    editor_notice: nwg::Notice,

    pub profiles: RefCell<Vec<Profile>>,
}

impl ControlPanel {
    pub fn new(profiles: Vec<Profile>) -> Self {
        Self {
            profiles: RefCell::new(profiles),
            ..Default::default()
        }
    }

    fn profile_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = self
            .profiles
            .borrow()
            .iter()
            .map(|p| format!("{}", p))
            .collect();
        if labels.len() > 0 {
            labels[0].push_str(" (default)");
        }
        labels
    }

    fn selected_index(&self) -> Option<usize> {
        match self.profiles.borrow().len() {
            0 => None,
            _ => Some(0),
        }
    }

    fn move_up(&self) {
        if let Some(idx) = self.menu.selection() {
            if idx > 0 {
                {
                    let mut profiles = self.profiles.borrow_mut();
                    let item = profiles.remove(idx);
                    profiles.insert(idx - 1, item);
                }
                self.menu.set_collection(self.profile_labels());
                self.set_selection(Some(idx - 1));
            }
        }
    }

    fn move_down(&self) {
        if let Some(idx) = self.menu.selection() {
            if self.menu.len() > 1 && idx < self.menu.len() - 1 {
                {
                    let mut profiles = self.profiles.borrow_mut();
                    let item = profiles.remove(idx);
                    profiles.insert(idx + 1, item);
                }
                self.menu.set_collection(self.profile_labels());
                self.set_selection(Some(idx + 1));
            }
        }
    }

    fn remove(&self) {
        if let Some(idx) = self.menu.selection() {
            {
                let mut profiles = self.profiles.borrow_mut();
                let name = &profiles[idx].name;
                if !self.make_sure(
                    "Confirm",
                    &format!("Are you sure you want to delete the profile '{}'?", name),
                ) {
                    return;
                }
                profiles.remove(idx);
            }
            self.menu.set_collection(self.profile_labels());
            if self.menu.len() > 0 {
                let idx = min(idx, self.menu.len() - 1);
                self.set_selection(Some(idx));
            }
        }
    }

    fn make_sure(&self, title: &str, content: &str) -> bool {
        let params = nwg::MessageParams {
            title,
            content,
            buttons: nwg::MessageButtons::YesNo,
            icons: nwg::MessageIcons::Question,
        };
        match nwg::modal_message(self.window.handle, &params) {
            nwg::MessageChoice::Yes => true,
            _ => false,
        }
    }

    fn set_selection(&self, idx: Option<usize>) {
        self.menu.set_selection(idx);
        self.profile_selected();
    }

    fn profile_selected(&self) {
        if let Some(idx) = self.menu.selection() {
            let profile = &self.profiles.borrow()[idx];
            self.preview_profile(profile);
        }
    }

    fn preview_text(&self) -> String {
        match self.selected_index() {
            Some(idx) => {
                let profile = &self.profiles.borrow()[idx];
                get_preview_text(profile)
            }
            None => "Select a profile...".into(),
        }
    }

    fn preview_profile(&self, profile: &Profile) {
        self.success_icon(false);
        let text = get_preview_text(profile);
        self.preview.set_text(&text);
    }

    fn new_profile(&self) {
        self.open_editor(None, Profile::default());
    }

    fn edit_profile(&self) {
        if let Some(idx) = self.menu.selection() {
            let profile = self.profiles.borrow()[idx].clone();
            self.open_editor(Some(idx), profile);
        }
    }

    fn open_editor(&self, idx: Option<usize>, profile: Profile) {
        let mut handle = self.editor_data.borrow_mut();
        if handle.is_some() {
            return;
        }

        let notice = self.editor_notice.sender();

        *handle = Some(thread::spawn(move || {
            let editor = KeypadEditor::new(profile);
            let ui = KeypadEditor::build_ui(editor).expect("Failed to build editor UI");
            nwg::dispatch_thread_events();

            notice.notice();
            {
                let profile = ui.profile_new.borrow_mut().take();
                (idx, profile)
            }
        }));
    }

    fn editor_closed(&self) {
        let mut data = self.editor_data.borrow_mut();
        if let Some(handle) = data.take() {
            let returned = handle.join().unwrap();
            if let Some(profile) = returned.1 {
                if let Some(idx) = returned.0 {
                    let mut profiles = self.profiles.borrow_mut();
                    profiles.remove(idx);
                    profiles.insert(idx, profile);
                } else {
                    let mut profiles = self.profiles.borrow_mut();
                    profiles.push(profile.clone());
                }
                self.menu.set_collection(self.profile_labels());
                let select = returned.0.or(Some(self.menu.len() - 1));
                self.set_selection(select);
            }
        };
    }

    fn apply_profile(&self) {
        if !self.menu.selection().is_some() {
            return;
        }
        let idx = self.menu.selection().unwrap();
        let profile = self.profiles.borrow()[idx].clone();
        match Keypad::auto_detect().and_then(|mut k| k.send_combos_to_device(profile.combos)) {
            Ok(_) => self.success_icon(true),
            Err(e) => {
                nwg::simple_message("Error", &format!("Error: {}", e));
            }
        };
    }

    fn read_from_keypad(&self) {
        match Keypad::auto_detect().and_then(|mut k| k.get_combos_from_device()) {
            Ok(combos) => {
                let dummy = Profile {
                    combos,
                    ..Default::default()
                };
                self.preview_profile(&dummy);
                self.set_selection(None);
                self.success_icon(true);
            }
            Err(e) => {
                nwg::modal_error_message(self.window.handle, "Error", &format!("Error: {}", e));
            }
        };
    }

    fn success_icon(&self, show: bool) {
        self.success_frame.set_visible(show);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn get_preview_text(profile: &Profile) -> String {
    let combos: Vec<_> = profile.combos.iter().map(|c| format!("{}", c)).collect();
    combos.join("\r\n")
}
