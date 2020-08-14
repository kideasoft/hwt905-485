//! A platform agnostic driver to interface with the HWT905-485 sensor
//!
//! # Example
//!   TODO:

use chrono::prelude::*;
use crc16::*;
use serialport::{Error, SerialPort};

pub struct Hwt905 {
    modbus_addr: u8,
    port: Box<dyn SerialPort>,
}

impl Hwt905 {
    pub fn new(port_path: &str, modbus_addr: u8, baud_rate: u32) -> Result<Self, Error> {
        let mut port = serialport::open(port_path)?;
        port.set_baud_rate(baud_rate)?;
        Ok(Hwt905 {
            // port_path,
            modbus_addr,
            // baud_rate,
            port,
        })
    }

    pub fn magnetic_field(&mut self) -> Result<(i16, i16, i16), Error> {
        let mut buffer: Vec<u8> = vec![self.modbus_addr, 0x03, 0x00, 0x3a, 0x00, 0x03];
        let crc = crc16::State::<MODBUS>::calculate(&buffer).to_le_bytes();
        buffer.extend_from_slice(&crc);
        self.port.write(&buffer)?;

        std::thread::sleep(std::time::Duration::from_millis(200));

        let mut buf = [0_u8; 1024];
        let n = self.port.read(&mut buf)?;

        // DEBUG:
        // println!("receive: {:?}", &buf[..n]);

        let crc = crc16::State::<MODBUS>::calculate(&buf[..n - 2]);
        let crc_check = crc == u16::from_le_bytes([buf[n - 2], buf[n - 1]]);
        if !crc_check {
            eprintln!(
                "{} crc check error for reply from hwt905",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }

        let mag_x = i16::from_le_bytes([buf[4], buf[3]]);
        let mag_y = i16::from_le_bytes([buf[6], buf[5]]);
        let mag_z = i16::from_le_bytes([buf[8], buf[7]]);
        Ok((mag_x, mag_y, mag_z))
    }
}
