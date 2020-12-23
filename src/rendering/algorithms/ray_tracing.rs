use crate::image::Colour;
use crate::maths::{Coordinates, Ray, Scalar};
use crate::rendering::algorithms::Algorithm;
use crate::scene::{Object, Scene};
use nalgebra::Matrix;
use ncollide3d::query::RayCast;

type RayIntersection = ncollide3d::query::RayIntersection<Scalar>;

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
        return render_point(scene, coordinates);
    }
}

fn render_point(scene: &Scene, coordinates: &Coordinates) -> Colour {
    let eye = scene.camera.to_world_space(coordinates);

    let ray = Ray::new(scene.camera.position, Matrix::normalize(&(eye - scene.camera.position)));

    for object in &scene.objects {
        if let Some(intersection) = object.shape.toi_and_normal_with_ray(&object.transformation, &ray, Scalar::MAX, true) {
            return calculate_colour(scene, object, &ray, &intersection);
        }
    }

    return scene.background_colour.clone();
}

fn calculate_colour(scene: &Scene, object: &Object, ray: &Ray, intersection: &RayIntersection) -> Colour {
    let mut colour = &object.material.colour * &scene.ambient_light;

    let point_of_intersection = ray.origin + ray.dir * intersection.toi;
    for light in &scene.lights {
        let light_ray = Ray::new(light.position, Matrix::normalize(&(point_of_intersection - &light.position)));

        let lambertian = -light_ray.dir.dot(&intersection.normal) * &object.material.colour * &light.colour;

        colour = colour + lambertian.clamp();
    }

    return colour;
}
