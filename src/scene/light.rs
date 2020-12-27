use crate::image::Colour;
use crate::maths::ray;
use crate::maths::{sphere, Isometry, Point, Ray, Scalar};
use std::iter::once;

// TODO: Config.
// TODO: Cached uniform/random.
// TODO: Hemisphere only.

pub struct Light {
    pub position: Point,
    pub colour: Colour,
    pub shape: LightShape,
    pub sample_factor: Scalar,
    transformation: Isometry,
    cached_samples: Option<Vec<Point>>,
}

pub enum LightShape {
    Point,
    Sphere {
        radius: Scalar,
        sampling: LightSampling,
        sample_count: usize,
    },
}

pub enum LightSampling {
    Random,
    Uniform,
}

impl Light {
    pub fn point(position: Point, colour: Colour) -> Light {
        return Light::new(position, colour, LightShape::Point, 1, true);
    }

    pub fn spherical(position: Point, colour: Colour, radius: Scalar, sampling: LightSampling, sample_count: usize, cache_samples: bool) -> Light {
        let shape = LightShape::Sphere {
            radius,
            sampling,
            sample_count,
        };

        return Light::new(position, colour, shape, sample_count, cache_samples);
    }

    fn new(position: Point, colour: Colour, shape: LightShape, sample_count: usize, cache_samples: bool) -> Light {
        let transformation = Isometry::translation(position.x, position.y, position.z);

        let cached_samples = match cache_samples {
            false => None,
            true => Some(shape.surface_points(&transformation).collect()),
        };

        return Light {
            position,
            colour,
            shape,
            sample_factor: 1.0 / sample_count as Scalar,
            transformation,
            cached_samples,
        };
    }

    pub fn sample_rays_to<'a>(&'a self, point: &'a Point) -> Box<dyn Iterator<Item = Ray> + 'a> {
        return match &self.cached_samples {
            Some(samples) => Box::new(samples.iter().map(move |sample| ray::between(&sample, point))),
            None => Box::new(self.shape.surface_points(&self.transformation).map(move |sample| ray::between(&sample, point))),
        };
    }
}

impl LightShape {
    pub fn surface_points<'a>(&self, transformation: &'a Isometry) -> Box<dyn Iterator<Item = Point> + 'a> {
        return match self {
            LightShape::Point => Box::new(once(transformation * Point::origin())),
            LightShape::Sphere {
                radius,
                sampling,
                sample_count,
            } => Box::new(sampling.surface_points(*radius, *sample_count).map(move |sample| transformation * sample)),
        };
    }
}

impl LightSampling {
    pub fn surface_points(&self, radius: Scalar, sample_count: usize) -> Box<dyn Iterator<Item = Point>> {
        return match self {
            LightSampling::Random => Box::new(sphere::random_surface_points(radius).take(sample_count)),
            LightSampling::Uniform => Box::new(sphere::uniform_surface_points(radius, sample_count)),
        };
    }
}
