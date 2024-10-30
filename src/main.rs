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

    let init_cmd = [0x05, 0xDE, 0xAD, 0xBE, 0xEF];
    let magic_cmd = [0x05, 0xDE, 0xCA, 0xFB, 0xAD];

    let mut i: u32 = 0;
    loop {
        if i % 2 == 0 {
            println!("write init cmd");
            i2c.write(&init_cmd)?;
        } else {
            let mut code: [u8; 4] = [0xAA; 4];
            println!("write magic code");
            i2c.write_read(&magic_cmd, &mut code)?;
            println!("magic code = {:02x?}", code);
        }
        i += 1;
        thread::sleep(Duration::from_secs(2));
    }
}
