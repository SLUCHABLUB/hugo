use crate::{HEIGHT, Pixel, WIDTH};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

/// An image that can be displayed on Hugo's screen.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Image<P = Pixel> {
    /// An array of the rows of lights.
    /// Each row is an array of pixels.
    rows: [[P; WIDTH]; HEIGHT],
}

impl<P> Image<P> {
    pub fn set(&mut self, x: usize, y: usize, pixel: P) {
        self.rows[y][x] = pixel;
    }

    pub fn map<F, P1>(self, mut function: F) -> Image<P1>
    where
        F: FnMut(P) -> P1,
    {
        Image {
            rows: self.rows.map(|row| row.map(&mut function)),
        }
    }
}

impl<P> Image<P>
where
    P: Copy,
{
    pub(crate) fn rows(self) -> [[P; WIDTH]; HEIGHT] {
        self.rows
    }

    pub fn get(self, x: usize, y: usize) -> Option<P> {
        self.rows.get(y).copied()?.get(x).copied()
    }

    pub fn pixel_coordinates(self) -> impl Iterator<Item = ([usize; 2], P)> {
        self.rows
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(x, pixel)| ([x, y], pixel))
            })
            .flatten()
    }
}

// We cannot derive this since arrays only implement `Default` for sizes up to 32.
impl<P> Default for Image<P>
where
    P: Copy + Default,
{
    fn default() -> Self {
        Self {
            rows: [[P::default(); WIDTH]; HEIGHT],
        }
    }
}

impl Distribution<Image> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Image {
        use std::array::from_fn;

        Image {
            rows: from_fn(|_| from_fn(|_| self.sample(rng))),
        }
    }
}
