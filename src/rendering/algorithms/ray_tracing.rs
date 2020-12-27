use crate::image::Colour;
use crate::maths::{vector, Coordinates};
use crate::rendering::algorithms::Algorithm;
use crate::scene::{RayCollision, Scene};
use nalgebra::Unit;
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
    let material = collision.object.texture.material_at_point(&collision.intersection);

    let mut colour = material.ambient_colour * &scene.ambient_light;

    for light in &scene.lights {
        // Sample rays a ray from the light to the point_of_intersection.
        for light_ray in light.sample_rays_to(&collision.intersection) {
            // See if there is another object closer; if so it will be blocking the light ray.
            // TODO: An object could block itself! Check the collision point is the same.
            if let Some(other) = scene.first_collision_with_ray(&light_ray) {
                if !ptr::eq(collision.object, other.object) {
                    continue;
                }
            }

            // Nothing blocking, however if light . normal is negative then the light must have hit the back of the surface
            // so we can ignore it.
            let light_dot_normal = -light_ray.dir.dot(&collision.normal);
            if light_dot_normal <= 0.0 {
                continue;
            }

            // Diffuse contribution.
            colour = colour + light_dot_normal * material.diffuse_colour * light.colour * light.sample_factor;

            // Specular contribution.
            if material.shininess > 0.0 {
                let reflection = vector::reflect(&light_ray.dir, &collision.normal);
                let to_viewer = Unit::new_normalize(collision.intersection - scene.camera.position);
                let r_dot_v = reflection.dot(&to_viewer);
                if r_dot_v > 0.0 {
                    // light.colour and sample_factor already factored in above. Not bothering with separate diffuse/specular colours for a light.
                    colour = colour + material.specular_colour * r_dot_v.powf(material.shininess);
                }
            }
        }
    }

    return colour.clamp();
}
