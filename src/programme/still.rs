use crate::Image;

pub struct Still {
    image: Image,
}

impl Still {
    pub(super) fn new(image: Image) -> Still {
        Still { image }
    }

    pub(super) fn image(&self) -> Image {
        self.image
    }
}
