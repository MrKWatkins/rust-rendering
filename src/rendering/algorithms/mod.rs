mod ray_tracing;
pub use self::ray_tracing::RayTracing;

mod gradient;
pub use self::gradient::Gradient;

use crate::image::Colour;
use crate::maths::Coordinates;
use crate::scene::Scene;

pub trait Algorithm {
    fn render_point(&self, scene: &Scene, camera_space_coordinates: &Coordinates) -> Colour;
}
