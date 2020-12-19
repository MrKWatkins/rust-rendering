use crate::image::Colour;

pub struct Material {
    pub colour: Colour,
    _private: (),
}

impl Material {
    pub fn matte(colour: Colour) -> Material {
        return Material { colour, _private: () };
    }
}
