use crate::configuration::from_command_line;
use crate::image::Colour;
use crate::material::Material;
use crate::maths::Point;
use crate::rendering::algorithms::RayTracing;
use crate::rendering::render;
use crate::scene::{Object, Scene};
use std::time::Instant;

// Some modules declared as pub to suppress dead code warnings.
mod configuration;
pub mod image;
pub mod material;
pub mod maths;
pub mod rendering;
pub mod scene;

fn main() {
    let configuration = from_command_line().unwrap();

    println!("Output file: {:?}", configuration.output);
    println!("Image size: {}x{}", configuration.width, configuration.height);

    let algorithm = RayTracing::new();
    let scene = build_scene();

    let image = time_function("render", || render(&algorithm, &configuration, &scene));

    let rgb = time_function("to_rgb_image", || image.to_rgb_image());

    time_function("save", || {
        rgb.save(&configuration.output)
            .unwrap_or_else(|e| println!("Unexpected error: {}", e.to_string()))
    });
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    scene.add(Object::sphere(&Point::new(0.0, 0.0, 1.0), 0.1, Material::matte(Colour::new(0.0, 1.0, 0.0))));

    return scene;
}

fn time_function<TResult>(name: &str, function: impl Fn() -> TResult) -> TResult {
    let start = Instant::now();

    let result = function();
    let duration = start.elapsed();

    println!("Time elapsed for {} is: {:?}", name, duration);

    return result;
}
