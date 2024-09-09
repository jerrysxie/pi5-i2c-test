# pi5-i2c-test

This is an example Rust program to manipulate the I2C bus on Raspberry Pi on Raspbian Linux

## Setup
1. Enable I2C interface using config.txt or Raspberry Pi Configuration. Make sure `i2cdetect -l` shows `i2c-1`
1. Hook up to the I2C1 pins of Pi
1. Hook up a GPIO pin to Pi as interrupt line
1. Reconfigure the pin number in main.rs if needed
