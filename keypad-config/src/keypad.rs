use std::convert::{TryInto};
use num_traits::cast::FromPrimitive;
use serialport::SerialPort;
use thiserror::Error;
use crate::keys::{Key, ModifierKey, KeyCombo};

const HELLO: u8 = 'H' as u8;
const HELLO_ACK: u8 = 'A' as u8;
const READ_KEYS: u8 = 'R' as u8;
const WRITE_KEYS: u8 = 'W' as u8;

#[derive(Error, Debug)]
pub enum KeypadError {
    #[error("Serial opening error")]
    SerialError(#[from] serialport::Error),
    #[error("Serial communication error")]
    SerialCommunicationError(#[from] std::io::Error),
    #[error("Invalid data error")]
    InvalidDataError,
    #[error("The device at port `{0}` did not respond to handshake")]
    HandshakeError(String),
    #[error("No device found")]
    NoDeviceFound
}

pub struct Keypad {
    serial_port: Box<dyn SerialPort>
}

impl Keypad {
    pub fn auto_detect() -> Result<Keypad, KeypadError> {
        let settings = serialport::SerialPortSettings {
            baud_rate: 115200,
            timeout: std::time::Duration::from_millis(25),
            ..Default::default()
        };

        for port in serialport::available_ports()? {
            if let Ok(mut port) = serialport::open_with_settings(&port.port_name, &settings) {
                if handshake(&mut *port).is_ok() {
                    return Ok(Keypad { serial_port: port });
                }
            }
        }

        Err(KeypadError::NoDeviceFound)
    }

    pub fn send_combos_to_device(&mut self, combos: &Vec<KeyCombo>) -> Result<Vec<KeyCombo>, KeypadError> {
        let mut buf = [0u8; 49];
        buf[0] = WRITE_KEYS;
    
        let mut idx = 1;
        
        for combo in combos {
            let bytes: Vec<u8> = combo.to_bytes();
            for b in bytes {
                buf[idx] = b;
                idx += 1;
            }
        }
    
        self.serial_port.write(&buf)?;
        self.serial_port.flush()?;
    
        read_combos_from_port(&mut *self.serial_port)
    }
    
    pub fn get_combos_from_device(&mut self) -> Result<Vec<KeyCombo>, KeypadError> {
        self.serial_port.write(&[READ_KEYS])?;
        self.serial_port.flush()?;
    
        read_combos_from_port(&mut *self.serial_port)
    }
}

fn handshake(port: &mut dyn SerialPort) -> Result<(), KeypadError> {
    port.write(&[HELLO])?;
    port.flush()?;
    
    let mut resp = [0u8; 1];
    port.read_exact(&mut resp)?;
    match resp[0] {
        HELLO_ACK => Ok(()),
        _ => Err(KeypadError::HandshakeError(port.name().unwrap_or(Default::default())))
    }
}

impl KeyCombo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KeypadError> {
        let keys: Result<Vec<_>, _> = bytes.chunks(2)
            .map(|chunk| TryInto::<[u8;2]>::try_into(chunk).map(u16::from_le_bytes).map_err(|_| KeypadError::InvalidDataError))
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

fn read_combos_from_port(port: &mut dyn SerialPort) -> Result<Vec<KeyCombo>, KeypadError> {
    let mut resp: [u8; 48] = [0; 48];
    port.read_exact(&mut resp)?;

    let combos: Result<Vec<KeyCombo>, _> = resp.chunks(8)
        .map(|chunk| KeyCombo::from_bytes(chunk))
        .collect();
    
    combos
}