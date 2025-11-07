use crate::{Image, Pixel};
use rand::random;
use std::hash::{DefaultHasher, Hash, Hasher};

const PAUSE_FRAMES: usize = 1000;
const MAXIMUM_LOOP_LENGTH: usize = 280;

const DEAD: Pixel = Pixel::Off;
const ALIVE: Pixel = Pixel::On;

pub struct Life {
    image: Image,
    hashes: [u64; MAXIMUM_LOOP_LENGTH],
    hash_head: usize,
    simulation: Image,

    // The number of frames until we should restart the simulation.
    restart_after: Option<usize>,
}

impl Life {
    pub(super) fn new(image: Image) -> Life {
        Life {
            image,
            hashes: [0; MAXIMUM_LOOP_LENGTH],
            hash_head: 0,
            restart_after: None,
            simulation: image,
        }
    }

    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        #[rustfmt::skip]
        const OFFSETS: [[isize; 2]; 8] = [
            [-1, -1],
            [-1,  0],
            [-1,  1],
            [ 0, -1],
            [ 0,  1],
            [ 1, -1],
            [ 1,  0],
            [ 1,  1],
        ];

        let mut count = 0;

        for [x_offset, y_offset] in OFFSETS {
            let Some(neighbour_x) = x.checked_add_signed(x_offset) else {
                continue;
            };
            let Some(neighbour_y) = y.checked_add_signed(y_offset) else {
                continue;
            };

            if self.simulation.get(neighbour_x, neighbour_y) == Some(ALIVE) {
                count += 1;
            }
        }

        count
    }

    fn evolve(&mut self) {
        let current_generation = self.image;
        let mut next_generation = Image::default();

        for ([x, y], cell) in current_generation.pixel_coordinates() {
            let neighbour_count = self.neighbour_count(x, y);

            let new_cell = match cell {
                DEAD if neighbour_count == 3 => ALIVE,
                DEAD => DEAD,
                ALIVE => match neighbour_count {
                    0..2 | 4.. => DEAD,
                    2 | 3 => ALIVE,
                },
            };

            next_generation.set(x, y, new_cell);
        }

        self.image = next_generation;
    }

    fn is_looping(&mut self) -> bool {
        let mut state = DefaultHasher::new();

        self.simulation.hash(&mut state);

        let hash = state.finish();

        if self.hashes.contains(&hash) {
            return true;
        }

        self.hashes[self.hash_head] = hash;

        self.hash_head += 1;
        self.hash_head %= MAXIMUM_LOOP_LENGTH;

        false
    }

    pub(super) fn next_frame(&mut self) -> Image {
        if let Some(restart_after) = self.restart_after.as_mut() {
            *restart_after -= 1;
        }

        if self.restart_after == Some(0) {
            *self = Life::new(random());

            return self.simulation;
        }

        self.evolve();

        if self.restart_after.is_none() && self.is_looping() {
            self.restart_after = Some(PAUSE_FRAMES);
        }

        self.image
    }
}
