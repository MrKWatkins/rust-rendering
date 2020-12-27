use crate::scene::io::json::{JsonPoint, JsonScalar, JsonTexture, JsonVector};
use crate::scene::Object;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonObject {
    pub position: JsonPoint,

    pub texture: JsonTexture,

    #[serde(flatten)]
    pub shape: JsonShape,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JsonShape {
    Sphere { radius: JsonScalar },
    Plane { normal: JsonVector },
}

impl JsonObject {
    pub fn to_object(&self) -> Object {
        let position = self.position.to_point();
        let texture = self.texture.to_texture();

        return match &self.shape {
            JsonShape::Sphere { radius } => Object::new_sphere(position, *radius, texture),
            JsonShape::Plane { normal } => Object::new_plane(position, normal.to_vector(), texture),
        };
    }
}
