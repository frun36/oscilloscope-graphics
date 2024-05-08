#![no_std]
#![no_main]

use core::f32::consts::PI;

use drawable::parametric_path::ParametricPath;
// Ensure we halt the program on panic
use panic_halt as _;

use hal::pac;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

use display::Display;

mod display;
mod drawable;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const TOP: u16 = 1024;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Should by default be a 125 MHz system clock? - measurements suggest it is equal to ~65 MHz
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.set_top(TOP);
    pwm.enable();

    let channel_x = &mut pwm.channel_a;
    channel_x.output_to(pins.gpio2);

    let channel_y = &mut pwm.channel_b;
    channel_y.output_to(pins.gpio3);

    // Init display
    let mut display = Display::new(channel_x, channel_y, -4., 4., -3., 3., &mut delay);

    

    let mut u = 0.;

    loop {
        let lissajous = ParametricPath::new(0., 0.04, 2. * PI, 0, |t| {
            (3.* libm::sinf(t + u), 3. * libm::sinf(2. * t))
        });

        display.draw(&lissajous);

        u += 0.05;
    }
}
