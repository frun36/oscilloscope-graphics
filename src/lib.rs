#![no_std]

use fixed::types::{I24F8, I3F5};

pub const TOP: u16 = 255;

pub const X_MIN: i8 = -4;
pub const X_MAX: i8 = 4;
pub const Y_MIN: i8 = -3;
pub const Y_MAX: i8 = 3;


pub type Coordinate = I3F5;
pub type Parameter = I24F8;

pub mod display;
pub mod drawable;