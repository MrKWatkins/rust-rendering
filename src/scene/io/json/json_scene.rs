use crate::scene::io::json::{JsonCamera, JsonColour, JsonLight, JsonObject};
use crate::scene::Scene;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub struct JsonScene {
    pub ambient_light: Option<JsonColour>,
    pub background_colour: Option<JsonColour>,
    pub camera: Option<JsonCamera>,
    pub objects: Vec<JsonObject>,
    pub lights: Vec<JsonLight>,
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

        if let Some(ambient_light) = &self.ambient_light {
            scene.ambient_light = ambient_light.to_colour();
        }

        if let Some(background_colour) = &self.background_colour {
            scene.background_colour = background_colour.to_colour();
        }

        if let Some(camera) = &self.camera {
            scene.camera = camera.to_camera();
        }

        scene.add_lights(self.lights.iter().map(|light| light.to_light()));

        scene.add_objects(self.objects.iter().map(|object| object.to_object()));

        return scene;
    }
}
