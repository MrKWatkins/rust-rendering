use crate::scene::io::json::{JsonColour, JsonScalar};
use crate::scene::Material;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonMaterial {
    pub ambient_colour: JsonColour,
    pub diffuse_colour: Option<JsonColour>,
    pub specular_colour: Option<JsonColour>,
    #[serde(default = "default_shininess")]
    pub shininess: JsonScalar,
    #[serde(default = "default_reflectivity")]
    pub reflectivity: JsonScalar,
}

fn default_shininess() -> JsonScalar {
    return 0.0;
}

fn default_reflectivity() -> JsonScalar {
    return 0.0;
}

impl JsonMaterial {
    pub fn to_material(&self) -> Material {
        return Material {
            ambient_colour: self.ambient_colour.to_colour(),
            diffuse_colour: self.diffuse_colour.unwrap_or(self.ambient_colour).to_colour(),
            specular_colour: self.specular_colour.unwrap_or(self.ambient_colour).to_colour(),
            shininess: self.shininess,
            reflectivity: self.reflectivity,
        };
    }
}
