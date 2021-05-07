#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
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

      let mut tx_string: String<30> = String::from("ADC example\r\n");
      
      loop {
            let adc_in_data: u16 = adc1.read(&mut adc_in_pin).expect("Error reading from ADC");
            let tx_buffer = tx_string.as_bytes();
            tx.bwrite_all(tx_buffer).unwrap();
            tx.bwrite_all("\r\n".as_bytes()).unwrap();
            asm::delay(1_000_000);
            tx_string = String::<30>::from(adc_in_data);
      }
}