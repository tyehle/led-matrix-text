#![no_std]
#![no_main]

extern crate panic_halt;

use atsamd21g18a::{TC4, TC5};
use feather_m0 as hal;

use cortex_m_rt::entry;
use hal::clock::GenericClockController;
use hal::gpio::*;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::sercom::{SPIMaster4, Sercom4Pad0, Sercom4Pad2, Sercom4Pad3};
use hal::timer::TimerCounter;

use matrix_display::*;

mod best_font;

type SPI = SPIMaster4<Sercom4Pad0<Pa12<PfD>>, Sercom4Pad2<Pb10<PfD>>, Sercom4Pad3<Pb11<PfD>>>;
type LEDPin = Pa17<Output<OpenDrain>>;

/// Get the SPI bus setup
fn setup() -> (
    LEDPin,
    TimerCounter<TC5>,
    LEDArray<
        Pa7<Output<OpenDrain>>,
        Pa18<Output<OpenDrain>>,
        Pa16<Output<OpenDrain>>,
        TimerCounter<TC4>,
        SPI,
        Pa20<Output<OpenDrain>>,
        Pa15<Output<OpenDrain>>,
    >,
) {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    // initialize the pins
    let row_pins = (
        pins.d9.into_open_drain_output(&mut pins.port),
        pins.d10.into_open_drain_output(&mut pins.port),
        pins.d11.into_open_drain_output(&mut pins.port),
    );

    let reg_pin = pins.d6.into_open_drain_output(&mut pins.port);
    let output_disable = pins.d5.into_open_drain_output(&mut pins.port);

    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_low().unwrap();

    // Setup the timer
    let gclk0 = clocks.gclk0();
    let tc45 = &clocks.tc4_tc5(&gclk0).unwrap();
    let timer = hal::timer::TimerCounter::tc4_(tc45, peripherals.TC4, &mut peripherals.PM);

    let tc5 = hal::timer::TimerCounter::tc5_(tc45, peripherals.TC5, &mut peripherals.PM);

    // setup the SPI bus
    let spi = hal::spi_master(
        &mut clocks,
        10.mhz(),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.sck,
        pins.mosi,
        pins.miso,
        &mut pins.port,
    );

    let array = LEDArray {
        // array: image,
        array: [[0; 16]; 8],
        row_pins,
        timer,
        spi,
        reg_pin,
        output_disable,
    };

    (red_led, tc5, array)
}

fn scroll(frame_num: usize, image: &[&mut [u8]; 8], frame_buf: &mut [[u8; 16]; 8]) {
    for r in 0..frame_buf.len() {
        for c in 0..frame_buf[r].len() {
            let image_col = (c + frame_num) % image[r].len();
            frame_buf[r][c] = image[r][image_col];
        }
    }
}

#[entry]
fn main() -> ! {
    let (mut red_led, mut _timer, mut array) = setup();

    #[derive(Clone, Copy)]
    struct DelayHertz(u32);
    impl From<DelayHertz> for hal::time::Hertz {
        fn from(delay: DelayHertz) -> hal::time::Hertz {
            hal::time::Hertz(delay.0)
        }
    }
    impl core::ops::Shl<usize> for DelayHertz {
        type Output = DelayHertz;
        fn shl(self, amount: usize) -> DelayHertz {
            DelayHertz(self.0 << amount)
        }
    }

    array.timer.start(1.hz());

    let mut frame_num = 0_usize;
    let mut image = [
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
        &mut [0u8; 48][..],
    ];

    best_font::spell("Hello", &mut image).unwrap();

    loop {
        scroll(frame_num / 3, &image, &mut array.array);
        frame_num += 1;
        array.scan(DelayHertz(1000)).unwrap_or(());
        red_led.toggle();
    }
}
