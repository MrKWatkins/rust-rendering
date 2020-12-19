use crate::image::Colour;
use crate::scene::Object;

pub struct Scene {
    pub background_colour: Colour,
    pub objects: Vec<Object>,
    _private: (),
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            background_colour: Colour::black(),
            objects: vec![],
            _private: (),
        };
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
}
