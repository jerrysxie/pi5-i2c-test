use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::i2c::I2c;

const ADDR: u16 = 0x62;
const GPIO_ISR: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    let mut pin = Gpio::new()?.get(GPIO_ISR)?.into_input();

    // Set the I2C slave address to the device we're communicating with.
    i2c.set_slave_address(ADDR)?;

    let mut buf = [0u8; 5];

    pin.set_interrupt(rppal::gpio::Trigger::RisingEdge, None);

    loop {
        if let Ok(Some(_)) = pin.poll_interrupt(false, Some(Duration::from_millis(100))) {
            i2c.read(&mut buf)?;

            println!("{:?}", buf);
        } else {
            println!("Waiting for interrupt");
        }
    }
}
