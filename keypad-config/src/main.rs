#[macro_use] extern crate enum_primitive;

mod keys;
use std::error::Error;
use std::io::{Read, Write};
use std::convert::{TryInto};
use num_traits::cast::FromPrimitive;
use serialport::SerialPort;
use keys::{Key, ModifierKey};

#[derive(Debug)]
struct KeyCombo {
    modifier: Option<ModifierKey>,
    key: Option<Key>
}

impl KeyCombo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let modifier = u16::from_le_bytes(bytes[0..2].try_into()?);
        let key = u16::from_le_bytes(bytes[2..4].try_into()?);
        let combo = KeyCombo { 
            modifier: ModifierKey::from_u16(modifier), 
            key: Key::from_u16(key) 
        };
        Ok(combo)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mod_bytes = match self.modifier {
            Some(modifier) => (modifier as u16).to_le_bytes(),
            None => [0u8; 2]
        };
        let key_bytes = match self.key {
            Some(key) => (key as u16).to_le_bytes(),
            None => [0u8; 2]
        };

        [mod_bytes, key_bytes].concat()
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
        KeyCombo { modifier: Some(ModifierKey::LeftShift), key: Some(Key::P) },
        KeyCombo { modifier: None, key: Some(Key::W) },
        KeyCombo { modifier: None, key: Some(Key::E) },
        KeyCombo { modifier: Some(ModifierKey::LeftShift), key: Some(Key::R) },
        KeyCombo { modifier: None, key: Some(Key::T) },
        KeyCombo { modifier: None, key: Some(Key::Y) },
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