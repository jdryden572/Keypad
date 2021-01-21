use std::env;

use keypad::*;

fn main() -> Result<(), KeypadError> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut keypad = Keypad::auto_detect()?;

    // keypad.flash_keys([true, false, true, false, true, false])?;
    // std::thread::sleep(Duration::from_millis(2000));

    let saved = keypad.get_combos_from_device()?;
    for combo in saved.iter() {
        println!("{}", combo);
    }

    let combos = [
        KeyCombo {
            one: KeyPress::windows().key(Key::L),
            two: None
        },
        KeyCombo {
            one: KeyPress::shift().key(Key::M),
            two: Some(KeyPress::key(Key::R))
        },
        KeyCombo {
            one: KeyPress::ctrl().key(Key::R),
            two: Some(KeyPress::key(Key::D))
        },
        KeyCombo {
            one: KeyPress::ctrl().key(Key::R),
            two: Some(KeyPress::key(Key::L))
        },
        KeyCombo {
            one: KeyPress::ctrl().key(Key::B),
            two: None
        },
        KeyCombo {
            one: KeyPress::ctrl().key(Key::Period),
            two: None
        },
    ];

    // let combos = [
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad1),
    //         two: None
    //     },
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad2),
    //         two: None
    //     },
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad3),
    //         two: None
    //     },
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad4),
    //         two: None
    //     },
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad5),
    //         two: None
    //     },
    //     KeyCombo {
    //         one: KeyPress::key(Key::KeyPad6),
    //         two: None
    //     },
    // ];

    let saved = keypad.send_combos_to_device(combos)?;
    for combo in saved.iter() {
        println!("{}", combo);
    }

    Ok(())
}
