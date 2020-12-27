use crate::material::{Material, Texture};
use crate::scene::io::json::{JsonColour, JsonScalar};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonMaterial {
    #[serde(flatten)]
    pub texture: JsonTexture,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonTexture {
    Solid {
        colour: JsonColour,
    },
    Chequerboard {
        colour1: JsonColour,
        colour2: JsonColour,
        size: JsonScalar,
    },
}

impl JsonMaterial {
    pub fn to_material(&self) -> Material {
        return Material::new(self.texture.to_texture());
    }
}

impl JsonTexture {
    pub fn to_texture(&self) -> Texture {
        return match &self {
            JsonTexture::Solid { colour } => Texture::Solid { colour: colour.to_colour() },
            JsonTexture::Chequerboard { colour1, colour2, size } => Texture::Chequerboard {
                colour1: colour1.to_colour(),
                colour2: colour2.to_colour(),
                size: *size,
            },
        };
    }
}
