use embedded_hal::blocking::i2c::Write;

use hal::prelude::_stm32f3xx_hal_time_U32Ext;
use stm32f3xx_hal::{
    self as hal,
    i2c::I2c,
    pac,
    gpio::{AF4, gpiob::{PB6, PB7}},
    rcc::{Clocks, APB1}
};

pub struct LEDBar{
    i2c: I2c<pac::I2C1, (PB6<AF4>, PB7<AF4>)>,
    i2c_addr: u8
}

impl LEDBar{
    pub fn new(scl: PB6<AF4>, sda: PB7<AF4>, i2c_module: pac::I2C1, clocks: Clocks, rcc_apb1: &mut APB1, i2c_addr: u8) -> Self{
        let i2c = I2c::new(
            i2c_module,
            (scl, sda),
            100u32.khz(),
            clocks,
            rcc_apb1);
        LEDBar{i2c, i2c_addr}
    }

    pub fn write_percentage(&mut self, percentage: u8) -> Result<(), hal::i2c::Error>{
        let value = !((0x01 << (percentage as u16 / 13)) * 2 - 1) as u8;
        self.i2c.write(self.i2c_addr, &[value])
    }
}
