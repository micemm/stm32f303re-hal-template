#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

// blink LED on PA5

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      
      // porta_5 is on board LED on nucleo 64 board
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
      let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
      
      loop {
            led.set_high().unwrap();
            asm::delay(2_000_000);
            led.set_low().unwrap();
            asm::delay(2_000_000);
      }
}
