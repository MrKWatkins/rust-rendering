use crate::image::Colour;
use crate::maths::{Coordinates, Ray, RayIntersection};
use crate::rendering::algorithms::Algorithm;
use crate::scene::{Object, Scene};
use std::ptr;

pub struct RayTracing {
    _private: (),
}

impl RayTracing {
    pub fn new() -> RayTracing {
        return RayTracing { _private: () };
    }
}

impl Algorithm for RayTracing {
    fn render_point(&self, scene: &Scene, camera_space_coordinates: &Coordinates) -> Colour {
        return render_point(scene, camera_space_coordinates);
    }
}

fn render_point(scene: &Scene, camera_space_coordinates: &Coordinates) -> Colour {
    let ray = scene.camera.ray_to(camera_space_coordinates);

    if let Some(collision) = scene.first_collision_with_ray(&ray) {
        return calculate_colour(scene, collision.object, &ray, &collision.intersection);
    }

    return scene.background_colour.clone();
}

fn calculate_colour(scene: &Scene, object: &Object, ray: &Ray, intersection: &RayIntersection) -> Colour {
    let mut colour = &object.material.colour * &scene.ambient_light;

    let point_of_intersection = ray.origin + ray.dir * intersection.toi;

    for light in &scene.lights {
        // Get a ray from the light to the point_of_intersection.
        let light_ray = light.ray_to(&point_of_intersection);

        // See if there is another object closer; if so it will be blocking the light ray.
        if let Some(other) = scene.first_collision_with_ray(&light_ray) {
            if !ptr::eq(object, other.object) {
                continue;
            }
        }

        // Nothing blocking, add the colour contribution for the light.
        let lambertian = -light_ray.dir.dot(&intersection.normal) * &object.material.colour * &light.colour;

        colour = colour + lambertian.clamp();
    }

    return colour;
}
