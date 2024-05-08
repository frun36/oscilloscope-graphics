use rp2040_hal::pwm::AnySlice;

use super::Drawable;

pub struct ParametricPath<F>
where
    F: Fn(f32) -> (f32, f32),
{
    t_0: f32,
    t_step: f32,
    t_1: f32,
    us: u32,
    function: F,
}

impl<F> ParametricPath<F>
where
    F: Fn(f32) -> (f32, f32),
{
    pub fn new(t_0: f32, t_step: f32, t_1: f32, us: u32, function: F) -> Self {
        Self {
            t_0,
            t_step,
            t_1,
            us,
            function,
        }
    }
}

impl<F> Drawable for ParametricPath<F>
where
    F: Fn(f32) -> (f32, f32),
{
    fn draw<S: AnySlice>(&self, display: &mut crate::display::Display<S>) {
        let mut t = self.t_0;
        while t <= self.t_1 {
            let (x, y) = (self.function)(t);
            display.set_position(x, y);
            display.wait_us(self.us);
            t += self.t_step;
        }
    }
}
