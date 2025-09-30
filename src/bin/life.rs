use hugo::{HUGO, Image, Pixel};
use rand::random;
use std::hash::{DefaultHasher, Hash, Hasher};

const FRAMES_PER_GENERATION: usize = 60;
const PAUSE_FRAMES: usize = 1000;
const MAXIMUM_LOOP_LENGTH: usize = 280;

const DEAD: Pixel = Pixel::Off;
const ALIVE: Pixel = Pixel::On;

fn neighbour_count(simulation: Image, x: usize, y: usize) -> usize {
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

        if simulation.get(neighbour_x, neighbour_y) == Some(ALIVE) {
            count += 1;
        }
    }

    count
}

fn evolve(current_generation: Image) -> Image {
    let mut next_generation = Image::default();

    for ([x, y], cell) in current_generation.pixel_coordinates() {
        let neighbour_count = neighbour_count(current_generation, x, y);

        let new_cell = match cell {
            DEAD if neighbour_count == 3 => ALIVE,
            DEAD => DEAD,
            ALIVE => match neighbour_count {
                0..2 | 4.. => DEAD,
                2 | 3 => ALIVE,
            },
        };

        next_generation.set(x, y, new_cell)
    }

    next_generation
}

fn is_looping(
    simulation: Image,
    hash_head: &mut usize,
    hashes: &mut [u64; MAXIMUM_LOOP_LENGTH],
) -> bool {
    let mut hasher = DefaultHasher::new();

    simulation.hash(&mut hasher);

    let hash = hasher.finish();

    if hashes.contains(&hash) {
        return true;
    }

    hashes[*hash_head] = hash;

    *hash_head += 1;
    *hash_head %= MAXIMUM_LOOP_LENGTH;

    false
}

fn main() {
    let mut hugo = HUGO.lock().unwrap();

    let mut simulation = random();

    let mut hashes = [0; MAXIMUM_LOOP_LENGTH];
    let mut hash_head = 0;

    let mut evolve_after = FRAMES_PER_GENERATION;

    // The number of iterations until we should restart the simulation.
    let mut restart_after = None;

    loop {
        // TODO: do we need to redraw 60 times for each generation?
        hugo.draw_image(simulation);

        if let Some(restart_after) = restart_after.as_mut() {
            *restart_after -= 1;
        }

        if restart_after == Some(0) {
            simulation = random();
            hashes.fill(0);

            restart_after = None;
        }

        evolve_after -= 1;

        if evolve_after == 0 {
            simulation = evolve(simulation);

            if restart_after.is_none() && is_looping(simulation, &mut hash_head, &mut hashes) {
                restart_after = Some(PAUSE_FRAMES);
            }

            evolve_after = FRAMES_PER_GENERATION;
        }
    }
}
