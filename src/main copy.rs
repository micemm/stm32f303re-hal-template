#![no_std]
#![no_main]

use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
use hal::gpio;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
use heapless::String;

// use adc to read analog value and print it to uart (virtual com port)

#[entry]
fn main() -> ! {
      let mut dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut flash = dp.FLASH.constrain();

      let clocks = rcc.cfgr.freeze(&mut flash.acr);

      // porta_5 is on board LED on nucleo 64 board
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
      // uart pins
      let pa2_tx = gpioa.pa2.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
      let pa3_rx = gpioa.pa3.into_af7(&mut gpioa.moder, &mut gpioa.afrl);
      // adc pin
      let mut adc_in_pin = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr); // PA0

      // setup adc
      let mut adc1 = hal::adc::Adc::adc1(
            dp.ADC1,
            &mut dp.ADC1_2,
            &mut rcc.ahb,
            hal::adc::CkMode::default(),
            clocks
      );

      // setup uart
      let uart = hal::serial::Serial::usart2(dp.USART2, (pa2_tx, pa3_rx), 9600.bps(), clocks, &mut rcc.apb1);
      let (mut tx, mut _rx) = uart.split();
      
      let dma1 = dp.DMA1.split(&mut rcc.ahb);
      let mut tx_channel = dma1.ch7;
      
      let mut sending;
      let mut tx_string: String<30> = String::from("ADC example\r\n");
      let slice_array= [0x41_u8, 0x42_u8, 0x43_u8, 0x00_u8];
      // let mut tx_buffer = singleton!(: [u8; 30] = *b"starting ADC sample...").unwrap();
      let mut tx_string = singleton!(: String<30> = String::from("hello world")).unwrap();
      
      loop {
            let mut adc_in_data: u16 = 0;
            {
                  let mut tx_buffer = tx_string.as_bytes();
                  sending = tx.write_all(tx_buffer, tx_channel);
                  adc_in_data = adc1.read(&mut adc_in_pin).expect("Error reading from ADC");
                  asm::delay(4_000_000);
                  // ownership has to be returned
                  let (tx_buffer_ret, tx_channel_ret, tx_ret) = sending.wait();
                  tx = tx_ret;
                  tx_channel = tx_channel_ret;
                  //tx_buffer = tx_buffer_ret;
            }
            
            *tx_string = String::<30>::from(adc_in_data);
      }
}