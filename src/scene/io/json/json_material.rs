use crate::material::Material;
use crate::scene::io::json::JsonColour;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonMaterial {
    pub colour: JsonColour,
}

impl JsonMaterial {
    pub fn to_material(&self) -> Material {
        return Material::matte(self.colour.to_colour());
    }
}
