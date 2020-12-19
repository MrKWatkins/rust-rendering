use crate::image::Colour;
use crate::maths::{Point, Ray, Scalar, Vector};
use crate::rendering::algorithms::Algorithm;
use crate::scene::Scene;
use nalgebra::Matrix;
use ncollide3d::query::RayCast;

pub struct RayTracing {
    positive_z: Vector,
    _private: (),
}

impl RayTracing {
    pub fn new() -> RayTracing {
        return RayTracing {
            positive_z: Matrix::normalize(&Vector::new(0.0, 0.0, 1.0)),
            _private: (),
        };
    }
}

impl Algorithm for RayTracing {
    fn render_point(&self, x: Scalar, y: Scalar, scene: &Scene) -> Colour {
        // Create a ray from the (x,y) point at 0 on the z-axis directed into positive z.
        let ray = Ray::new(Point::new(x, y, 0.0), self.positive_z);

        for object in &scene.objects {
            if object.shape.intersects_ray(&object.position, &ray, Scalar::MAX) {
                return object.material.colour.clone();
            }
        }

        return scene.background_colour.clone();
    }
}
