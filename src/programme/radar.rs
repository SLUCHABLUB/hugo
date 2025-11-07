use crate::{HEIGHT, Image, Pixel, WIDTH};
use rand::random_bool;

type Level = u8;

const LEVELS: Level = 10;
const MAX_LEVEL: Level = 60;

fn level_to_pixel(cycle_index: u8) -> impl Fn(Level) -> Pixel {
    move |level| {
        if level > cycle_index {
            Pixel::On
        } else {
            Pixel::Off
        }
    }
}

pub struct Radar {
    image: Image<Level>,
    cycle_index: u8,

    scan_position: usize,
    scan_velocity: isize,
}

impl Radar {
    pub(super) fn new(image: Image) -> Radar {
        Radar {
            image: image.map(|pixel| match pixel {
                Pixel::Off => 0,
                Pixel::On => Level::MAX,
            }),
            cycle_index: 0,
            scan_position: 0,
            scan_velocity: 1,
        }
    }

    fn current_frame(&self) -> Image {
        self.image.map(level_to_pixel(self.cycle_index))
    }

    fn decay(&mut self) {
        self.image = self.image.map(|level| level.saturating_sub(1));
    }

    pub(super) fn next_frame(&mut self) -> Image {
        self.cycle_index += 1;
        self.cycle_index %= LEVELS;

        self.decay();

        for y in 0..HEIGHT {
            self.image.set(self.scan_position, y, LEVELS);

            if random_bool(0.005) {
                self.image.set(self.scan_position, y, MAX_LEVEL);
            }
        }

        self.scan_position = self.scan_position.strict_add_signed(self.scan_velocity);

        if self.scan_position == 0 || self.scan_position == WIDTH - 1 {
            self.scan_velocity *= -1;
        }

        self.current_frame()
    }
}
