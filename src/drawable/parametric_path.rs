use core::f32::consts::PI;

use rp_pico::hal::pwm::AnySlice;

use super::Drawable;

pub struct ParametricPath<F> {
    t_0: f32,
    t_step: f32,
    t_1: f32,
    before_us: u32,
    us: u32,
    after_us: u32,
    function: F,
}

impl<F> ParametricPath<F>
where
    F: Fn(f32) -> (f32, f32),
{
    pub fn new(
        t_0: f32,
        t_step: f32,
        t_1: f32,
        before_us: u32,
        us: u32,
        after_us: u32,
        function: F,
    ) -> Self {
        Self {
            t_0,
            t_step,
            t_1,
            before_us,
            us,
            after_us,
            function,
        }
    }
}

impl<F> Drawable for ParametricPath<F>
where
    F: Fn(f32) -> (f32, f32),
{
    fn draw<S: AnySlice>(&self, display: &mut crate::display::Display<S>) {
        let (x, y) = (self.function)(self.t_0);
        display.set_position(x, y);
        display.wait_us(self.before_us);

        let mut t = self.t_0 + self.t_step;
        if self.t_step.is_sign_positive() {
            while t <= self.t_1 {
                let (x, y) = (self.function)(t);
                display.set_position(x, y);
                display.wait_us(self.us);
                t += self.t_step;
            }
        } else {
            while t >= self.t_1 {
                let (x, y) = (self.function)(t);
                display.set_position(x, y);
                display.wait_us(self.us);
                t += self.t_step;
            }
        }

        display.wait_us(self.after_us);
    }
}

impl ParametricPath<fn(f32) -> (f32, f32)> {
    pub fn arc(
        o: (f32, f32),
        r: f32,
        t_0: f32,
        t_step: f32,
        t_1: f32,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> ParametricPath<impl Fn(f32) -> (f32, f32)> {
        let f = move |t| {
            let (y, x) = libm::sincosf(t);
            (r * (x + o.0), r * (y + o.1))
        };
        ParametricPath {
            t_0,
            t_step,
            t_1,
            before_us,
            us,
            after_us,
            function: f,
        }
    }

    pub fn circle(
        o: (f32, f32),
        r: f32,
        t_step: f32,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> ParametricPath<impl Fn(f32) -> (f32, f32)> {
        Self::arc(o, r, 0., t_step, 2. * PI, before_us, us, after_us)
    }

    pub fn segment(
        a: (f32, f32),
        b: (f32, f32),
        t_step: f32,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> ParametricPath<impl Fn(f32) -> (f32, f32)> {
        let function = move |t| (a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1));

        ParametricPath {
            t_0: 0.,
            t_step,
            t_1: 1.,
            before_us,
            us,
            after_us,
            function,
        }
    }
}
