use bevy::prelude::*;

pub trait SetImage {
    /// Set the image if not already set
    fn set_image(&mut self, image: &Handle<Image>);
}

impl SetImage for Sprite {
    fn set_image(&mut self, image: &Handle<Image>) {
        if self.image != *image {
            self.image = image.clone();
        }
    }
}
