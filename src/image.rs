use crate::{HEIGHT, Pixel, WIDTH};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use serde::Deserialize;
use std::borrow::Cow;
use std::array::from_fn;

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
        self.rows.into_iter().enumerate().flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, pixel)| ([x, y], pixel))
        })
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

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Representation<'data> {
            #[serde(with = "serde_arrays")]
            Pixels([Pixel; WIDTH * HEIGHT]),
            #[serde(with = "serde_arrays")]
            Rows([Row; HEIGHT]),
            Text(Cow<'data, str>),
        }

        #[derive(Deserialize)]
        #[serde(transparent)]
        struct Row {
            #[serde(with = "serde_arrays")]
            pixels: [Pixel; WIDTH],
        }

        Representation::deserialize(deserializer).and_then(|representation| Ok(match representation {
            Representation::Pixels(pixels) => Image {
                rows: from_fn(|y| from_fn(|x| pixels[y * WIDTH + x]))
            },
            Representation::Rows(rows) => Image { rows: rows.map(|row| row.pixels) },
            Representation::Text(text) => todo!("render the text: {text:?}"),
        }))
    }
}