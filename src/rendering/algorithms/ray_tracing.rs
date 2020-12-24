use crate::image::Colour;
use crate::maths::{Coordinates, Ray, Scalar};
use crate::rendering::algorithms::Algorithm;
use crate::scene::{Object, Scene};
use nalgebra::Matrix;
use ncollide3d::query::RayCast;
use std::ptr;

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

    'lights: for light in &scene.lights {
        // Trace a ray from the light to the point_of_intersection.
        let light_ray = Ray::new(light.position, Matrix::normalize(&(point_of_intersection - &light.position)));
        let light_distance = object.shape.toi_with_ray(&object.transformation, &light_ray, Scalar::MAX, false);
        if light_distance.is_none() {
            // This could happen if the intersection is right on the edge for floating point reasons.
            continue;
        }

        // See if there is another object closer; if so it will be blocking the light ray.
        for other_object in &scene.objects {
            if ptr::eq(object, other_object) {
                continue;
            }
            if let Some(object_distance) = other_object.shape.toi_with_ray(&other_object.transformation, &light_ray, Scalar::MAX, false) {
                if object_distance < light_distance.unwrap() {
                    continue 'lights;
                }
            }
        }

        // Nothing blocking, add the colour contribution for the light.
        let lambertian = -light_ray.dir.dot(&intersection.normal) * &object.material.colour * &light.colour;

        colour = colour + lambertian.clamp();
    }

    return colour;
}
