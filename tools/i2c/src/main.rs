use std::{fs::File, io, thread::sleep, time::Duration};

use i2c_linux::I2c;

enum LedAddress {
    EyeBlue = 0xCC >> 1,
    EyeGreen = 0xC8 >> 1,
    EyeRed = 0xC4 >> 1,
    LeftEar = 0xC2 >> 1,
    RightEar = 0xC0 >> 1,
    Skull = 0xCA >> 1,
}

struct LedInterface {
    i2c: I2c<File>,
}

impl LedInterface {
    fn initialize(address: LedAddress) -> Result<Self, io::Error> {
        // Soft reset
        let mut i2c = I2c::from_path("/dev/i2c-head")?;
        i2c.smbus_set_slave_address(0xD6 >> 1, false)?;
        i2c.smbus_write_byte_data(0xA5, 0x5A).unwrap();

        let mut i2c = I2c::from_path("/dev/i2c-head")?;
        i2c.smbus_set_slave_address(address as u16, false)?;

        // Initialize
        i2c.smbus_write_byte_data(0x00, 0x00).unwrap();
        i2c.smbus_write_byte_data(0x01, 0x00).unwrap();

        // Set LED output mode
        i2c.smbus_write_byte_data(0x14, 0b10101010).unwrap();
        i2c.smbus_write_byte_data(0x15, 0b10101010).unwrap();
        i2c.smbus_write_byte_data(0x16, 0b10101010).unwrap();
        i2c.smbus_write_byte_data(0x17, 0b10101010).unwrap();

        Ok(Self { i2c })
    }

    fn write_leds(&mut self, intensites: &[u8]) -> Result<(), io::Error> {
        assert!(intensites.len() <= 16);

        self.i2c
            .smbus_write_block_data(0x80 | 0x20 | 0x02, intensites)
    }
}

fn main() -> io::Result<()> {
    let mut leds = LedInterface::initialize(LedAddress::EyeRed).unwrap();

    let mut vals = [0; 16];

    loop {
        for i in 0..16 {
            vals[i] = 0x0;
            vals[(i + 1) % 16] = 0xFF;
            leds.write_leds(&vals).unwrap();
            sleep(Duration::from_millis(100));
        }
    }
}
