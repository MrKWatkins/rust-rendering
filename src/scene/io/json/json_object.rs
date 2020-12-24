use crate::scene::io::json::{JsonMaterial, JsonPoint, JsonScalar, JsonVector};
use crate::scene::Object;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonObject {
    pub position: JsonPoint,

    pub material: JsonMaterial,

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
        let material = self.material.to_material();

        return match &self.shape {
            JsonShape::Sphere { radius } => Object::new_sphere(position, *radius, material),
            JsonShape::Plane { normal } => Object::new_plane(position, normal.to_vector(), material),
        };
    }
}
