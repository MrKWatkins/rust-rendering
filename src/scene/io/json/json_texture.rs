use crate::scene::io::json::{JsonMaterial, JsonScalar};
use crate::scene::Texture;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonTexture {
    Solid {
        #[serde(flatten)]
        material: JsonMaterial,
    },
    Chequerboard {
        material1: JsonMaterial,
        material2: JsonMaterial,
        size: JsonScalar,
    },
}

impl JsonTexture {
    pub fn to_texture(&self) -> Texture {
        return match &self {
            JsonTexture::Solid { material } => Texture::Solid {
                material: material.to_material(),
            },
            JsonTexture::Chequerboard { material1, material2, size } => Texture::Chequerboard {
                material1: material1.to_material(),
                material2: material2.to_material(),
                size: *size,
            },
        };
    }
}
