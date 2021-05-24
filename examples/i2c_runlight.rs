//! Example of using I2C.
//! Interact with PCF8574 I/O expander, used to create LED run light
//! Target board: STM32F303RE Nucleo
// see also: i2c_scan.rs, https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/i2c_scanner.rs

#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3xx_hal::{self as hal, pac, prelude::*};

const I2C_ADDR: u8 = 32;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure I2C1
    let scl =
        gpiob
            .pb6
            .into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda =
        gpiob
            .pb7
            .into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let mut i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        100u32.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let mut pos: u8 = 0x01;
    let mut direction = true;
    loop {
        let value = !(0x01_u8 << pos);
        i2c.write(I2C_ADDR, &[value]).unwrap();
        
        // calculate next position
        if pos == 7 {direction = false}
        if pos == 0 {direction = true}
        if direction{pos += 1}
        else {pos -= 1}

        asm::delay(1_000_000);
    }
}