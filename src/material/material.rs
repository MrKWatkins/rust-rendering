use crate::image::Colour;
use crate::material::Texture;

pub struct Material {
    pub texture: Texture,
    _private: (),
}

impl Material {
    pub fn new(texture: Texture) -> Material {
        return Material { texture, _private: () };
    }

    pub fn solid(colour: Colour) -> Material {
        return Material::new(Texture::Solid { colour });
    }
}
