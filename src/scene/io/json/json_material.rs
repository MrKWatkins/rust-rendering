use crate::scene::io::json::{JsonColour, JsonScalar};
use crate::scene::Material;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonMaterial {
    pub ambient_colour: JsonColour,
    pub diffuse_colour: Option<JsonColour>,
    pub specular_colour: Option<JsonColour>,
    pub shininess: Option<JsonScalar>,
}

impl JsonMaterial {
    pub fn to_material(&self) -> Material {
        return Material {
            ambient_colour: self.ambient_colour.to_colour(),
            diffuse_colour: self.diffuse_colour.unwrap_or(self.ambient_colour).to_colour(),
            specular_colour: self.specular_colour.unwrap_or(self.ambient_colour).to_colour(),
            shininess: self.shininess.unwrap_or(0.0),
        };
    }
}
