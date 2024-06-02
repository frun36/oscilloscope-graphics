use cortex_m::delay::Delay;
use rp_pico::hal::{
    adc::AdcFifo,
    pwm::{AnySlice, Channel, A, B},
};

use embedded_hal::pwm::SetDutyCycle;

use crate::{drawable::Drawable, Coordinate};

pub struct Display<'a, S>
where
    S: AnySlice,
{
    pwm_channel_x: &'a mut Channel<S, A>,
    pwm_channel_y: &'a mut Channel<S, B>,
    delay: &'a mut Delay,
    adc_fifo: AdcFifo<'a, u16>,
}

impl<'a, S> Display<'a, S>
where
    S: AnySlice,
{
    pub fn new(
        pwm_channel_x: &'a mut Channel<S, A>,
        pwm_channel_y: &'a mut Channel<S, B>,
        delay: &'a mut Delay,
        adc_fifo: AdcFifo<'a, u16>,
    ) -> Self {
        Self {
            pwm_channel_x,
            pwm_channel_y,
            delay,
            adc_fifo,
        }
    }

    fn coord_to_x_duty_cycle(&self, x: Coordinate) -> u16 {
        let bits = x.to_bits();
        (bits as i16 + 128) as u16
    }

    fn coord_to_y_duty_cycle(&self, y: Coordinate) -> u16 {
        let y = y * Coordinate::from_num(4. / 3.);
        let bits = y.to_bits();
        (bits as i16 + 128) as u16
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        let (x, y) = (
            Coordinate::saturating_from_num(x),
            Coordinate::saturating_from_num(y),
        );
        let _ = self
            .pwm_channel_x
            .set_duty_cycle(self.coord_to_x_duty_cycle(x));
        let _ = self
            .pwm_channel_y
            .set_duty_cycle(self.coord_to_y_duty_cycle(y));
    }

    pub fn draw<D: Drawable>(&mut self, img: &D) {
        img.draw(self);
    }

    pub fn wait_us(&mut self, us: u32) {
        self.delay.delay_us(us);
    }

    pub fn read_knob(&mut self) -> Option<f32> {
        if self.adc_fifo.len() > 0 {
            let adc_counts: u16 = self.adc_fifo.read();
            Some(adc_counts as f32 / 4096.)
        } else {
            None
        }
    }
}
