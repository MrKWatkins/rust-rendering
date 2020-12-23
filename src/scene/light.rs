use crate::image::Colour;
use crate::maths::Point;

pub struct Light {
    pub position: Point,

    pub colour: Colour,
}

impl Light {
    pub fn new(position: Point, colour: Colour) -> Light {
        return Light { position, colour };
    }
}
