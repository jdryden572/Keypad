use crate::KeyPress;

use super::keys::{Key, KeyCombo, ModifierKey};
use num_traits::cast::FromPrimitive;
use serialport::SerialPort;
use std::convert::TryInto;
use thiserror::Error;

const HELLO: u8 = 'H' as u8;
const ACK: u8 = 'A' as u8;
const READ_KEYS: u8 = 'R' as u8;
const WRITE_KEYS: u8 = 'W' as u8;
const FLASH: u8 = 'F' as u8;

#[derive(Error, Debug)]
pub enum KeypadError {
    #[error("Serial opening error")]
    SerialError(#[from] serialport::Error),
    #[error("Serial communication error")]
    SerialCommunicationError(#[from] std::io::Error),
    #[error("Invalid data error")]
    InvalidDataError,
    #[error("No ACK was recieved")]
    NoAcknowledge,
    #[error("No device found")]
    NoDeviceFound,
    #[error("Device returned unexpected key count")]
    WrongKeyCountFromDevice,
}

pub struct Keypad {
    serial_port: Box<dyn SerialPort>,
}

impl Keypad {
    pub fn auto_detect() -> Result<Keypad, KeypadError> {
        let settings = serialport::SerialPortSettings {
            baud_rate: 115200,
            timeout: std::time::Duration::from_millis(500),
            ..Default::default()
        };

        log::info!("Beginning auto-detect");
        let ports = serialport::available_ports()?;
        log::info!("Found {} available ports", ports.len());

        for port in ports {
            let name: &str = port.port_name.as_ref();
            log::info!("Trying port {}...", name);
            if let Ok(mut port) = serialport::open_with_settings(name, &settings) {
                if handshake(&mut *port).is_ok() {
                    return Ok(Keypad { serial_port: port });
                }
            }
        }

        Err(KeypadError::NoDeviceFound)
    }

    pub fn send_combos_to_device(
        &mut self,
        combos: [KeyCombo; 6],
    ) -> Result<[KeyCombo; 6], KeypadError> {
        log::info!("Loading combos into buffer...");
        let mut buf = [0u8; 49];
        buf[0] = WRITE_KEYS;

        let mut idx = 1;

        for combo in combos.iter() {
            let bytes: Vec<u8> = combo.to_bytes();
            for b in bytes {
                buf[idx] = b;
                idx += 1;
            }
        }

        log::info!("Sending WRITE_KEYS command...");

        self.serial_port.write(&buf)?;
        self.serial_port.flush()?;

        read_combos_from_port(&mut *self.serial_port)
    }

    pub fn get_combos_from_device(&mut self) -> Result<[KeyCombo; 6], KeypadError> {
        log::info!("Sending READ_KEYS command...");
        self.serial_port.write(&[READ_KEYS])?;
        self.serial_port.flush()?;

        read_combos_from_port(&mut *self.serial_port)
    }

    pub fn flash_keys(&mut self, flash: [bool; 6]) -> Result<(), KeypadError> {
        let mut mask = 0u8;
        for (idx, &on) in flash.iter().enumerate() {
            if on {
                mask = mask | 1 << idx;
            }
        }

        log::info!("Sending flash command {:b}", mask);
        self.serial_port.write(&[FLASH, mask])?;
        self.serial_port.flush()?;
        wait_for_acknowledge(&mut *self.serial_port)?;
        Ok(())
    }
}

fn handshake(port: &mut dyn SerialPort) -> Result<(), KeypadError> {
    let name = port.name().unwrap_or(String::new());
    log::info!("Sending handshake to {}", name);
    port.write(&[HELLO])?;
    port.flush()?;

    match wait_for_acknowledge(port) {
        Ok(()) => {
            log::info!("Handshake returned! Keypad at {}", name);
            Ok(())
        }
        Err(e) => {
            log::info!("No handshake.");
            Err(e)
        }
    }
}

fn wait_for_acknowledge(port: &mut dyn SerialPort) -> Result<(), KeypadError> {
    let mut resp = [0u8; 1];
    port.read_exact(&mut resp)?;
    if resp[0] == ACK {
        Ok(())
    } else {
        Err(KeypadError::NoAcknowledge)
    }
}

impl KeyPress {
    fn from_u16(modifier: u16, key: u16) -> Result<Self, KeypadError> {
        log::trace!("Modifier: {}, Key: {}", modifier, key);
        let press = KeyPress {
            ctrl: modifier & ModifierKey::LeftCtrl as u16 == ModifierKey::LeftCtrl as u16,
            alt: modifier & ModifierKey::LeftAlt as u16 == ModifierKey::LeftAlt as u16,
            shift: modifier & ModifierKey::LeftShift as u16 == ModifierKey::LeftShift as u16,
            windows: modifier & ModifierKey::LeftGui as u16 == ModifierKey::LeftGui as u16,
            key: Key::from_u16(key).ok_or_else(|| KeypadError::InvalidDataError)?
        };
        log::trace!("KeyPress: {}", press);
        Ok(press)
    }

    fn to_bytes(&self) -> [u8; 4] {
        log::trace!("Write KeyPress: {}", self);
        let mut modifier = 0u16;
        if self.ctrl {
            modifier = modifier | ModifierKey::LeftCtrl as u16;
        }
        if self.alt {
            modifier = modifier | ModifierKey::LeftAlt as u16;
        }
        if self.shift {
            modifier = modifier | ModifierKey::LeftShift as u16;
        }
        if self.windows {
            modifier = modifier | ModifierKey::LeftGui as u16;
        }
        log::trace!("Modifier: {}, Key: {}", modifier, self.key as u16);
        let modifier = modifier.to_le_bytes();
        let key = (self.key as u16).to_le_bytes();

        [
            modifier[0],
            modifier[1],
            key[0],
            key[1],
        ]
    }
}

fn optional_key_bytes(option: &Option<KeyPress>) -> [u8; 4] {
    match option {
        Some(press) => press.to_bytes(),
        None => [0u8; 4]
    }
}

impl KeyCombo {
    fn from_bytes(bytes: &[u8]) -> Result<Self, KeypadError> {
        let keys: Result<Vec<_>, _> = bytes
            .chunks(2)
            .map(|chunk| {
                TryInto::<[u8; 2]>::try_into(chunk)
                    .map(u16::from_le_bytes)
                    .map_err(|_| KeypadError::InvalidDataError)
            })
            .collect();

        let mut keys = keys?.into_iter();
        let modifier = keys.next().unwrap();
        let key = keys.next().unwrap();
        let one = KeyPress::from_u16(modifier, key)?;

        let modifier = keys.next().unwrap();
        let key = keys.next().unwrap();
        let two = if key > 0 {
            Some(KeyPress::from_u16(modifier, key)?)
        } else {
            None
        };

        let combo = KeyCombo { one, two };
        Ok(combo)
    }

    fn to_bytes(&self) -> Vec<u8> {
        [
            self.one.to_bytes(),
            optional_key_bytes(&self.two)
        ]
        .concat()
    }
}

fn read_combos_from_port(port: &mut dyn SerialPort) -> Result<[KeyCombo; 6], KeypadError> {
    log::info!("Reading combos response into buffer...");
    let mut resp: [u8; 48] = [0; 48];
    port.read_exact(&mut resp)?;

    log::info!("Parsing response into KeyCombos...");
    let combos: Result<Vec<KeyCombo>, _> = resp
        .chunks(8)
        .map(|chunk| KeyCombo::from_bytes(chunk))
        .collect();

    match combos {
        Ok(combos) => {
            log::info!("Parsed {} combos", combos.len());
            combos
                .try_into()
                .map_err(|_| KeypadError::WrongKeyCountFromDevice)
        }
        Err(e) => {
            log::error!("Error parsing combos: {}", e);
            Err(e)
        }
    }
}
