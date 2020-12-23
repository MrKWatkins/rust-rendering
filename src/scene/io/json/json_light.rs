use crate::scene::io::json::{JsonColour, JsonPoint};
use crate::scene::Light;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonLight {
    pub position: JsonPoint,

    pub colour: JsonColour,
}

impl JsonLight {
    pub fn to_light(&self) -> Light {
        return Light::new(self.position.to_point(), self.colour.to_colour());
    }
}
