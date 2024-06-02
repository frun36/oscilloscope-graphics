use fixed::{
    consts::PI,
    traits::{LossyFrom, LossyInto},
};
use rp_pico::hal::pwm::AnySlice;

use crate::{Coordinate, Parameter};

use super::Drawable;

pub struct Parametric<F> {
    t_0: Parameter,
    t_step: Parameter,
    t_1: Parameter,
    before_us: u32,
    us: u32,
    after_us: u32,
    function: F,
}

impl<F> Parametric<F>
where
    F: Fn(Parameter) -> (Coordinate, Coordinate),
{
    pub fn path(
        t_0: Parameter,
        t_step: Parameter,
        t_1: Parameter,
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

impl<F> Drawable for Parametric<F>
where
    F: Fn(Parameter) -> (Coordinate, Coordinate),
{
    fn draw<S: AnySlice>(&self, display: &mut crate::display::Display<S>) {
        let (x, y) = (self.function)(self.t_0);
        display.set_position(x, y);
        display.wait_us(self.before_us);

        let mut t = self.t_0 + self.t_step;
        if self.t_step.is_positive() {
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

impl Parametric<fn(Parameter) -> (Coordinate, Coordinate)> {
    pub fn arc(
        o: (Coordinate, Coordinate),
        r: Coordinate,
        t_0: Parameter,
        t_step: Parameter,
        t_1: Parameter,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> Parametric<impl Fn(Parameter) -> (Coordinate, Coordinate)> {
        let f = move |t: Parameter| {
            let (y, x) = cordic::sin_cos(t);
            let (x, y): (f32, f32) = (x.lossy_into(), y.lossy_into());
            let (x, y) = (Coordinate::from_num(x), Coordinate::from_num(y));
            (r * (x.saturating_add(o.0)), r * (y.saturating_add(o.1)))
        };
        Parametric {
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
        o: (Coordinate, Coordinate),
        r: Coordinate,
        t_step: Parameter,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> Parametric<impl Fn(Parameter) -> (Coordinate, Coordinate)> {
        Self::arc(
            o,
            r,
            Parameter::from_num(0),
            t_step,
            Parameter::from_num(6.28318530718),
            before_us,
            us,
            after_us,
        )
    }

    pub fn segment(
        a: (Coordinate, Coordinate),
        b: (Coordinate, Coordinate),
        t_step: Parameter,
        before_us: u32,
        us: u32,
        after_us: u32,
    ) -> Parametric<impl Fn(Parameter) -> (Coordinate, Coordinate)> {
        let function = move |t: Parameter| {
            let t: f32 = t.lossy_into();
            let t = Coordinate::from_num(t);
            (a.0 + t * (b.0 - a.0), a.1 + t * (b.1 - a.1))
        };

        Parametric {
            t_0: Parameter::from_num(0),
            t_step,
            t_1: Parameter::from_num(1),
            before_us,
            us,
            after_us,
            function,
        }
    }

    pub fn point(
        p: (Coordinate, Coordinate),
        us: u32,
    ) -> Parametric<impl Fn(Parameter) -> (Coordinate, Coordinate)> {
        let function = move |_t| (p.0, p.1);

        Parametric {
            t_0: Parameter::from_num(0),
            t_step: Parameter::from_num(1),
            t_1: Parameter::from_num(0),
            before_us: us,
            us: 0,
            after_us: 0,
            function,
        }
    }
}
