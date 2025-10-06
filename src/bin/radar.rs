use hugo::{HEIGHT, HUGO, Image, Pixel, WIDTH};
use rand::random_bool;

type Level = u8;

const FRAMES_PER_TICK: usize = 20;

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

fn main() {
    let mut hugo = HUGO.lock().unwrap();

    let mut image = Image::<Level>::default();

    let mut frame = 0;
    let mut cycle_index = 0;

    let mut scan_position: usize = 0;
    let mut scan_velocity: isize = 1;

    loop {
        hugo.draw_image(image.map(level_to_pixel(cycle_index)));

        cycle_index += 1;
        cycle_index %= LEVELS;

        if frame != 0 {
            frame -= 1;
            continue;
        }

        frame = FRAMES_PER_TICK;

        image = image.map(|level| level.saturating_sub(1));

        for y in 0..HEIGHT {
            image.set(scan_position, y, LEVELS);

            if random_bool(0.005) {
                image.set(scan_position, y, MAX_LEVEL);
            }
        }

        scan_position = scan_position.strict_add_signed(scan_velocity);

        if scan_position == 0 || scan_position == WIDTH - 1 {
            scan_velocity = -scan_velocity;
        }
    }
}
