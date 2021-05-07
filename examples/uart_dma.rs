#![no_std]
#![no_main]

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

// write to uart using DMA
// see also:
// https://github.com/stm32-rs/stm32f3xx-hal/blob/v0.6.0/examples/serial_dma.rs

const TX_BUFFER_LENGTH: usize = 100;

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
      let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), 9600.bps(), clocks, &mut rcc.apb1);
      // get tx and rx structs
      let (mut tx, mut _rx) = uart.split();
      
      // create DMA struct
      let dma1 = dp.DMA1.split(&mut rcc.ahb);
      // dma channel for tx (number: see datasheet)
      let mut tx_channel = dma1.ch7;
      
      // create DMA buffer for tx
      // This buffer has to be a slice (&[u8]) with static lifetime. This con be achived by:
      // 1) String literals: (Problem: only constant data can be written)
      // let mut tx_slice = "testDMA\r\n".as_bytes();
      
      // 2) use the singleton!() macro to create a buffer of fixed length
      // (note: make sure that this line is only executed once as it will not create a second buffer but return 'None' if it is called again)
      // This creates an array of fixed size that can be used as buffer. 
      // As the buffer is passed as slice (&[u8]) when transmitting data, it is also possible to write only parts of the array:
      let tx_buffer = singleton!(: [u8; TX_BUFFER_LENGTH] = [0; TX_BUFFER_LENGTH]).unwrap();

      // exapmle: write "Hello World!" using this buffer:
      let bytes = "Hello world!\r\n".as_bytes();
      // copy all values to the buffer:
      // for (i, value) in bytes.iter().enumerate(){
      //       tx_buffer[i] = *value; // copy the value to the buffer
      // }
      // or without loop:
      tx_buffer[..bytes.len()].clone_from_slice(bytes);

      let mut tx_slice = &tx_buffer[..bytes.len()]; // take only the part of the buffer with values
      
      loop {
            let sending = tx.write_all(tx_slice, tx_channel);
            // do other stuff, data is being sent in the background
            // ownership has to be returned -> wait until the data transfer is finished
            let (tx_slice_ret, tx_channel_ret, tx_ret) = sending.wait();
            tx = tx_ret;
            tx_channel = tx_channel_ret;
            tx_slice = tx_slice_ret;
            asm::delay(4_000_000);
      }
}