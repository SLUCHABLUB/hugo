use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use rppal::gpio::Level;
use serde::Deserialize;
use serde::de::Error;
use serde::de::Unexpected;
use std::borrow::Cow;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Pixel {
    #[default]
    Off,
    On,
}

impl Pixel {
    pub fn on(on: bool) -> Pixel {
        if on { Pixel::On } else { Pixel::Off }
    }

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

impl<'de> Deserialize<'de> for Pixel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let expected = r#""on", "off", "red", "black", 1, 0, true or false"#;

        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Representation<'data> {
            Bool(bool),
            Integer(i64),
            Named(Cow<'data, str>),
        }

        Representation::deserialize(deserializer).and_then(|representation| {
            Ok(match representation {
                Representation::Bool(on) => Pixel::on(on),
                Representation::Integer(integer) => Pixel::on(integer != 0),
                Representation::Named(cow) => match cow.to_lowercase().as_str() {
                    "on" | "red" => Pixel::On,
                    "off" | "black" => Pixel::Off,
                    other => {
                        return Err(<D::Error as Error>::invalid_value(
                            Unexpected::Other(other),
                            &expected,
                        ));
                    }
                },
            })
        })
    }
}
