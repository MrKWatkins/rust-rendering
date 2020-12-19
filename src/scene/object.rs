use crate::material::Material;
use crate::maths::{Point, Scalar, Sphere};
use nalgebra::Isometry3;
use ncollide3d::shape::Shape;

pub struct Object {
    pub shape: Box<dyn Shape<Scalar>>,

    pub position: Isometry3<Scalar>,

    pub material: Material,
}

impl Object {
    pub fn new<T: Shape<Scalar>>(shape: T, position: &Point, material: Material) -> Object {
        Object {
            shape: Box::new(shape),
            position: Isometry3::translation(position.x, position.y, position.z),
            material,
        }
    }

    pub fn sphere(centre: &Point, radius: Scalar, material: Material) -> Object {
        return Object::new(Sphere::new(radius), centre, material);
    }
}
