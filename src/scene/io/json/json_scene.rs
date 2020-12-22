use crate::scene::io::json::{JsonCamera, JsonObject};
use crate::scene::Scene;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct JsonScene {
    pub camera: Option<JsonCamera>,
    pub objects: Vec<JsonObject>,
}

impl JsonScene {
    pub fn load(path: &Path) -> Result<JsonScene, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let json_scene: JsonScene = serde_json::from_reader(reader)?;

        return Ok(json_scene);
    }

    pub fn to_scene(&self) -> Scene {
        let mut scene = Scene::new();

        if let Some(camera) = &self.camera {
            scene.camera = camera.to_camera();
        }

        for object in &self.objects {
            scene.add_object(object.to_object());
        }

        return scene;
    }
}
