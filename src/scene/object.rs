use crate::maths::{Isometry, Plane, Point, Scalar, Sphere, Vector};
use crate::scene::Texture;
use nalgebra::Unit;
use ncollide3d::shape::{Shape, ShapeHandle};
use std::ptr;

pub struct Object {
    pub shape: ShapeHandle<Scalar>,

    pub position: Point,

    pub texture: Texture,

    pub transformation: Isometry,

    _private: (),
}

impl Object {
    pub fn new<T: Shape<Scalar>>(shape: T, position: Point, texture: Texture) -> Object {
        Object {
            shape: ShapeHandle::new(shape),
            position,
            texture,
            transformation: Isometry::translation(position.x, position.y, position.z),
            _private: (),
        }
    }

    pub fn new_sphere(centre: Point, radius: Scalar, texture: Texture) -> Object {
        return Object::new(Sphere::new(radius), centre, texture);
    }

    pub fn new_plane(centre: Point, normal: Vector, texture: Texture) -> Object {
        return Object::new(Plane::new(Unit::new_normalize(normal)), centre, texture);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        return ptr::eq(self, other);
    }
}

impl Eq for Object {}
