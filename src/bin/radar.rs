use hugo::{Image, Programme, HUGO};

fn main() {
    Programme::from_name("radar", Image::default())
        .unwrap()
        .run_continuously(&mut HUGO.lock().unwrap());
}
