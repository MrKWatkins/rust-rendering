use crate::image::Colour;
use crate::maths::{vector, Point, Ray, Scalar, Vector};
use crate::scene::{Camera, Light, Object};
use nalgebra::Unit;
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
            Some(interference) => Some(RayCollision::new(
                ray,
                ray.point_at(interference.inter.toi),
                interference.inter.normal,
                interference.co.data(),
            )),
            None => None,
        };
    }
}

// TODO: Use laziness?
pub struct RayCollision<'a> {
    pub ray: &'a Ray,
    pub intersection: Point,
    pub normal: Unit<Vector>,
    pub object: &'a Object,
    _private: (),
}

impl RayCollision<'_> {
    fn new<'a>(ray: &'a Ray, intersection: Point, normal: Vector, object: &'a Object) -> RayCollision<'a> {
        return RayCollision {
            ray,
            intersection,
            normal: Unit::new_normalize(normal),
            object,
            _private: (),
        };
    }

    pub fn reflection_ray(&self) -> Ray {
        return Ray::new(self.intersection, vector::reflect(&self.ray.dir, &self.normal));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{Material, Texture};
    use approx::assert_abs_diff_eq;
    use nalgebra::Matrix;
    use rstest::rstest;

    #[rstest(
        direction,
        normal,
        expected_reflected_direction,
        case(Vector::x(), Matrix::normalize(&Vector::new(1.0, 1.0, 0.0)), Vector::y()),
        case(Vector::z(), Matrix::normalize(&Vector::new(0.0, (45.0f32 / 2.0).to_radians().tan(), 1.0)), Matrix::normalize(&Vector::new(0.0, 1.0, 1.0)))
    )]
    fn reflection_ray(direction: Vector, normal: Vector, expected_reflected_direction: Vector) {
        let ray = Ray::new(Point::origin(), direction);
        let intersection = Point::new(-1.0, 0.0, 0.0);
        let object = Object::new_sphere(
            Point::origin(),
            1.0,
            Texture::Solid {
                material: Material::matte(Colour::black()),
            },
        );

        let collision = RayCollision::new(&ray, intersection, normal, &object);

        let reflection = collision.reflection_ray();

        assert_abs_diff_eq!(reflection.origin.x, intersection.x, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.origin.y, intersection.y, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.origin.z, intersection.z, epsilon = 0.01);

        let expected_dir = expected_reflected_direction;

        assert_abs_diff_eq!(reflection.dir.x, expected_dir.x, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.dir.y, expected_dir.y, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.dir.z, expected_dir.z, epsilon = 0.01);
    }
}
