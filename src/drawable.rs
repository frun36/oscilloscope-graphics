use rp2040_hal::pwm::AnySlice;

use crate::display::Display;

pub mod parametric_path;

pub trait Drawable {
    fn draw<S: AnySlice>(&self, display: &mut Display<S>);
}
