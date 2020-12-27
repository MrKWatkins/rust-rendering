use crate::image::Colour;
use crate::maths::{Isometry, Point, Ray};
use nalgebra::Matrix;

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

    pub fn ray_to(&self, point: &Point) -> Ray {
        return Ray::new(self.position, Matrix::normalize(&(point - self.position)));
    }
}
