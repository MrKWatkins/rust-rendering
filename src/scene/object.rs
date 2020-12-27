use crate::material::Material;
use crate::maths::{Isometry, Plane, Point, Scalar, Sphere, Vector};
use nalgebra::Unit;
use ncollide3d::bounding_volume::AABB;
use ncollide3d::shape::{Shape, ShapeHandle};

pub struct Object {
    pub shape: ShapeHandle<Scalar>,

    pub position: Point,

    pub material: Material,

    pub transformation: Isometry,

    _private: (),
}

impl Object {
    pub fn new<T: Shape<Scalar>>(shape: T, position: Point, material: Material) -> Object {
        Object {
            shape: ShapeHandle::new(shape),
            position,
            material,
            transformation: Isometry::translation(position.x, position.y, position.z),
            _private: (),
        }
    }

    pub fn bounding_volume(&self) -> AABB<f32> {
        return self.shape.aabb(&self.transformation);
    }

    pub fn new_sphere(centre: Point, radius: Scalar, material: Material) -> Object {
        return Object::new(Sphere::new(radius), centre, material);
    }

    pub fn new_plane(centre: Point, normal: Vector, material: Material) -> Object {
        return Object::new(Plane::new(Unit::new_normalize(normal)), centre, material);
    }
}
