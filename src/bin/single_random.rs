use hugo::HUGO;
use rand::random;

fn main() {
    let mut hugo = HUGO.lock().unwrap();

    hugo.draw_image(random());
}