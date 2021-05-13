#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use hal::{pwm::{PwmChannel, TIM3_CH1, TIM3_CH2, WithPins}};
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*, pwm};

// pwm for leds on PA6 and PA7
// see also: https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/pwm.rs

// duty cylce in percent
const DUTY_1_PERCENT: u16 = 60;
const DUTY_2_PERCENT: u16 = 50;

const DELAY: u32 = 100_000;

fn set_duty_cycles(tim3_ch1: &mut PwmChannel<TIM3_CH1, WithPins>, tim3_ch2: &mut PwmChannel<TIM3_CH2, WithPins>, ch1_duty_percent: u16, ch2_duty_percent: u16){
      tim3_ch1.set_duty(tim3_ch1.get_max_duty() / 100 * ch1_duty_percent);
      tim3_ch2.set_duty(tim3_ch2.get_max_duty() / 100 * ch2_duty_percent);
}

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut flash = dp.FLASH.constrain();

      // set system clock frequency
      let clocks = rcc.cfgr.sysclk(16u32.mhz()).freeze(&mut flash.acr);
      
      // configure PA6 and PA7 as TIMER3_CH1 and TIMER3_CH2 (= AF2)
      let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
      let pa6 = gpioa.pa6.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
      let pa7 = gpioa.pa7.into_af2(&mut gpioa.moder, &mut gpioa.afrl);
      
      // create timer 3 struct
      let tim3_channels = pwm::tim3(
            dp.TIM3,
            1280,         // resolution of duty cycle
            50u32.hz(),  // frequency of period
            &clocks,          // To get the timer's clock speed
      );

      // create timer cahnnels
      let mut tim3_ch1 = tim3_channels.0.output_to_pa6(pa6);
      tim3_ch1.set_duty(tim3_ch1.get_max_duty() / 100 * DUTY_1_PERCENT); // set duty cylce
      tim3_ch1.enable();

      let mut tim3_ch2 = tim3_channels.1.output_to_pa7(pa7);
      tim3_ch2.set_duty(tim3_ch2.get_max_duty() / 100 * DUTY_2_PERCENT); // set duty cycle
      tim3_ch2.enable();


      // example appliactoion: create "glow effect" by periodically changing the duty cycle
      let mut duty1_percent: u16 = 0;
      let mut duty2_percent: u16 = 0;
      loop {
            while duty1_percent < 99 {
                  duty1_percent += 1;
                  set_duty_cycles(&mut tim3_ch1, &mut tim3_ch2, duty1_percent, duty2_percent);
                  asm::delay(DELAY);
            }
            while duty2_percent < 100 {
                  duty2_percent += 1;
                  set_duty_cycles(&mut tim3_ch1, &mut tim3_ch2, duty1_percent, duty2_percent);
                  asm::delay(DELAY);
            }
            while duty1_percent > 0 {
                  duty1_percent -= 1;
                  set_duty_cycles(&mut tim3_ch1, &mut tim3_ch2, duty1_percent, duty2_percent);
                  asm::delay(DELAY);
            }
            while duty2_percent > 0 {
                  duty2_percent -= 1;
                  set_duty_cycles(&mut tim3_ch1, &mut tim3_ch2, duty1_percent, duty2_percent);
                  asm::delay(DELAY);
            }
      }
}