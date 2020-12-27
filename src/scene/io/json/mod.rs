mod json_camera;
pub use self::json_camera::JsonCamera;

mod json_colour;
pub use self::json_colour::*;

mod json_light;
pub use self::json_light::*;

mod json_material;
pub use self::json_material::*;

mod json_object;
pub use self::json_object::*;

mod json_point;
pub use self::json_point::*;

mod json_scene;
pub use self::json_scene::*;

mod json_texture;
pub use self::json_texture::*;

mod json_vector;
pub use self::json_vector::*;

use crate::scene::Scene;
use std::error::Error;
use std::path::Path;

pub type JsonScalar = f32;

pub fn load(path: &Path) -> Result<Scene, Box<dyn Error>> {
    let json_scene = JsonScene::load(path)?;

    return Ok(json_scene.to_scene());
}
