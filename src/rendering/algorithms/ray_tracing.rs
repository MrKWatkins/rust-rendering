use crate::image::Colour;
use crate::maths::Coordinates;
use crate::rendering::algorithms::Algorithm;
use crate::scene::{RayCollision, Scene};
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
        return calculate_colour(scene, &collision);
    }

    return scene.background_colour.clone();
}

fn calculate_colour(scene: &Scene, collision: &RayCollision) -> Colour {
    let material_at_intersection = collision.object.texture.material_at_point(&collision.intersection);

    let mut colour = material_at_intersection.ambient_colour * &scene.ambient_light;

    for light in &scene.lights {
        // Get a ray from the light to the point_of_intersection.
        let light_ray = light.ray_to(&collision.intersection);

        // See if there is another object closer; if so it will be blocking the light ray.
        if let Some(other) = scene.first_collision_with_ray(&light_ray) {
            if !ptr::eq(collision.object, other.object) {
                continue;
            }
        }

        // Nothing blocking, add the colour contribution for the light.
        let lambertian = -light_ray.dir.dot(&collision.normal) * material_at_intersection.diffuse_colour * light.colour;

        colour = colour + lambertian.clamp();
    }

    return colour;
}
