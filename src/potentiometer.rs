use stm32f3xx_hal::{
    self as hal,
    adc::Adc,
    pac::{ADC1, ADC1_2},
    rcc::{AHB, Clocks},
    gpio::{Analog, gpioa::PA0}
};
use embedded_hal::adc::OneShot;

pub struct Potentiometer{
    adc: Adc<ADC1>,
    analog_in: PA0<Analog>
}

impl Potentiometer{
    pub fn new(adc_module: ADC1, adc1_2: &mut ADC1_2, rcc_ahb: &mut AHB, clocks: Clocks, analog_in: PA0<Analog>) -> Self{
        let adc = Adc::adc1(
            adc_module,
            adc1_2,
            rcc_ahb,
            hal::adc::CkMode::default(),
            clocks);
        Potentiometer{adc, analog_in}
    }

    pub fn read_percentage(&mut self) -> u8{
        let adc_in_data: u16 = self.adc.read(&mut self.analog_in).expect("Error reading from ADC");
        let percentage = (((adc_in_data as f32) / (((0x1 << 12) - 1) as f32) * 100_f32) + 0.5_f32) as u8; // 12 bit ADC
        percentage
    }
}
