use crate::image::Colour;
use crate::maths::*;
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
            //camera: Camera::default(),
            camera: Camera::new(Point::new(0.0, 0.0, -1.0), &Point::new(0.0, 0.0, 0.0), &Vector::y(), 30.0),
            objects: vec![],
            _private: (),
        };
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
}
