//! Example for OLED-Display (ssd1306, https://www.waveshare.com/0.96inch-oled-b.htm) over SPI
//! Target board: STM32F303RE Nucleo
// see also: https://github.com/jamwaffles/ssd1306/blob/55bc848f79e34631b920efb44aec17059343eaf7/examples/graphics.rs, https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/spi.rs

#![no_std]
#![no_main]

use embedded_hal::spi::Mode;
use hal::spi::Spi;
use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f3xx_hal::{self as hal, pac, prelude::*, delay::Delay};

use ssd1306::{prelude::*, Builder};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::PrimitiveStyleBuilder,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = pac::CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // configure pins for SPI diplay
    let sck = gpiob.pb3.into_af5(&mut gpiob.moder, &mut gpiob.afrl);
    let miso = gpiob.pb4.into_af5(&mut gpiob.moder, &mut gpiob.afrl); // not used because display does not send data
    let mosi = gpiob.pb5.into_af5(&mut gpiob.moder, &mut gpiob.afrl);

    let mut disp_reset = gpioa.pa8.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let dc = gpiob.pb10.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    let mut disp_chip_select = gpioa.pa10.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    // i do not understand why it is required to explicitly provide the type, but WORD has to be set (to u8)
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

    disp_chip_select.set_low().unwrap();

    // configure display
    let interface = SPIInterfaceNoCS::new(spi, dc);
    let mut disp: GraphicsMode<_,_> = Builder::new().connect(interface).into();

    disp.reset(&mut disp_reset, &mut delay).unwrap();
    disp.init().unwrap();

    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // square
    Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(96, yoffset + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();
    disp_chip_select.set_low().unwrap();

    loop {
        asm::delay(1_000_000);
    }
}