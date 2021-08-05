use embedded_hal::PwmPin;
use stm32f3xx_hal::{
    prelude::*,
    pwm::{PwmChannel, TIM3_CH1, TIM3_CH2, WithPins},
    gpio::{AF2, gpioa::{PA6, PA7}},
    pac::TIM3,
    rcc::Clocks,
    pwm
};

pub struct TwoColorLED
{
    pwm1: PwmChannel<TIM3_CH1, WithPins>,
    pwm2: PwmChannel<TIM3_CH2, WithPins>
}

impl TwoColorLED
{
    pub fn new(timer_module: TIM3, pin1: PA6<AF2>, pin2: PA7<AF2>, clocks: &Clocks) -> Self{
        let tim3_channels = pwm::tim3(
            timer_module,
            1280,
            50u32.hz(),
            clocks
        );
        let mut pwm1 = tim3_channels.0.output_to_pa6(pin1);
        let mut pwm2 = tim3_channels.1.output_to_pa7(pin2);
        pwm1.enable();
        pwm2.enable();
        TwoColorLED{pwm1, pwm2}
    }

    pub fn write_percentage(&mut self, percentage: u8)
    {
        let percentage1 = percentage as u16;
        let percentage2 = 100 - percentage1;
        self.pwm1.set_duty(self.pwm1.get_max_duty() / 100 * percentage1);
        self.pwm2.set_duty(self.pwm2.get_max_duty() / 100 * percentage2);
    }
}
