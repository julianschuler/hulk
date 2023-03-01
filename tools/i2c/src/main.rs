use std::{fs::File, io, thread::sleep, time::Duration};

use i2c_linux::I2c;

enum LedAddress {
    EyeBlue = 0xCC,
    EyeGreen = 0xC8,
    EyeRed = 0xC4,
    LeftEar = 0xC2,
    RightEar = 0xC0,
    Skull = 0xCA,
}

struct LedInterface {
    i2c: I2c<File>,
}

impl LedInterface {
    fn reset_all() -> Result<(), io::Error> {
        let mut i2c = I2c::from_path("/dev/i2c-head")?;
        i2c.smbus_set_slave_address(0xD6 >> 1, false)?;
        i2c.smbus_write_byte_data(0xA5, 0x5A)
    }

    fn initialize(address: LedAddress) -> Result<Self, io::Error> {
        let mut i2c = I2c::from_path("/dev/i2c-head")?;
        i2c.smbus_set_slave_address(address as u16 >> 1, false)?;

        // Initialize
        i2c.smbus_write_byte_data(0x00, 0x00)?;
        i2c.smbus_write_byte_data(0x01, 0x00)?;

        // Set LED output mode
        i2c.smbus_write_byte_data(0x14, 0b10101010)?;
        i2c.smbus_write_byte_data(0x15, 0b10101010)?;
        i2c.smbus_write_byte_data(0x16, 0b10101010)?;
        i2c.smbus_write_byte_data(0x17, 0b10101010)?;

        Ok(Self { i2c })
    }

    fn write_leds(&mut self, intensites: &[u8]) -> Result<(), io::Error> {
        assert!(intensites.len() <= 16);

        self.i2c
            .smbus_write_block_data(0x80 | 0x20 | 0x02, intensites)
    }
}

struct TouchSensorInterface {
    i2c: I2c<File>,
}

impl TouchSensorInterface {
    fn initialize() -> Result<Self, io::Error> {
        let mut i2c = I2c::from_path("/dev/i2c-head")?;
        i2c.smbus_set_slave_address(0x50 >> 1, false)?;

        // Clear touch interupt flag
        i2c.smbus_write_byte_data(0x00, 0x00)?;
        // Disable multitouch circutry
        i2c.smbus_write_byte_data(0x2A, 0x00)?;
        // Set sensitivity multiplier to 64x, data scaling factor to 1x
        i2c.smbus_write_byte_data(0x1F, 0x10)?;
        // Set sensor input thresholds to 40
        i2c.smbus_write_byte_data(0x30, 0x28)?;
        i2c.smbus_write_byte_data(0x31, 0x28)?;
        i2c.smbus_write_byte_data(0x32, 0x28)?;
        // Enable RF noise filtering
        i2c.smbus_write_byte_data(0x44, 0x44)?;
        // Capacity calibration on all channels
        i2c.smbus_write_byte_data(0x26, 0x07)?;
        while i2c.smbus_read_byte_data(0x26)? != 0 {}

        Ok(Self { i2c })
    }

    fn read(&mut self) -> Result<(bool, bool, bool), io::Error> {
        let output = self.i2c.smbus_read_byte_data(0x03)?;
        // Clear touch interupt flag
        self.i2c.smbus_write_byte_data(0x00, 0x00)?;
        Ok((output & 0b1 != 0, output & 0b10 != 0, output & 0b100 != 0))
    }
}

fn main() -> io::Result<()> {
    LedInterface::reset_all()?;

    let mut red_leds = LedInterface::initialize(LedAddress::EyeRed)?;
    let mut green_leds = LedInterface::initialize(LedAddress::EyeGreen)?;
    let mut blue_leds = LedInterface::initialize(LedAddress::EyeBlue)?;
    let mut touch = TouchSensorInterface::initialize()?;

    let mut vals = [0; 16];

    loop {
        for i in 0..16 {
            vals[i] = 0x0;
            vals[(i + 1) % 16] = 0x50;
            red_leds.write_leds(&vals).unwrap();
            green_leds.write_leds(&vals).unwrap();
            blue_leds.write_leds(&vals).unwrap();
            let vals = touch.read().unwrap();
            dbg!(vals);
            sleep(Duration::from_millis(100));
        }
    }
}
