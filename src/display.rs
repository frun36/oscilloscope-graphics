use cortex_m::delay::Delay;
use rp_pico::hal::pwm::{AnySlice, Channel, A, B};

use embedded_hal::pwm::SetDutyCycle;

use crate::drawable::Drawable;

pub struct Display<'a, S>
where
    S: AnySlice,
{
    pwm_channel_x: &'a mut Channel<S, A>,
    pwm_channel_y: &'a mut Channel<S, B>,
    pwm_top: u16,
    x_min: f32,
    x_max: f32,
    x_size: f32,
    y_min: f32,
    y_max: f32,
    y_size: f32,
    delay: &'a mut Delay,
}

impl<'a, S> Display<'a, S>
where
    S: AnySlice,
{
    pub fn new(
        pwm_channel_x: &'a mut Channel<S, A>,
        pwm_channel_y: &'a mut Channel<S, B>,
        pwm_top: u16,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        delay: &'a mut Delay,
    ) -> Self {
        Self {
            pwm_channel_x,
            pwm_channel_y,
            pwm_top,
            x_min,
            x_max,
            x_size: x_max - x_min,
            y_min,
            y_max,
            y_size: y_max - y_min,
            delay,
        }
    }

    fn coord_to_x_duty_cycle(&self, x: f32) -> u16 {
        // let trimmed = x.max(self.x_min).min(self.x_max);
        let shifted = x - self.x_min;
        let normalized = shifted / self.x_size;
        (normalized * self.pwm_top as f32) as u16
    }

    fn coord_to_y_duty_cycle(&self, y: f32) -> u16 {
        // let trimmed = y.max(self.y_min).min(self.y_max);
        let shifted = y - self.y_min;
        let normalized = shifted / self.y_size;
        (normalized * self.pwm_top as f32) as u16
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
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
}
