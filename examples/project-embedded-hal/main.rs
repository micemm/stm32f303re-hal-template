#![no_std]
#![no_main]

mod i2c_led_bar;
mod spi_oled_display;
use i2c_led_bar::LEDBar;
use spi_oled_display::OLEDDisplay;

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3xx_hal::{self as hal, pac, prelude::*, delay::Delay};
use embedded_hal::spi::Mode;
use hal::spi::Spi;

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

    // Configure I2C
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (scl, sda),
        100u32.khz(),
        clocks,
        &mut rcc.apb1,
    );

    // configure SPI and pins for display
    let sck = gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
    let miso = gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl); // not used because display does not send data
    let mosi = gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
    let mut disp_reset = gpioa.pa8.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let dc = gpiob.pb10.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let disp_chip_select = gpioa.pa10.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let spi: Spi<pac::SPI1, (stm32f3xx_hal::gpio::gpiob::PB3<stm32f3xx_hal::gpio::AF5>, stm32f3xx_hal::gpio::gpiob::PB4<stm32f3xx_hal::gpio::AF5>, stm32f3xx_hal::gpio::gpiob::PB5<stm32f3xx_hal::gpio::AF5>), u8> = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi), 
        Mode{
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        },
        8.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    // objects for ADC
    let mut adc_in_pin = gpioa.pa0.into_analog(&mut gpioa.moder, &mut gpioa.pupdr); // PA0
    
    // setup adc
    let mut adc1 = hal::adc::Adc::adc1(
        dp.ADC1,
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        hal::adc::CkMode::default(),
        clocks
    );
    
    // create objects to intercat with hardware
    let mut display = OLEDDisplay::new(spi, dc, disp_chip_select, &mut disp_reset, &mut delay);
    asm::delay(1_000_000);
    display.show_text("Start");

    let mut led_bar = LEDBar::new(i2c, I2C_ADDR);
    led_bar.write_percentage(50).unwrap();

    loop {
        let adc_in_data: u16 = adc1.read(&mut adc_in_pin).expect("Error reading from ADC");
        let percentage = (((adc_in_data as f32) / (((0x1 << 12) - 1) as f32) * 100_f32) + 0.5_f32) as u8; // 12 bit ADC
        led_bar.write_percentage(percentage).unwrap();
        let mut value_string = String::<30>::from(percentage);
        value_string.push('%').unwrap();
        display.show_text(value_string.as_str());
        asm::delay(1_000_000);
    }
}
