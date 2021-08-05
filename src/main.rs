#![no_std]
#![no_main]

mod i2c_led_bar;
mod spi_oled_display;
mod pwm_led;
mod potentiometer;
use i2c_led_bar::LEDBar;
use spi_oled_display::OLEDDisplay;
use potentiometer::Potentiometer;
use pwm_led::TwoColorLED;

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3xx_hal::{pac, prelude::*, delay::Delay};

use heapless::String;

const I2C_ADDR: u8 = 32;

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    
    let clocks = rcc
        .cfgr
        .sysclk(72u32.mhz())
        .pclk1(36u32.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let mut delay = Delay::new(cp.SYST, clocks);

    // Display (SPI)
    let mut display = OLEDDisplay::new(
        gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl),
        gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl),
        gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl),
        gpioa.pa8.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        gpiob.pb10.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper),
        gpioa.pa10.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper),
        dp.SPI1,
        &mut delay,
        &mut rcc.apb2,
        clocks
    );

    // LED bar on GPIO extension (I2C)
    let mut led_bar = LEDBar::new(
        gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl),
        gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl),
        dp.I2C1,
        clocks,
        &mut rcc.apb1,
        I2C_ADDR
    );

    // Potentiometer
    let mut potentiometer = Potentiometer::new(
        dp.ADC1,
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        clocks,
        gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr)
    );

    // two color LED
    let mut two_color_led = TwoColorLED::new(
        dp.TIM3,
        gpioa.pa6.into_af2(&mut gpioa.moder, &mut gpioa.afrl),
        gpioa.pa7.into_af2(&mut gpioa.moder, &mut gpioa.afrl),
        &clocks
    );
    
    asm::delay(1_000_000);
    display.show_text("Start");

    led_bar.write_percentage(50).unwrap();

    loop {
        asm::nop();
        let percentage = potentiometer.read_percentage();
        led_bar.write_percentage(percentage).unwrap();
        two_color_led.write_percentage(percentage);
        let mut value_string = String::<30>::from(percentage);
        value_string.push('%').unwrap();
        display.show_text(value_string.as_str());
        asm::delay(1_000_000);
    }
}
