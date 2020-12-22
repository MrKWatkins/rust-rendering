use crate::image::Colour;
use crate::scene::io::json::JsonScalar;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonColour(JsonScalar, JsonScalar, JsonScalar);

impl JsonColour {
    pub fn to_colour(&self) -> Colour {
        return Colour::new(self.0, self.1, self.2);
    }
}
