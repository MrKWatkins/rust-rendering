pub type Scalar = f32;

pub type Perspective = nalgebra::Perspective3<Scalar>;

pub type Point = nalgebra::Point3<Scalar>;

pub type Ray = ncollide3d::query::Ray<Scalar>;

pub type Vector = nalgebra::Vector3<Scalar>;

pub type Isometry = nalgebra::Isometry3<Scalar>;

pub type Sphere = ncollide3d::shape::Ball<Scalar>;

pub type TransformationMatrix = nalgebra::Matrix4<Scalar>;
