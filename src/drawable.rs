use rp_pico::hal::pwm::AnySlice;

use crate::display::Display;

pub mod parametric;

pub trait Drawable {
    fn draw<S: AnySlice>(&self, display: &mut Display<S>);
}
