use crate::image::Colour;
use crate::maths::{Coordinates, Ray, Scalar};
use crate::rendering::algorithms::Algorithm;
use crate::scene::Scene;
use nalgebra::Matrix;
use ncollide3d::query::RayCast;

pub struct RayTracing {
    _private: (),
}

impl RayTracing {
    pub fn new() -> RayTracing {
        return RayTracing { _private: () };
    }
}

impl Algorithm for RayTracing {
    fn render_point(&self, scene: &Scene, coordinates: &Coordinates) -> Colour {
        let eye = scene.camera.to_world_space(coordinates);

        let ray = Ray::new(scene.camera.position, Matrix::normalize(&(eye - scene.camera.position)));

        for object in &scene.objects {
            if object.shape.intersects_ray(&object.transformation, &ray, Scalar::MAX) {
                return object.material.colour.clone();
            }
        }

        return scene.background_colour.clone();
    }
}
