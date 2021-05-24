//! Example of using I2C.
//! Scans available I2C devices on bus and print the result.
//! Target board: STM32F303RE Nucleo
// source (modified): https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/i2c_scanner.rs

#![no_std]
#![no_main]

use core::ops::Range;

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3xx_hal::{self as hal, pac, prelude::*};
use heapless::String;

const UART_BAUDRATE: u32 = 9600;
const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure I2C1
    let mut scl =
        gpiob
            .pb6
            .into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let mut sda =
        gpiob
            .pb7
            .into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    //scl.internal_pull_up(&mut gpiob.pupdr, true);
    //sda.internal_pull_up(&mut gpiob.pupdr, true);
    let mut i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        100u32.khz(),
        clocks,
        &mut rcc.apb1,
    );

    // configure UART -> for virtual COM port
    // switch tx and rx pin to alternative function
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let pa2_tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
    let pa3_rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

    // create UART struct
    let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), UART_BAUDRATE.bps(), clocks, &mut rcc.apb1);
    // get tx and rx structs
    let (mut tx, mut _rx) = uart.split();

    //hprintln!("Start i2c scanning...").expect("Error using hprintln.");
    tx.bwrite_all("Starting i2c scanning...\r\n".as_bytes()).unwrap();
    //hprintln!().unwrap();

    for addr in 0x00_u8..0x80 {
            // Write the empty array and check the slave response.
            if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok() {
                  //hprint!("{:02x}", addr).unwrap();
                  tx.bwrite_all(String::<5>::from(addr).as_bytes()).unwrap();
            } else {
                  //hprint!("..").unwrap();
                  tx.bwrite_all("..".as_bytes()).unwrap();
            }
            if addr % 0x10 == 0x0F {
                  //hprintln!().unwrap();
                  tx.bwrite_all("\r\n".as_bytes()).unwrap();
            } else {
                  //hprint!(" ").unwrap();
                  tx.bwrite_all(" ".as_bytes()).unwrap();
            }
    }

    //hprintln!().unwrap();
    //hprintln!("Done!").unwrap();

    loop {
        asm::wfi();
    }
}
