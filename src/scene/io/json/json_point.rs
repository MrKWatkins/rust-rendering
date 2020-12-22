use crate::maths::Point;
use crate::scene::io::json::JsonScalar;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonPoint(JsonScalar, JsonScalar, JsonScalar);

impl JsonPoint {
    pub fn to_point(&self) -> Point {
        return Point::new(self.0, self.1, self.2);
    }
}
