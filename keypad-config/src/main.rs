use std::env;
use std::time::Duration;

use keypad::*;

fn main() -> Result<(), KeypadError> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut keypad = Keypad::auto_detect()?;

    keypad.flash_keys([true, false, true, false, true, false])?;
    std::thread::sleep(Duration::from_millis(2000));

    let saved = keypad.get_combos_from_device()?;
    for combo in saved.iter() {
        println!("{}", combo);
    }

    let combos = [
        KeyCombo {
            modifier_one: Some(ModifierKey::LeftGui),
            modifier_two: None,
            key_one: Some(Key::L),
            key_two: None,
        },
        KeyCombo {
            modifier_one: None,
            modifier_two: None,
            key_one: Some(Key::B),
            key_two: None,
        },
        KeyCombo {
            modifier_one: Some(ModifierKey::LeftCtrl),
            modifier_two: None,
            key_one: Some(Key::R),
            key_two: Some(Key::D),
        },
        KeyCombo {
            modifier_one: Some(ModifierKey::LeftCtrl),
            modifier_two: None,
            key_one: Some(Key::R),
            key_two: Some(Key::L),
        },
        KeyCombo {
            modifier_one: Some(ModifierKey::LeftCtrl),
            modifier_two: None,
            key_one: Some(Key::B),
            key_two: None,
        },
        KeyCombo {
            modifier_one: Some(ModifierKey::LeftCtrl),
            modifier_two: None,
            key_one: Some(Key::Period),
            key_two: None,
        },
    ];

    // // let combos = vec![
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::F), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::E), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::D), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::C), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::B), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::A), key_two: None },
    // // ];

    // // let combos = vec![
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad1), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad2), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad3), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad4), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad5), key_two: None },
    // //     KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::KeyPad6), key_two: None },
    // // ];

    let saved = keypad.send_combos_to_device(combos)?;
    for combo in saved.iter() {
        println!("{}", combo);
    }

    Ok(())
}
