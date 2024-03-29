use std::fmt::{Display, Formatter};

use enum_primitive::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct KeyCombo {
    pub one: KeyPress,
    pub two: Option<KeyPress>,
}

impl Display for KeyCombo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.one)?;
        if let Some(two) = &self.two {
            write!(f, ", {}", two)?;
        }
        Ok(())
    }
}

impl Default for KeyCombo {
    fn default() -> Self {
        Self {
            one: KeyPress::key(Key::A),
            two: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct KeyPress {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub windows: bool,
    pub key: Key,
}

impl KeyPress {
    pub fn key(key: Key) -> Self {
        Self {
            ctrl: false,
            alt: false,
            shift: false,
            windows: false,
            key
        }
    }

    pub fn ctrl() -> KeyPressBuilder {
        KeyPressBuilder {
            ctrl: true,
            ..Default::default()
        }
    }

    pub fn alt() -> KeyPressBuilder {
        KeyPressBuilder {
            alt: true,
            ..Default::default()
        }
    }

    pub fn shift() -> KeyPressBuilder {
        KeyPressBuilder {
            shift: true,
            ..Default::default()
        }
    }

    pub fn windows() -> KeyPressBuilder {
        KeyPressBuilder {
            windows: true,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct KeyPressBuilder {
    ctrl: bool,
    alt: bool,
    shift: bool,
    windows: bool,
}

impl KeyPressBuilder {
    pub fn ctrl(mut self) -> KeyPressBuilder {
        self.ctrl = true;
        self
    }

    pub fn alt(mut self) -> KeyPressBuilder {
        self.alt = true;
        self
    }

    pub fn shift(mut self) -> KeyPressBuilder {
        self.shift = true;
        self
    }

    pub fn windows(mut self) -> KeyPressBuilder {
        self.windows = true;
        self
    }

    pub fn key(self, key: Key) -> KeyPress {
        KeyPress {
            ctrl: self.ctrl,
            alt: self.alt,
            shift: self.shift,
            windows: self.windows,
            key
        }
    }
}

impl Display for KeyPress {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.ctrl {
            write!(f, "Ctrl + ")?;
        }
        if self.alt {
            write!(f, "Alt + ")?;
        }
        if self.shift {
            write!(f, "Shift + ")?;
        }
        if self.windows {
            write!(f, "Win + ")?;
        }
        write!(f, "{}", self.key)
    }
}

enum_from_primitive! {
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ModifierKey {
    LeftCtrl = 0x01 | 0xE000,
    LeftShift = 0x02 | 0xE000,
    LeftAlt = 0x04 | 0xE000,
    LeftGui = 0x08 | 0xE000,
    RightCtrl = 0x10 | 0xE000,
    RightShift = 0x20 | 0xE000,
    RightAlt = 0x40 | 0xE000,
    RightGui = 0x80 | 0xE000,
}
}

impl ModifierKey {
    pub const ALL: [ModifierKey; 8] = [
        ModifierKey::LeftCtrl,
        ModifierKey::LeftShift,
        ModifierKey::LeftAlt,
        ModifierKey::LeftGui,
        ModifierKey::RightCtrl,
        ModifierKey::RightShift,
        ModifierKey::RightAlt,
        ModifierKey::RightGui,
    ];
}

enum_from_primitive! {
#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Key {
    A = 4 | 0xF000,
    B = 5 | 0xF000,
    C = 6 | 0xF000,
    D = 7 | 0xF000,
    E = 8 | 0xF000,
    F = 9 | 0xF000,
    G = 10 | 0xF000,
    H = 11 | 0xF000,
    I = 12 | 0xF000,
    J = 13 | 0xF000,
    K = 14 | 0xF000,
    L = 15 | 0xF000,
    M = 16 | 0xF000,
    N = 17 | 0xF000,
    O = 18 | 0xF000,
    P = 19 | 0xF000,
    Q = 20 | 0xF000,
    R = 21 | 0xF000,
    S = 22 | 0xF000,
    T = 23 | 0xF000,
    U = 24 | 0xF000,
    V = 25 | 0xF000,
    W = 26 | 0xF000,
    X = 27 | 0xF000,
    Y = 28 | 0xF000,
    Z = 29 | 0xF000,
    Key1 = 30 | 0xF000,
    Key2 = 31 | 0xF000,
    Key3 = 32 | 0xF000,
    Key4 = 33 | 0xF000,
    Key5 = 34 | 0xF000,
    Key6 = 35 | 0xF000,
    Key7 = 36 | 0xF000,
    Key8 = 37 | 0xF000,
    Key9 = 38 | 0xF000,
    Key0 = 39 | 0xF000,
    Enter = 40 | 0xF000,
    Esc = 41 | 0xF000,
    Backspace = 42 | 0xF000,
    Tab = 43 | 0xF000,
    Space = 44 | 0xF000,
    Minus = 45 | 0xF000,
    Equal = 46 | 0xF000,
    LeftBrace = 47 | 0xF000,
    RightBrace = 48 | 0xF000,
    Backslash = 49 | 0xF000,
    NonUsNum = 50 | 0xF000,
    Semicolon = 51 | 0xF000,
    Guote = 52 | 0xF000,
    Tilde = 53 | 0xF000,
    Comma = 54 | 0xF000,
    Period = 55 | 0xF000,
    Slash = 56 | 0xF000,
    CapsLock = 57 | 0xF000,
    F1 = 58 | 0xF000,
    F2 = 59 | 0xF000,
    F3 = 60 | 0xF000,
    F4 = 61 | 0xF000,
    F5 = 62 | 0xF000,
    F6 = 63 | 0xF000,
    F7 = 64 | 0xF000,
    F8 = 65 | 0xF000,
    F9 = 66 | 0xF000,
    F10 = 67 | 0xF000,
    F11 = 68 | 0xF000,
    F12 = 69 | 0xF000,
    PrintScreen = 70 | 0xF000,
    ScrollLock = 71 | 0xF000,
    Pause = 72 | 0xF000,
    Insert = 73 | 0xF000,
    Home = 74 | 0xF000,
    PageUp = 75 | 0xF000,
    Delete = 76 | 0xF000,
    End = 77 | 0xF000,
    PageDown = 78 | 0xF000,
    Right = 79 | 0xF000,
    Left = 80 | 0xF000,
    Down = 81 | 0xF000,
    Up = 82 | 0xF000,
    NumLock = 83 | 0xF000,
    KeyPadSlash = 84 | 0xF000,
    KeyPadAsterix = 85 | 0xF000,
    KeyPadMinus = 86 | 0xF000,
    KeyPadPlus = 87 | 0xF000,
    KeyPadEnter = 88 | 0xF000,
    KeyPad1 = 89 | 0xF000,
    KeyPad2 = 90 | 0xF000,
    KeyPad3 = 91 | 0xF000,
    KeyPad4 = 92 | 0xF000,
    KeyPad5 = 93 | 0xF000,
    KeyPad6 = 94 | 0xF000,
    KeyPad7 = 95 | 0xF000,
    KeyPad8 = 96 | 0xF000,
    KeyPad9 = 97 | 0xF000,
    KeyPad0 = 98 | 0xF000,
    KeyPadPeriod = 99 | 0xF000,
    NonUsBs = 100 | 0xF000,
    Menu = 101 | 0xF000,
    F13 = 104 | 0xF000,
    F14 = 105 | 0xF000,
    F15 = 106 | 0xF000,
    F16 = 107 | 0xF000,
    F17 = 108 | 0xF000,
    F18 = 109 | 0xF000,
    F19 = 110 | 0xF000,
    F20 = 111 | 0xF000,
    F21 = 112 | 0xF000,
    F22 = 113 | 0xF000,
    F23 = 114 | 0xF000,
    F24 = 115 | 0xF000,
}
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Key::A => "A",
                Key::B => "B",
                Key::C => "C",
                Key::D => "D",
                Key::E => "E",
                Key::F => "F",
                Key::G => "G",
                Key::H => "H",
                Key::I => "I",
                Key::J => "J",
                Key::K => "K",
                Key::L => "L",
                Key::M => "M",
                Key::N => "N",
                Key::O => "O",
                Key::P => "P",
                Key::Q => "Q",
                Key::R => "R",
                Key::S => "S",
                Key::T => "T",
                Key::U => "U",
                Key::V => "V",
                Key::W => "W",
                Key::X => "X",
                Key::Y => "Y",
                Key::Z => "Z",
                Key::Key1 => "Key1",
                Key::Key2 => "Key2",
                Key::Key3 => "Key3",
                Key::Key4 => "Key4",
                Key::Key5 => "Key5",
                Key::Key6 => "Key6",
                Key::Key7 => "Key7",
                Key::Key8 => "Key8",
                Key::Key9 => "Key9",
                Key::Key0 => "Key0",
                Key::Enter => "Enter",
                Key::Esc => "Esc",
                Key::Backspace => "Backspace",
                Key::Tab => "Tab",
                Key::Space => "Space",
                Key::Minus => "Minus",
                Key::Equal => "Equal",
                Key::LeftBrace => "LeftBrace",
                Key::RightBrace => "RightBrace",
                Key::Backslash => "Backslash",
                Key::NonUsNum => "NonUsNum",
                Key::Semicolon => "Semicolon",
                Key::Guote => "Guote",
                Key::Tilde => "Tilde",
                Key::Comma => "Comma",
                Key::Period => "Period",
                Key::Slash => "Slash",
                Key::CapsLock => "CapsLock",
                Key::F1 => "F1",
                Key::F2 => "F2",
                Key::F3 => "F3",
                Key::F4 => "F4",
                Key::F5 => "F5",
                Key::F6 => "F6",
                Key::F7 => "F7",
                Key::F8 => "F8",
                Key::F9 => "F9",
                Key::F10 => "F10",
                Key::F11 => "F11",
                Key::F12 => "F12",
                Key::PrintScreen => "PrintScreen",
                Key::ScrollLock => "ScrollLock",
                Key::Pause => "Pause",
                Key::Insert => "Insert",
                Key::Home => "Home",
                Key::PageUp => "PageUp",
                Key::Delete => "Delete",
                Key::End => "End",
                Key::PageDown => "PageDown",
                Key::Right => "Right",
                Key::Left => "Left",
                Key::Down => "Down",
                Key::Up => "Up",
                Key::NumLock => "NumLock",
                Key::KeyPadSlash => "KeyPadSlash",
                Key::KeyPadAsterix => "KeyPadAsterix",
                Key::KeyPadMinus => "KeyPadMinus",
                Key::KeyPadPlus => "KeyPadPlus",
                Key::KeyPadEnter => "KeyPadEnter",
                Key::KeyPad1 => "KeyPad1",
                Key::KeyPad2 => "KeyPad2",
                Key::KeyPad3 => "KeyPad3",
                Key::KeyPad4 => "KeyPad4",
                Key::KeyPad5 => "KeyPad5",
                Key::KeyPad6 => "KeyPad6",
                Key::KeyPad7 => "KeyPad7",
                Key::KeyPad8 => "KeyPad8",
                Key::KeyPad9 => "KeyPad9",
                Key::KeyPad0 => "KeyPad0",
                Key::KeyPadPeriod => "KeyPadPeriod",
                Key::NonUsBs => "NonUsBs",
                Key::Menu => "Menu",
                Key::F13 => "F13",
                Key::F14 => "F14",
                Key::F15 => "F15",
                Key::F16 => "F16",
                Key::F17 => "F17",
                Key::F18 => "F18",
                Key::F19 => "F19",
                Key::F20 => "F20",
                Key::F21 => "F21",
                Key::F22 => "F22",
                Key::F23 => "F23",
                Key::F24 => "F24",
            }
        )
    }
}

impl Key {
    pub const ALL: [Key; 110] = [
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
        Key::Key1,
        Key::Key2,
        Key::Key3,
        Key::Key4,
        Key::Key5,
        Key::Key6,
        Key::Key7,
        Key::Key8,
        Key::Key9,
        Key::Key0,
        Key::Enter,
        Key::Esc,
        Key::Backspace,
        Key::Tab,
        Key::Space,
        Key::Minus,
        Key::Equal,
        Key::LeftBrace,
        Key::RightBrace,
        Key::Backslash,
        Key::NonUsNum,
        Key::Semicolon,
        Key::Guote,
        Key::Tilde,
        Key::Comma,
        Key::Period,
        Key::Slash,
        Key::CapsLock,
        Key::F1,
        Key::F2,
        Key::F3,
        Key::F4,
        Key::F5,
        Key::F6,
        Key::F7,
        Key::F8,
        Key::F9,
        Key::F10,
        Key::F11,
        Key::F12,
        Key::PrintScreen,
        Key::ScrollLock,
        Key::Pause,
        Key::Insert,
        Key::Home,
        Key::PageUp,
        Key::Delete,
        Key::End,
        Key::PageDown,
        Key::Right,
        Key::Left,
        Key::Down,
        Key::Up,
        Key::NumLock,
        Key::KeyPadSlash,
        Key::KeyPadAsterix,
        Key::KeyPadMinus,
        Key::KeyPadPlus,
        Key::KeyPadEnter,
        Key::KeyPad1,
        Key::KeyPad2,
        Key::KeyPad3,
        Key::KeyPad4,
        Key::KeyPad5,
        Key::KeyPad6,
        Key::KeyPad7,
        Key::KeyPad8,
        Key::KeyPad9,
        Key::KeyPad0,
        Key::KeyPadPeriod,
        Key::NonUsBs,
        Key::Menu,
        Key::F13,
        Key::F14,
        Key::F15,
        Key::F16,
        Key::F17,
        Key::F18,
        Key::F19,
        Key::F20,
        Key::F21,
        Key::F22,
        Key::F23,
        Key::F24,
    ];
}
