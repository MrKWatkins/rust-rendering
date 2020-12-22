use crate::maths::Sphere;
use crate::scene::io::json::{JsonMaterial, JsonPoint, JsonScalar};
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
}

impl JsonObject {
    pub fn to_object(&self) -> Object {
        let shape = match self.shape {
            JsonShape::Sphere { radius } => Sphere::new(radius),
        };

        return Object::new(shape, self.position.to_point(), self.material.to_material());
    }
}
