#![no_std]
#![no_main]

use core::f32::consts::PI;

use panic_probe as _;

use defmt::info;
use defmt_rtt as _;

use rp_pico::hal::{self, adc::AdcPin, clocks::Clock, pac, Adc};

use oscilloscope_graphics::display::Display;
use oscilloscope_graphics::drawable::parametric_path::ParametricPath;

const TOP: u16 = 1024;

#[rp_pico::entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);

    // By default a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let sio = hal::Sio::new(peripherals.SIO);
    let pins = hal::gpio::Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init ADC
    let mut adc = Adc::new(peripherals.ADC, &mut peripherals.RESETS);
    let mut pot = AdcPin::new(pins.gpio26.into_floating_input()).unwrap();

    let adc_fifo = adc
        .build_fifo()
        .clock_divider(0, 0) // sample as fast as possible (500ksps)
        .set_channel(&mut pot)
        .start();

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.set_top(TOP);
    pwm.enable();

    let channel_x = &mut pwm.channel_a;
    channel_x.output_to(pins.gpio2);

    let channel_y = &mut pwm.channel_b;
    channel_y.output_to(pins.gpio3);

    // Init display
    let mut display = Display::new(
        channel_x, channel_y, TOP, -4., 4., -3., 3., &mut delay, adc_fifo,
    );

    let mut u = 0.;
    let mut r = 2.;

    info!("Hellou");

    loop {
        for i in 0..4 {
            let theta0 = 0.25 * PI * (2 * i) as f32 + u;
            let theta1 = 0.25 * PI * (2 * i + 1) as f32 + u;
            let (sin0, cos0) = libm::sincosf(theta0);
            let (sin1, cos1) = libm::sincosf(theta1);
            let seg0 = ParametricPath::segment((0., 0.), (r * cos0, r * sin0), 0.1, 1000, 1, 0);
            let arc = ParametricPath::arc((0., 0.), r, theta0, 0.1, theta1, 1000, 1, 0);
            let seg1 = ParametricPath::segment((r * cos1, r * sin1), (0., 0.), -0.1, 1000, 1, 0);

            display.draw(&seg0);
            display.draw(&arc);
            display.draw(&seg1);
        }

        u += 0.05;

        if let Some(x) = display.read_knob() {
            r = 1. + 2. * x;
        }
    }
}
