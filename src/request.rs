use hugo::Image;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Request {
    image: Option<Image>,
    duration_in_seconds: Number,
    programme: Option<String>,
}

impl Request {
    pub fn new(image: Image) -> Request {
        Request {
            image: Some(image),
            duration_in_seconds: Number::Integer(0),
            programme: None,
        }
    }

    pub fn image(&self) -> Option<Image> {
        self.image
    }

    pub fn duration(&self) -> Duration {
        match self.duration_in_seconds {
            Number::Integer(seconds) => {
                let seconds = u64::try_from(seconds).unwrap_or(0);
                Duration::from_secs(seconds)
            }
            Number::Float(seconds) => {
                Duration::try_from_secs_f64(seconds).unwrap_or(Duration::ZERO)
            }
        }
    }

    pub fn programme(&self) -> Option<&str> {
        self.programme.as_deref()
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Number {
    Integer(i64),
    Float(f64),
}

impl Default for Number {
    fn default() -> Self {
        Number::Integer(0)
    }
}
