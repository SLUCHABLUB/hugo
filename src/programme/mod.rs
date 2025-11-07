use crate::{Hugo, Image};

mod life;
mod radar;
mod still;

pub use life::Life;
pub use radar::Radar;
use std::time::{Duration, Instant};
pub use still::Still;

#[non_exhaustive]
pub enum Programme {
    Life(Life),
    Radar(Radar),
    Still(Still),
}

impl Programme {
    pub fn from_name(name: &str, start: Image) -> Option<Programme> {
        match name {
            "life" => Some(Programme::Life(Life::new(start))),
            "radar" => Some(Programme::Radar(Radar::new(start))),
            "still" => Some(Programme::Still(Still::new(start))),
            _ => None,
        }
    }

    fn default_frames_per_second(&self) -> f64 {
        match self {
            Programme::Life(_) | Programme::Radar(_) => 1.0,
            Programme::Still(_) => 0.0,
        }
    }

    fn default_frame_duration(&self) -> Duration {
        let seconds = self.default_frames_per_second().recip();
        Duration::from_secs_f64(seconds)
    }

    pub fn frame_rate_to_duration(&self, frame_rate: Option<f64>) -> Duration {
        frame_rate
            .map(f64::recip)
            .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok())
            .unwrap_or_else(|| self.default_frame_duration())
    }

    pub fn next_frame(&mut self) -> Image {
        match self {
            Programme::Life(life) => life.next_frame(),
            Programme::Radar(radar) => radar.next_frame(),
            Programme::Still(still) => still.image(),
        }
    }

    // TODO: make the return type `!`
    pub fn run_continuously(&mut self, hugo: &mut Hugo) {
        let frame_duration = self.default_frame_duration();

        let mut next_frame = Instant::now();
        let mut image = Image::default();

        loop {
            hugo.draw_image(image);

            if Instant::now() >= next_frame {
                next_frame += frame_duration;

                image = self.next_frame();
            }
        }
    }
}
