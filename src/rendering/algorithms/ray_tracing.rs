use crate::image::Colour;
use crate::maths::{vector, Coordinates, Ray, Scalar};
use crate::rendering::algorithms::Algorithm;
use crate::scene::{RayCollision, Scene};
use nalgebra::{distance, Unit};

// Cutoff used for contributions from light intensity and reflections. If the amount is going to be less than one
// notch in an RGB image then ignore it.
const MINIMUM_INTENSITY: Scalar = 1.0 / 256.0;

const RECURSION_DEPTH: usize = 10;

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
        let ray = scene.camera.ray_to(camera_space_coordinates);
        return trace_ray(scene, &ray, 0);
    }
}

fn trace_ray(scene: &Scene, ray: &Ray, recursion_depth: usize) -> Colour {
    if let Some(collision) = scene.first_collision_with_ray(&ray) {
        return calculate_colour(scene, &collision, recursion_depth);
    }

    return scene.background_colour.clone();
}

fn calculate_colour(scene: &Scene, collision: &RayCollision, recursion_depth: usize) -> Colour {
    let material = collision.object.texture.material_at_point(&collision.intersection);

    let mut colour = material.ambient_colour * &scene.ambient_light;

    for light in &scene.lights {
        // Sample rays from the light to the point_of_intersection.
        for light_ray in light.sample_rays_to(&collision.intersection) {
            // This could be approximated and done once per light using the position of the light.
            let intensity = light.attenuation.get_intensity(distance(&light_ray.origin, &collision.intersection));
            if intensity < MINIMUM_INTENSITY {
                continue;
            }

            // See if there is another object closer; if so it will be blocking the light ray.
            // TODO: An object could block itself! Check the collision point is the same.
            if let Some(other) = scene.first_collision_with_ray(&light_ray) {
                if collision.object != other.object {
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
            colour = colour + light_dot_normal * material.diffuse_colour * light.colour * light.sample_factor * intensity;

            // Specular contribution.
            if material.shininess > 0.0 {
                let reflection = vector::reflect(&light_ray.dir, &collision.normal);
                let to_viewer = Unit::new_normalize(collision.intersection - scene.camera.position);
                let r_dot_v = reflection.dot(&to_viewer);
                if r_dot_v > 0.0 {
                    // light.colour and sample_factor already factored in above. Not bothering with separate diffuse/specular colours for a light.
                    colour = colour + material.specular_colour * r_dot_v.powf(material.shininess) * light.sample_factor * intensity;
                }
            }
        }
    }

    // Calculate the reflectivity if necessary. Don't bother if the colour is already white!
    if recursion_depth < RECURSION_DEPTH && material.reflectivity > MINIMUM_INTENSITY && (colour.r < 1.0 || colour.g < 1.0 || colour.b < 1.0) {
        // Trace a ray out from the collision point.
        let reflection_ray = collision.reflection_ray();

        // Need to exclude the current object or we might collide with that due to floating point imprecision.
        if let Some(reflection_collision) = scene.first_collision_with_ray_excluding(&reflection_ray, collision.object) {
            colour = colour + material.reflectivity * calculate_colour(scene, &reflection_collision, recursion_depth + 1);
        }
    }

    return colour.clamp();
}
