#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use hal::{pac::USART2, serial::Rx};
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
// read data coming in on uart and write it to uart
// https://github.com/stm32-rs/stm32f3xx-hal/blob/v0.6.0/examples/serial_dma.rs

const BAUDRATE: u32 = 9600;

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut flash = dp.FLASH.constrain();

      let clocks = rcc.cfgr.freeze(&mut flash.acr);
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

      // switch tx and rx pin to alternative function
      let pa2_tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
      let pa3_rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

      // create UART struct
      let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), BAUDRATE.bps(), clocks, &mut rcc.apb1);
      // get tx and rx structs
      let (mut tx, mut rx) = uart.split();
      
      loop {
            let c = rx.read(); // can be ok or error (if there was no data to read, error is returned)
            if c.is_ok(){ // if the result is ok, there was data -> write the data to uart
                  tx.write(c.unwrap()).unwrap();
            }
      }
}