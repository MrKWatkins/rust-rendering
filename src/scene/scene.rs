use crate::image::Colour;
use crate::maths::{Ray, RayIntersection, Scalar};
use crate::scene::{Camera, Light, Object};
use ncollide3d::pipeline::{CollisionGroups, GeometricQueryType};
use ncollide3d::world::CollisionWorld;

pub struct Scene {
    pub ambient_light: Colour,
    pub background_colour: Colour,
    pub camera: Camera,
    pub lights: Vec<Light>,
    world: CollisionWorld<Scalar, Object>,
    groups: CollisionGroups,
    query: GeometricQueryType<Scalar>,
}

pub struct RayCollision<'a> {
    pub ray: &'a Ray,
    pub intersection: RayIntersection,
    pub object: &'a Object,
}

impl Scene {
    pub fn new() -> Scene {
        return Scene {
            ambient_light: Colour::black(),
            background_colour: Colour::black(),
            camera: Camera::default(),
            lights: vec![],
            world: CollisionWorld::<Scalar, Object>::new(0.01),
            groups: CollisionGroups::new(),
            query: GeometricQueryType::Contacts(0.0, 0.0),
        };
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_lights(&mut self, lights: impl IntoIterator<Item = Light>) {
        for light in lights {
            self.lights.push(light);
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.world.add(object.transformation, object.shape.clone(), self.groups, self.query, object);
        self.world.update();
    }

    pub fn add_objects(&mut self, objects: impl IntoIterator<Item = Object>) {
        for object in objects {
            self.world.add(object.transformation, object.shape.clone(), self.groups, self.query, object);
        }
        self.world.update();
    }

    pub fn first_collision_with_ray<'a>(&'a self, ray: &'a Ray) -> Option<RayCollision<'a>> {
        return match self.world.first_interference_with_ray(&ray, std::f32::MAX, &self.groups) {
            Some(interference) => Some(RayCollision {
                ray,
                intersection: interference.inter,
                object: interference.co.data(),
            }),
            None => None,
        };
    }
}
