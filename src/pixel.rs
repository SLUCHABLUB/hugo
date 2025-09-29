use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rppal::gpio::Level;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Pixel {
    #[default]
    Off,
    On,
}

impl Pixel {
    pub(crate) fn level(self) -> Level {
        match self {
            Pixel::Off => Level::Low,
            Pixel::On => Level::High,
        }
    }
}

impl Distribution<Pixel> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Pixel {
        if self.sample(rng) {
            Pixel::On
        } else {
            Pixel::Off
        }
    }
}
