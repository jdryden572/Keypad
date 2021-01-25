use std::env;

use keypad::*;

fn main() -> Result<(), KeypadError> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let mut keypad = Keypad::auto_detect()?;

    let saved = keypad.get_combos_from_device()?;
    for combo in saved.iter() {
        println!("{}", combo);
    }

    Ok(())
}
