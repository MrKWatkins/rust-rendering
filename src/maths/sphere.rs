use crate::maths::{consts, Point, Scalar};
use rand::{thread_rng, Rng};

pub fn point_from_spherical_coords(radius: Scalar, theta: Scalar, phi: Scalar) -> Point {
    // Benchmarking showed it was a little quicker (~7%) to multiple each component by radius, rather than
    // multiply the point as a whole by radius, presumably due to avoiding the allocation and construction
    // of the intermediate point.
    return Point::new(radius * theta.sin() * phi.cos(), radius * theta.sin() * phi.sin(), radius * theta.cos());
}

pub fn point_from_latitude_and_longitude(radius: Scalar, latitude: Scalar, longitude: Scalar) -> Point {
    return Point::new(
        radius * latitude.cos() * longitude.cos(),
        radius * latitude.cos() * longitude.sin(),
        radius * latitude.sin(),
    );
}

// Based on http://extremelearning.com.au/how-to-generate-uniformly-random-points-on-n-spheres-and-n-balls, method 10.
pub fn random_surface_point<TRng: Rng>(rng: &mut TRng, radius: Scalar) -> Point {
    let u: Scalar = rng.gen();
    let v: Scalar = rng.gen();

    let theta = consts::TWO_PI * u;
    let phi = (2.0 * v - 1.0).acos();

    return point_from_spherical_coords(radius, theta, phi);
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct RandomSurfacePointIterator {
    radius: Scalar,
}

impl Iterator for RandomSurfacePointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        return Some(random_surface_point(&mut thread_rng(), self.radius));
    }
}

pub fn random_surface_points(radius: Scalar) -> RandomSurfacePointIterator {
    return RandomSurfacePointIterator { radius };
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct UniformSurfacePointIterator {
    radius: Scalar,
    multiplier: Scalar,
    number_of_points: usize,
    point: usize,
}

impl Iterator for UniformSurfacePointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.point > self.number_of_points {
            return None;
        }

        let latitude = (self.point as Scalar * self.multiplier - 1.0).asin();
        let longitude = consts::GOLDEN_ANGLE * self.point as Scalar;

        self.point += 1;

        return Some(point_from_latitude_and_longitude(self.radius, latitude, longitude));
    }
}

// Based on https://bduvenhage.me/geometry/2019/07/31/generating-equidistant-vectors.html.
pub fn uniform_surface_points(radius: Scalar, number_of_points: usize) -> UniformSurfacePointIterator {
    return UniformSurfacePointIterator {
        radius,
        multiplier: 2.0 / (number_of_points + 1) as Scalar,
        number_of_points,
        point: 1,
    };
}
