use hugo::{HUGO, Programme};
use rand::random;

fn main() {
    Programme::from_name("life", random())
        .unwrap()
        .run_continuously(&mut HUGO.lock().unwrap());
}
