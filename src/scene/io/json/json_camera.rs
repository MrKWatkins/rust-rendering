use crate::scene::io::json::{JsonPoint, JsonScalar, JsonVector};
use crate::scene::Camera;
use serde::Deserialize;

// TODO: Options so you can just specify part of the JSON and use defaults for the rest.
#[derive(Deserialize)]
pub struct JsonCamera {
    pub position: JsonPoint,
    pub looking_at: JsonPoint,
    pub up: JsonVector,
    pub field_of_view_degrees: JsonScalar,
}

impl JsonCamera {
    pub fn to_camera(&self) -> Camera {
        //pub fn new(position: Point, looking_at: &Point, up: &Vector, field_of_view_degrees: Scalar)
        return Camera::new(
            self.position.to_point(),
            &self.looking_at.to_point(),
            &self.up.to_vector(),
            self.field_of_view_degrees,
        );
    }
}
