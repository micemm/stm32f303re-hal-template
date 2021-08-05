use embedded_hal::blocking::spi::Write;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

use ssd1306::mode::GraphicsMode;
use ssd1306::{prelude::*, Builder};
use embedded_graphics::{
    pixelcolor::BinaryColor, 
    prelude::*, 
    primitives::{Circle, Rectangle, Triangle}, 
    style::PrimitiveStyleBuilder,
    style::TextStyleBuilder,
    fonts::{Font12x16, Text},
    image::{Image, ImageRaw}
};
use heapless::String;

pub struct OLEDDisplay<SPIWRITE: Write<u8>, DCPIN: OutputPin, CSPIN: OutputPin>{
    disp: GraphicsMode<SPIInterface<SPIWRITE, DCPIN, CSPIN>, DisplaySize128x64>
}

impl <SPIWRITE: Write<u8>, DCPIN: OutputPin, CSPIN: OutputPin> OLEDDisplay<SPIWRITE, DCPIN, CSPIN> {
    pub fn new(spi: SPIWRITE, dc_pin: DCPIN, cs_pin: CSPIN, reset_pin: &mut impl OutputPin, delay: &mut impl DelayMs<u8>) -> Self{
        let interface = SPIInterface::new(spi, dc_pin, cs_pin);
        let disp = Builder::new().connect(interface).into();
        let mut display = OLEDDisplay{disp};
        if !display.disp.reset(reset_pin, delay).is_ok(){
            //  TODO: handle error
        }
        display.disp.init().unwrap();
        display.show_rust_logo();
        display
    }

    pub fn show_rust_logo(&mut self){
        self.disp.clear();
        let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../../res/rust.raw"), 64, 64);
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
