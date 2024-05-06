use core::convert::Infallible;

use hal::pwm::{AnySlice, Channel, A, B};
use rp2040_hal as hal;

use embedded_hal::pwm::SetDutyCycle;

use crate::TOP;

pub struct Display<'a, S>
where
    S: AnySlice,
{
    pwm_channel_x: &'a mut Channel<S, A>,
    pwm_channel_y: &'a mut Channel<S, B>,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl<'a, S> Display<'a, S>
where
    S: AnySlice,
{
    pub fn new(
        pwm_channel_x: &'a mut Channel<S, A>,
        pwm_channel_y: &'a mut Channel<S, B>,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
    ) -> Self {
        Self {
            pwm_channel_x,
            pwm_channel_y,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn coord_to_x_duty_cycle(&self, x: f32) -> u16 {
        let trimmed = x.max(self.x_min).min(self.x_max);
        let shifted = trimmed - self.x_min;
        let normalized = shifted / (self.x_max - self.x_min);
        (normalized * TOP as f32) as u16
    }

    fn coord_to_y_duty_cycle(&self, y: f32) -> u16 {
        let trimmed = y.max(self.y_min).min(self.y_max);
        let shifted = trimmed - self.y_min;
        let normalized = shifted / (self.y_max - self.y_min);
        (normalized * TOP as f32) as u16
    }

    pub fn set_position(&mut self, x: f32, y: f32) -> Result<(), Infallible> {
        self.pwm_channel_x.set_duty_cycle(self.coord_to_x_duty_cycle(x))?;
        self.pwm_channel_y.set_duty_cycle(self.coord_to_y_duty_cycle(y))?;
        Ok(())
    }
}
