use crate::maths::{Point, Ray};
use nalgebra::Matrix;

pub fn between(from: &Point, to: &Point) -> Ray {
    return Ray::new(*from, Matrix::normalize(&(to - from)));
}
