use crate::image::Colour;
use crate::maths::Scalar;

pub struct Material {
    pub ambient_colour: Colour,
    pub diffuse_colour: Colour,
    pub specular_colour: Colour,
    pub shininess: Scalar,
}

impl Material {
    pub fn matte(colour: Colour) -> Material {
        return Material {
            ambient_colour: colour,
            diffuse_colour: colour,
            specular_colour: colour,
            shininess: 0.0,
        };
    }
}
