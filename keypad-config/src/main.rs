mod keys;
mod keypad;

use keys::*;
use keypad::{Keypad, KeypadError};


fn main() -> Result<(), KeypadError> {
    let mut keypad = Keypad::auto_detect()?;

    let saved = keypad.get_combos_from_device()?;
    for combo in saved {
        println!("{:?}", combo);
    }

    let combos = vec![
        KeyCombo { modifier_one: Some(ModifierKey::LeftShift), modifier_two: None, key_one: Some(Key::A), key_two: Some(Key::X) },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::B), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::C), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::D), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::E), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::F), key_two: None },
    ];

    let saved = keypad.send_combos_to_device(&combos)?;
    for combo in saved {
        println!("{:?}", combo);
    }

    Ok(())
}
