#![feature(array_try_map)]
#![doc = include_str!("../README.md")]

mod image;
mod pixel;

pub use image::Image;
pub use pixel::Pixel;

use rppal::gpio::{Gpio, Level, OutputPin};
use std::error::Error;
use std::hint::black_box;
use std::process::exit;
use std::sync::{LazyLock, Mutex};

const SHIFT_PIN_NUMBER: u8 = 16;
const CLEAR_PIN_NUMBER: u8 = 20;
const COLUMN_PIN_NUMBER: u8 = 21;

const ROW_PIN_NUMBERS: [u8; HEIGHT] = [18, 23, 24, 25, 8, 7, 12];

const BUSY_WAIT_CYCLES: usize = 20000;
const CLOCK_WAIT_CYCLES: usize = 100;

pub const WIDTH: usize = 95;
pub const HEIGHT: usize = 7;

/// The [`Hugo`] singleton.
pub static HUGO: LazyLock<Mutex<Hugo>> = LazyLock::new(|| {
    Mutex::new(match Hugo::new() {
        Ok(hugo) => hugo,
        Err(error) => {
            eprint!("[ERROR]: failed to initialise Hugo: {error}");
            exit(1)
        }
    })
});

/// A type that holds the state of Hugo.
///
/// There *should* only ever be one instance ant it can be found in [`HUGO`].
pub struct Hugo {
    shift: OutputPin,
    clear: OutputPin,
    column: OutputPin,
    rows: [OutputPin; HEIGHT],
}

impl Hugo {
    /// Initializes Hugo. Should only be ran once.
    fn new() -> Result<Hugo, Box<dyn Error>> {
        let gpio = Gpio::new()?;

        let mut shift = gpio.get(SHIFT_PIN_NUMBER)?.into_output();
        let mut clear = gpio.get(CLEAR_PIN_NUMBER)?.into_output();
        let mut column = gpio.get(COLUMN_PIN_NUMBER)?.into_output();

        shift.write(Level::Low);
        column.write(Level::Low);

        clear.write(Level::High);

        let rows = ROW_PIN_NUMBERS.try_map(|pin_number| -> Result<_, Box<dyn Error>> {
            let mut pin = gpio.get(pin_number)?.into_output();
            pin.write(Level::Low);
            Ok(pin)
        })?;

        Ok(Hugo {
            shift,
            clear,
            column,
            rows,
        })
    }

    fn clock_wait(&mut self) {
        for iteration in 0..CLOCK_WAIT_CYCLES {
            black_box(iteration);
        }
    }

    fn busy_wait(&mut self) {
        for iteration in 0..BUSY_WAIT_CYCLES {
            black_box(iteration);
        }
    }

    pub fn draw_image(&mut self, image: Image) {
        for (y, row) in image.rows().iter().enumerate() {
            for pixel in *row {
                self.column.write(pixel.level());

                self.clock_wait();

                self.shift.set_high();
                self.clock_wait();
                self.shift.set_low();
            }

            self.rows[y].set_high();
            self.busy_wait();
            self.rows[y].set_low();

            self.clear.set_low();
            self.clock_wait();
            self.clear.set_high();
        }
    }
}
