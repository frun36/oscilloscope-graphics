#![no_std]
#![no_main]

use core::f32::consts::PI;

use panic_halt as _;

use rp_pico::hal::{self, clocks::Clock, pac};

use defmt_rtt as _;
use defmt::info;

use oscilloscope_graphics::display::Display;
use oscilloscope_graphics::drawable::parametric_path::ParametricPath;

const TOP: u16 = 1024;

#[rp_pico::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // By default a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
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
    let mut display = Display::new(channel_x, channel_y, TOP, -4., 4., -3., 3., &mut delay);

    let mut u = 0.;

    info!("Hellou");

    loop {
        for i in 0..4 {
            let theta0 = 0.25 * PI * (2 * i) as f32 + u;
            let theta1 = 0.25 * PI * (2 * i + 1) as f32 + u;
            let (sin0, cos0) = libm::sincosf(theta0);
            let (sin1, cos1) = libm::sincosf(theta1);
            let seg0 = ParametricPath::segment((0., 0.), (2. * cos0, 2. * sin0), 0.1, 1000, 1, 0);
            let arc = ParametricPath::arc((0., 0.), 2., theta0, 0.1, theta1, 1000, 1, 0);
            let seg1 = ParametricPath::segment((2. * cos1, 2. * sin1), (0., 0.), -0.1, 1000, 1, 0);

            display.draw(&seg0);
            display.draw(&arc);
            display.draw(&seg1)
        }

        u += 0.05;
    }
}
