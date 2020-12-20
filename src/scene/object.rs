use crate::material::Material;
use crate::maths::{Isometry, Point, Scalar, Sphere};
use ncollide3d::shape::Shape;

pub struct Object {
    pub shape: Box<dyn Shape<Scalar>>,

    pub position: Point,

    pub material: Material,

    pub transformation: Isometry,
}

impl Object {
    pub fn new<T: Shape<Scalar>>(shape: T, position: Point, material: Material) -> Object {
        Object {
            shape: Box::new(shape),
            position,
            material,
            transformation: Isometry::translation(position.x, position.y, position.z),
        }
    }

    pub fn sphere(centre: Point, radius: Scalar, material: Material) -> Object {
        return Object::new(Sphere::new(radius), centre, material);
    }
}
