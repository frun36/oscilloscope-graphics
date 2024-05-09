use core::f32::consts::PI;

use rp2040_hal::pwm::AnySlice;

use super::Drawable;

pub struct ParametricPath<F> {
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

impl ParametricPath<fn(f32) -> (f32, f32)> {
    pub fn circle(
        o: (f32, f32),
        r: f32,
        t_step: f32,
        us: u32,
    ) -> ParametricPath<impl Fn(f32) -> (f32, f32)> {
        let f = move |t| {
            let (y, x) = libm::sincosf(t);
            (r * (x - o.0), r * (y - o.1))
        };
        ParametricPath {
            t_0: 0.,
            t_step,
            t_1: 2. * PI,
            us,
            function: f,
        }
    }
}
