use crate::image::Colour;
use crate::maths::{Isometry, Point};

pub struct Light {
    pub position: Point,

    pub colour: Colour,

    pub transformation: Isometry,
}

impl Light {
    pub fn new(position: Point, colour: Colour) -> Light {
        return Light {
            position,
            colour,
            transformation: Isometry::translation(position.x, position.y, position.z),
        };
    }
}
