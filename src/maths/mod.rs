pub mod consts;
pub mod ray;
pub mod sphere;
pub mod vector;

pub type Coordinates = nalgebra::Point2<Scalar>;
pub type Isometry = nalgebra::Isometry3<Scalar>;
pub type Perspective = nalgebra::Perspective3<Scalar>;
pub type Point = nalgebra::Point3<Scalar>;
pub type Plane = ncollide3d::shape::Plane<Scalar>;
pub type Ray = ncollide3d::query::Ray<Scalar>;
pub type RayIntersection = ncollide3d::query::RayIntersection<Scalar>;
pub type Scalar = f32;
pub type Sphere = ncollide3d::shape::Ball<Scalar>;
pub type TransformationMatrix = nalgebra::Matrix4<Scalar>;
pub type Vector = nalgebra::Vector3<Scalar>;
pub type Unit<T> = nalgebra::Unit<T>;
