use std::fmt::Display;

use keypad::{Key, KeyCombo, KeyPress};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub combos: [KeyCombo; 6],
    pub auto_launch_program: Option<String>,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        match self.auto_launch_program.as_ref() {
            Some(program) => write!(f, " ({})", program)?,
            None => {}
        };
        Ok(())
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: String::new(),
            auto_launch_program: None,
            combos: [
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
                KeyCombo {
                    one: KeyPress::key(Key::A),
                    two: None,
                },
            ],
        }
    }
}
