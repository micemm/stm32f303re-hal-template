use embedded_hal::blocking::i2c::Write;
pub struct LEDBar<I2CWRITE: Write>{
    i2c: I2CWRITE,
    i2c_addr: u8
}

impl <I2CWRITE: Write> LEDBar<I2CWRITE>{
    pub fn new(i2c: I2CWRITE, i2c_addr: u8) -> Self{
        LEDBar{i2c, i2c_addr}
    }

    pub fn write_percentage(&mut self, percentage: u8) -> Result<(), I2CWRITE::Error>{
        let value = !((0x01 << (percentage as u16 / 13)) * 2 - 1) as u8;
        self.i2c.write(self.i2c_addr, &[value])
    }
}
