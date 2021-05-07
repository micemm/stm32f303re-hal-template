#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

// https://github.com/stm32-rs/stm32f3xx-hal/blob/v0.6.0/examples/serial_dma.rs

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut flash = dp.FLASH.constrain();

      let clocks = rcc.cfgr.freeze(&mut flash.acr);

      // porta_5 is on board LED on nucleo 64 board
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
      let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
      let pa2_tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
      let pa3_rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);

      let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), 9600.bps(), clocks, &mut rcc.apb1);
      let (mut tx, mut _rx) = uart.split();
      
      let dma1 = dp.DMA1.split(&mut rcc.ahb);
      let mut tx_channel = dma1.ch7;
      let mut tx_buffer = "testDMA\r\n".as_bytes();
      
      let mut sending;
      
      loop {
            sending = tx.write_all(tx_buffer, tx_channel);
            asm::delay(4_000_000);
            // ownership has to be returned 
            let (tx_buffer_ret, tx_channel_ret, tx_ret) = sending.wait();
            tx = tx_ret;
            tx_channel = tx_channel_ret;
            tx_buffer = tx_buffer_ret;
            led.toggle().unwrap();
            tx.bwrite_all("test\r\n".as_bytes()).unwrap(); // write blocking
      }
}