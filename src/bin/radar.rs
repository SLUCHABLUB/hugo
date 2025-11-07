use hugo::{HUGO, Image, Programme};

fn main() {
    Programme::from_name("radar", Image::default())
        .unwrap()
        .run_continuously(&mut HUGO.lock().unwrap());
}
