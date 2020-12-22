use crate::image::Colour;
use crate::scene::{Camera, Object};

pub struct Scene {
    pub background_colour: Colour,
    pub camera: Camera,
    pub objects: Vec<Object>,
    _private: (),
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            background_colour: Colour::black(),
            camera: Camera::default(),
            objects: vec![],
            _private: (),
        };
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
