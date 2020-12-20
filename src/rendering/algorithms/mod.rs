mod ray_tracing;
pub use self::ray_tracing::RayTracing;

mod gradient;
pub use self::gradient::Gradient;

use crate::image::Colour;
use crate::rendering::ScreenSpaceCoords;
use crate::scene::Scene;

pub trait Algorithm {
    fn render_point(&self, scene: &Scene, coordinates: &ScreenSpaceCoords) -> Colour;
}
