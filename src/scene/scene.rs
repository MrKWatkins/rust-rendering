use crate::image::Colour;
use crate::scene::{Camera, Light, Object};

pub struct Scene {
    pub ambient_light: Colour,
    pub background_colour: Colour,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
    _private: (),
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            ambient_light: Colour::black(),
            background_colour: Colour::black(),
            camera: Camera::default(),
            lights: vec![],
            objects: vec![],
            _private: (),
        };
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
