use crate::image::Colour;
use crate::material::Material;
use ncollide3d::shape::Shape;

pub struct Object {
    pub shape: Box<dyn Shape<f32>>,

    pub material: Material,
}

impl Object {
    pub fn new<T: Shape<f32>>(shape: T) -> Object {
        Self {
            shape: Box::new(shape),
            material: Material {
                colour: Colour::new(0.0, 0.0, 0.0),
            },
        }
    }
}
