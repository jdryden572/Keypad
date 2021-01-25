use std::cell::RefCell;

use native_windows_derive as nwd;
use native_windows_gui as nwg;

use nwd::{NwgPartial, NwgUi};
use nwg::{CheckBoxState, GridLayoutItem};

use keypad::{Key, KeyCombo, KeyPress};

use crate::models::Profile;

#[derive(Default, NwgUi)]
pub struct KeypadEditor {
    #[nwg_control(size: (850, 300), title: "Keypad Profile Editor")]
    #[nwg_events( OnWindowClose: [KeypadEditor::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, min_size: [300, 300])]
    layout: nwg::GridLayout,

    #[nwg_control(text: &data.name())]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 3, row: 0)]
    name: nwg::TextInput,

    #[nwg_control(text: "Auto", check_state: data.use_auto_checkstate())]
    #[nwg_layout_item(layout: layout, col: 0, row: 1)]
    #[nwg_events( OnButtonClick: [KeypadEditor::auto_program_toggled] )]
    use_auto: nwg::CheckBox,

    #[nwg_control(text: &data.program(), readonly: !data.enable_auto())]
    #[nwg_layout_item(layout: layout, col: 1, col_span: 2, row: 1)]
    auto_program: nwg::TextInput,

    #[nwg_control(collection: data.labels())]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 3, row: 2, row_span: 3)]
    #[nwg_events( OnListBoxSelect: [KeypadEditor::select_key] )]
    menu: nwg::ListBox<String>,

    #[nwg_control(text: "Save")]
    #[nwg_layout_item(layout: layout, col: 0, row: 5, col_span: 2)]
    #[nwg_events(OnButtonClick: [KeypadEditor::save_clicked])]
    save_button: nwg::Button,

    #[nwg_control]
    #[nwg_layout_item(layout: layout, col: 3, col_span: 4, row: 0, row_span: 6)]
    frame1: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    frame2: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    frame3: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    frame4: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    frame5: nwg::Frame,

    #[nwg_control(flags: "BORDER")]
    frame6: nwg::Frame,

    #[nwg_partial(parent: frame1)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key1: KeyComboPartial,

    #[nwg_partial(parent: frame2)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key2: KeyComboPartial,

    #[nwg_partial(parent: frame3)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key3: KeyComboPartial,

    #[nwg_partial(parent: frame4)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key4: KeyComboPartial,

    #[nwg_partial(parent: frame5)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key5: KeyComboPartial,

    #[nwg_partial(parent: frame6)]
    #[nwg_events( (apply_btn, OnButtonClick): [KeypadEditor::apply_combo] )]
    key6: KeyComboPartial,

    profile: RefCell<Profile>,

    pub profile_new: RefCell<Option<Profile>>,
}

impl KeypadEditor {
    fn name(&self) -> String {
        self.profile.borrow().name.clone()
    }

    fn use_auto_checkstate(&self) -> CheckBoxState {
        bool_to_checkbox(self.enable_auto())
    }

    fn enable_auto(&self) -> bool {
        self.profile.borrow().auto_launch_program.is_some()
    }

    fn auto_program_toggled(&self) {
        let enabled = checkbox_to_bool(self.use_auto.check_state());
        self.auto_program.set_readonly(!enabled);
        if !enabled {
            self.auto_program.set_text("");
        }
    }

    fn program(&self) -> String {
        self.profile
            .borrow()
            .auto_launch_program
            .clone()
            .unwrap_or_else(|| String::new())
    }

    fn labels(&self) -> Vec<String> {
        self.profile
            .borrow()
            .combos
            .iter()
            .map(|combo| format!("{}", combo))
            .collect()
    }

    fn select_key(&self) {
        self.frame1.set_visible(false);
        self.frame2.set_visible(false);
        self.frame3.set_visible(false);
        self.frame4.set_visible(false);
        self.frame5.set_visible(false);
        self.frame6.set_visible(false);

        let layout = &self.layout;
        if layout.has_child(&self.frame1) {
            layout.remove_child(&self.frame1);
        }
        if layout.has_child(&self.frame2) {
            layout.remove_child(&self.frame2);
        }
        if layout.has_child(&self.frame3) {
            layout.remove_child(&self.frame3);
        }
        if layout.has_child(&self.frame4) {
            layout.remove_child(&self.frame4);
        }
        if layout.has_child(&self.frame5) {
            layout.remove_child(&self.frame5);
        }
        if layout.has_child(&self.frame6) {
            layout.remove_child(&self.frame6);
        }

        match self.menu.selection() {
            None | Some(0) => {
                let child = GridLayoutItem::new(&self.frame1, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame1.set_visible(true);
            }
            Some(1) => {
                let child = GridLayoutItem::new(&self.frame2, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame2.set_visible(true);
            }
            Some(2) => {
                let child = GridLayoutItem::new(&self.frame3, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame3.set_visible(true);
            }
            Some(3) => {
                let child = GridLayoutItem::new(&self.frame4, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame4.set_visible(true);
            }
            Some(4) => {
                let child = GridLayoutItem::new(&self.frame5, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame5.set_visible(true);
            }
            Some(5) => {
                let child = GridLayoutItem::new(&self.frame6, 3, 0, 4, 6);
                layout.add_child_item(child);
                self.frame6.set_visible(true);
            }
            Some(_) => unreachable!(),
        }
    }

    fn apply_combo(&self) {
        self.update_combo_and_label(&self.key1, 0);
        self.update_combo_and_label(&self.key2, 1);
        self.update_combo_and_label(&self.key3, 2);
        self.update_combo_and_label(&self.key4, 3);
        self.update_combo_and_label(&self.key5, 4);
        self.update_combo_and_label(&self.key6, 5);
    }

    fn update_combo_and_label(&self, key_partial: &KeyComboPartial, idx: usize) {
        let combo = key_partial.combo.borrow();
        let new_label = format!("{}", combo);
        {
            let mut labels = self.menu.collection_mut();
            labels[idx] = new_label;
        }
        self.menu.sync();

        {
            let mut profile = self.profile.borrow_mut();
            profile.combos[idx] = combo.clone();
        }
    }

    fn save_clicked(&self) {
        if self.name.text().trim().len() == 0 {
            nwg::error_message("Name required", "Please enter a name for the profile.");
            return;
        }

        {
            let mut profile = self.profile.borrow_mut();
            profile.name = self.name.text();
            profile.auto_launch_program = match checkbox_to_bool(self.use_auto.check_state()) {
                true => Some(self.auto_program.text()),
                false => None,
            }
        }
        let mut new = self.profile_new.borrow_mut();
        *new = Some(self.profile.borrow().clone());
        nwg::stop_thread_dispatch();
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn new(profile: Profile) -> Self {
        Self {
            key1: KeyComboPartial::new(profile.combos[0].clone()),
            key2: KeyComboPartial::new(profile.combos[1].clone()),
            key3: KeyComboPartial::new(profile.combos[2].clone()),
            key4: KeyComboPartial::new(profile.combos[3].clone()),
            key5: KeyComboPartial::new(profile.combos[4].clone()),
            key6: KeyComboPartial::new(profile.combos[5].clone()),
            profile: RefCell::new(profile),
            ..Default::default()
        }
    }
}

#[derive(Default, NwgPartial)]
pub struct KeyComboPartial {
    #[nwg_layout(min_size: [100, 120])]
    layout: nwg::GridLayout,

    #[nwg_control(text: "Ctrl", check_state: data.ctrl1())]
    #[nwg_layout_item(layout: layout, col: 0, row: 2, row_span: 2)]
    ctrl1: nwg::CheckBox,

    #[nwg_control(text: "Alt", check_state: data.alt1())]
    #[nwg_layout_item(layout: layout, col: 0, row: 4, row_span: 2)]
    alt1: nwg::CheckBox,

    #[nwg_control(text: "Shift", check_state: data.shift1())]
    #[nwg_layout_item(layout: layout, col: 0, row: 6, row_span: 2)]
    shift1: nwg::CheckBox,

    #[nwg_control(text: "Windows", check_state: data.windows1())]
    #[nwg_layout_item(layout: layout, col: 0, row: 8, row_span: 2)]
    windows1: nwg::CheckBox,

    #[nwg_control(text: "Press 1")]
    #[nwg_layout_item(layout: layout, col: 1, row: 0, row_span: 1)]
    label1: nwg::Label,

    #[nwg_control(
        collection: Key::ALL.iter().map(|k| format!("{}", k)).collect(),
        selected_index: data.key1_idx()
    )]
    #[nwg_layout_item(layout: layout, col: 1, row: 1, row_span: 19)]
    key1: nwg::ListBox<String>,

    #[nwg_control(text: "Press 2", check_state: data.key2_checkbox())]
    #[nwg_layout_item(layout: layout, col: 2, row: 0, row_span: 2)]
    #[nwg_events(OnButtonClick: [KeyComboPartial::key2_checkbox_clicked])]
    enable_key2: nwg::CheckBox,

    #[nwg_control(text: "Ctrl", check_state: data.ctrl2(), enabled: data.has_key2())]
    #[nwg_layout_item(layout: layout, col: 2, row: 2, row_span: 2)]
    ctrl2: nwg::CheckBox,

    #[nwg_control(text: "Alt", check_state: data.alt2(), enabled: data.has_key2())]
    #[nwg_layout_item(layout: layout, col: 2, row: 4, row_span: 2)]
    alt2: nwg::CheckBox,

    #[nwg_control(text: "Shift", check_state: data.shift2(), enabled: data.has_key2())]
    #[nwg_layout_item(layout: layout, col: 2, row: 6, row_span: 2)]
    shift2: nwg::CheckBox,

    #[nwg_control(text: "Windows", check_state: data.windows2(), enabled: data.has_key2())]
    #[nwg_layout_item(layout: layout, col: 2, row: 8, row_span: 2)]
    windows2: nwg::CheckBox,

    #[nwg_control(
        collection: Key::ALL.iter().map(|k| format!("{}", k)).collect(),
        selected_index: data.key2_idx(),
        enabled: data.has_key2(),
    )]
    #[nwg_layout_item(layout: layout, col: 3, row: 1, row_span: 19)]
    key2: nwg::ListBox<String>,

    #[nwg_control(text: "Apply")]
    #[nwg_layout_item(layout: layout, col: 0, row: 17, row_span: 3)]
    #[nwg_events(OnButtonClick: [KeyComboPartial::save_clicked])]
    apply_btn: nwg::Button,

    combo: RefCell<KeyCombo>,
}

impl KeyComboPartial {
    pub fn new(combo: KeyCombo) -> Self {
        Self {
            combo: RefCell::new(combo),
            ..Default::default()
        }
    }

    fn ctrl1(&self) -> CheckBoxState {
        bool_to_checkbox(self.combo.borrow().one.ctrl)
    }

    fn alt1(&self) -> CheckBoxState {
        bool_to_checkbox(self.combo.borrow().one.alt)
    }

    fn shift1(&self) -> CheckBoxState {
        bool_to_checkbox(self.combo.borrow().one.shift)
    }

    fn windows1(&self) -> CheckBoxState {
        bool_to_checkbox(self.combo.borrow().one.windows)
    }

    fn key1_idx(&self) -> Option<usize> {
        Key::ALL
            .iter()
            .enumerate()
            .find(|(_, &k)| k == self.combo.borrow().one.key)
            .map(|(i, _)| i)
    }

    fn key2_checkbox(&self) -> CheckBoxState {
        match self.combo.borrow().two {
            Some(_) => CheckBoxState::Checked,
            None => CheckBoxState::Unchecked,
        }
    }

    fn key2_checkbox_clicked(&self) {
        let enable = match self.enable_key2.check_state() {
            CheckBoxState::Checked => true,
            _ => false,
        };
        self.ctrl2.set_enabled(enable);
        self.alt2.set_enabled(enable);
        self.shift2.set_enabled(enable);
        self.windows2.set_enabled(enable);
        self.key2.set_enabled(enable);
    }

    fn has_key2(&self) -> bool {
        self.combo.borrow().two.is_some()
    }

    fn ctrl2(&self) -> CheckBoxState {
        option_to_checkbox(self.combo.borrow().two.as_ref().map(|p| p.ctrl))
    }

    fn alt2(&self) -> CheckBoxState {
        option_to_checkbox(self.combo.borrow().two.as_ref().map(|p| p.alt))
    }

    fn shift2(&self) -> CheckBoxState {
        option_to_checkbox(self.combo.borrow().two.as_ref().map(|p| p.shift))
    }

    fn windows2(&self) -> CheckBoxState {
        option_to_checkbox(self.combo.borrow().two.as_ref().map(|p| p.windows))
    }

    fn key2_idx(&self) -> Option<usize> {
        match self.combo.borrow().two.as_ref() {
            None => None,
            Some(press) => Key::ALL
                .iter()
                .enumerate()
                .find(|(_, &k)| k == press.key)
                .map(|(i, _)| i),
        }
    }

    fn save_clicked(&self) {
        let key1 = match self.key1.selection() {
            Some(idx) => index_to_key(idx),
            None => {
                nwg::simple_message("Invalid key combo", "Please select a key!");
                return;
            }
        };
        let one = KeyPress {
            ctrl: checkbox_to_bool(self.ctrl1.check_state()),
            alt: checkbox_to_bool(self.alt1.check_state()),
            shift: checkbox_to_bool(self.shift1.check_state()),
            windows: checkbox_to_bool(self.windows1.check_state()),
            key: key1,
        };

        let two = if checkbox_to_bool(self.enable_key2.check_state()) {
            let key2 = match self.key2.selection() {
                Some(idx) => index_to_key(idx),
                None => {
                    nwg::simple_message("Invalid key combo", "Please select a key!");
                    return;
                }
            };
            Some(KeyPress {
                ctrl: checkbox_to_bool(self.ctrl2.check_state()),
                alt: checkbox_to_bool(self.alt2.check_state()),
                shift: checkbox_to_bool(self.shift2.check_state()),
                windows: checkbox_to_bool(self.windows2.check_state()),
                key: key2,
            })
        } else {
            None
        };

        self.combo.replace(KeyCombo { one, two });
    }
}

fn bool_to_checkbox(val: bool) -> CheckBoxState {
    match val {
        true => CheckBoxState::Checked,
        false => CheckBoxState::Unchecked,
    }
}

fn checkbox_to_bool(checkbox: CheckBoxState) -> bool {
    checkbox == CheckBoxState::Checked
}

fn option_to_checkbox(val: Option<bool>) -> CheckBoxState {
    match val {
        Some(true) => CheckBoxState::Checked,
        _ => CheckBoxState::Unchecked,
    }
}

fn index_to_key(idx: usize) -> Key {
    Key::ALL[idx].clone()
}
