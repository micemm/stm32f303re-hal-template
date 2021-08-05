use ssd1306::mode::GraphicsMode;
use ssd1306::{prelude::*, Builder};
use embedded_graphics::{
    pixelcolor::BinaryColor, 
    prelude::*,
    style::TextStyleBuilder,
    fonts::{Font12x16, Text},
    image::{Image, ImageRaw}
};

use embedded_hal::spi::Mode;

use stm32f3xx_hal::{
    pac,
    prelude::*,
    delay::Delay,
    spi::Spi,
    gpio::{AF5, Output, PushPull, gpioa::{PA8, PA10}, gpiob::{PB3, PB4, PB5, PB10}},
    rcc::{Clocks, APB2}
};

pub struct OLEDDisplay{
    disp: GraphicsMode<
            SPIInterface<
                Spi<
                    pac::SPI1,
                    (PB3<AF5>, PB4<AF5>, PB5<AF5>),
                    u8
                >,
                PB10<Output<PushPull>>,
                PA10<Output<PushPull>>
            >,
        DisplaySize128x64>
}

impl OLEDDisplay{
    pub fn new(sck: PB3<AF5>, miso: PB4<AF5>, mosi: PB5<AF5>, mut reset_pin: PA8<Output<PushPull>>, dc_pin: PB10<Output<PushPull>>, cs_pin: PA10<Output<PushPull>>, spi: pac::SPI1, delay: &mut Delay, rcc_apb2: &mut APB2, clocks: Clocks) -> Self{

        let spi: Spi<pac::SPI1, (PB3<AF5>, PB4<AF5>, PB5<AF5>), u8> = Spi::spi1(
            spi,
            (sck, miso, mosi), 
            Mode{
                polarity: embedded_hal::spi::Polarity::IdleLow,
                phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            },
            8.mhz(),
            clocks,
            rcc_apb2,
        );

        let interface = SPIInterface::new(spi, dc_pin, cs_pin);
        let disp = Builder::new().connect(interface).into();
        let mut display = OLEDDisplay{disp};
        if !display.disp.reset(&mut reset_pin, delay).is_ok(){
            //  TODO: handle error
        }
        display.disp.init().unwrap();
        display.show_rust_logo();
        display
    }

    pub fn show_rust_logo(&mut self){
        self.disp.clear();
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../res/rust.raw"), 64, 64);
        let im = Image::new(&raw, Point::new(32, 0));
        im.draw(&mut self.disp).unwrap();
        self.disp.flush().unwrap();
    }

    pub fn show_text(&mut self, text: &str){
        self.disp.clear();
        
        let text_style = TextStyleBuilder::new(Font12x16)
            .text_color(BinaryColor::On)
            .build();
        
        Text::new(text, Point::new(0, 16))
            .into_styled(text_style)
            .draw(&mut self.disp)
            .unwrap();
        
        self.disp.flush().unwrap();
    }
}
