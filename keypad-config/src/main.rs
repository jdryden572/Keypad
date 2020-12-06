#[macro_use] extern crate enum_primitive;

mod keys;
use std::error::Error;
use std::io::{Read, Write};
use std::convert::{TryInto};
use num_traits::cast::FromPrimitive;
use serialport::SerialPort;
use keys::{Key, ModifierKey, KeyCombo};

impl KeyCombo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let keys: Result<Vec<_>, _> = bytes.chunks(2)
            .map(|chunk| TryInto::<[u8;2]>::try_into(chunk).map(u16::from_le_bytes))
            .collect();

        let mut keys = keys?.into_iter();
        let combo = KeyCombo {
            modifier_one: ModifierKey::from_u16(keys.next().unwrap()),
            modifier_two: ModifierKey::from_u16(keys.next().unwrap()),
            key_one: Key::from_u16(keys.next().unwrap()),
            key_two: Key::from_u16(keys.next().unwrap()),
        };

        Ok(combo)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            modifier_key_to_bytes(self.modifier_one),
            modifier_key_to_bytes(self.modifier_two),
            key_to_bytes(self.key_one),
            key_to_bytes(self.key_two),
        ].concat()
    }
}

fn modifier_key_to_bytes(key: Option<ModifierKey>) -> [u8; 2] {
    match key {
        Some(modifier) =>  (modifier as u16).to_le_bytes(),
        None => [0u8; 2]
    }
}

fn key_to_bytes(key: Option<Key>) -> [u8; 2] {
    match key {
        Some(key) =>  (key as u16).to_le_bytes(),
        None => [0u8; 2]
    }
}

fn main() {
    let settings = serialport::SerialPortSettings {
        baud_rate: 115200,
        timeout: std::time::Duration::from_millis(25),
        ..Default::default()
    };

    let mut port = serialport::open_with_settings("COM3", &settings).unwrap();

    let combos = vec![
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::A), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::B), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::C), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::D), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::E), key_two: None },
        KeyCombo { modifier_one: None, modifier_two: None, key_one: Some(Key::F), key_two: None },
    ];

    let saved = send_combos_to_device(&combos, &mut *port).unwrap();

    for combo in saved {
        println!("{:?}", combo);
    }
}

fn send_combos_to_device(combos: &Vec<KeyCombo>, port: &mut dyn SerialPort) -> Result<Vec<KeyCombo>, Box<dyn Error>> {
    let mut buf = [0u8; 25];
    buf[0] = 83; // 'S'

    let mut idx = 1;
    
    for combo in combos {
        let bytes: Vec<u8> = combo.to_bytes();
        for b in bytes {
            buf[idx] = b;
            idx += 1;
        }
    }

    port.write(&buf)?;
    port.flush()?;

    read_combos_from_port(&mut *port)
}

fn get_combos_from_device(port: &mut dyn SerialPort) -> Result<Vec<KeyCombo>, Box<dyn Error>> {
    let mut buf: [u8; 1] = [0];
    'P'.encode_utf8(&mut buf);

    port.write(&buf)?;
    port.flush()?;

    read_combos_from_port(&mut *port)
}

fn read_combos_from_port(port: &mut dyn SerialPort) -> Result<Vec<KeyCombo>, Box<dyn Error>> {
    let mut resp: [u8; 24] = [0; 24];
    port.read_exact(&mut resp)?;

    let combos: Vec<KeyCombo> = resp.chunks(4)
        .map(|chunk| KeyCombo::from_bytes(chunk).unwrap())
        .collect();
    
    Ok(combos)
}