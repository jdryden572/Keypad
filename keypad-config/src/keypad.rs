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

#[derive(Default)]
pub struct FlashCommand {
    pub one: bool,
    pub two: bool,
    pub three: bool,
    pub four: bool,
    pub five: bool,
    pub six: bool,
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

    pub fn flash_key(&mut self, flash: FlashCommand) -> Result<(), KeypadError> {
        let mut mask = 0u8;
        if flash.one {
            mask = mask | 1 << 0
        }
        if flash.two {
            mask = mask | 1 << 1
        }
        if flash.three {
            mask = mask | 1 << 2
        }
        if flash.four {
            mask = mask | 1 << 3
        }
        if flash.five {
            mask = mask | 1 << 4
        }
        if flash.six {
            mask = mask | 1 << 5
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
        let combo = KeyCombo {
            modifier_one: ModifierKey::from_u16(keys.next().unwrap()),
            modifier_two: ModifierKey::from_u16(keys.next().unwrap()),
            key_one: Key::from_u16(keys.next().unwrap()),
            key_two: Key::from_u16(keys.next().unwrap()),
        };

        Ok(combo)
    }

    fn to_bytes(&self) -> Vec<u8> {
        [
            modifier_key_to_bytes(self.modifier_one),
            modifier_key_to_bytes(self.modifier_two),
            key_to_bytes(self.key_one),
            key_to_bytes(self.key_two),
        ]
        .concat()
    }
}

fn modifier_key_to_bytes(key: Option<ModifierKey>) -> [u8; 2] {
    match key {
        Some(modifier) => (modifier as u16).to_le_bytes(),
        None => [0u8; 2],
    }
}

fn key_to_bytes(key: Option<Key>) -> [u8; 2] {
    match key {
        Some(key) => (key as u16).to_le_bytes(),
        None => [0u8; 2],
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
