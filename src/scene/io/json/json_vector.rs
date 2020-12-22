use crate::maths::Vector;
use crate::scene::io::json::JsonScalar;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JsonVector(JsonScalar, JsonScalar, JsonScalar);

impl JsonVector {
    pub fn to_vector(&self) -> Vector {
        return Vector::new(self.0, self.1, self.2);
    }
}
