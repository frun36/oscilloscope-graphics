#![no_std]

pub const TOP: u16 = 255;

pub const X_MIN: i8 = -4;
pub const X_MAX: i8 = 4;
pub const Y_MIN: i8 = -3;
pub const Y_MAX: i8 = 3;


type Coordinate = fixed::FixedI8<fixed::types::extra::U5>;

pub mod display;
pub mod drawable;