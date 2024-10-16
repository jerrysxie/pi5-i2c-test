use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::i2c::I2c;

const ADDR: u16 = 0x20;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;

    let clock = i2c.clock_speed()?;

    println!("clock = {:#?}", clock);

    i2c.set_timeout(5000)?;

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(ADDR)?;

    let init_cmd = [0xBE];
    let magic_cmd = [0xEF];

    loop {
        i2c.write(&init_cmd)?;

        let mut code: [u8; 4] = [0xAA; 4];
        i2c.write_read(&magic_cmd, &mut code)?;

        println!("magic code = {:#x?}", code);
        thread::sleep(Duration::from_secs(1));
    }
}
