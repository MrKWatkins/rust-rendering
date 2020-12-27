use crate::maths::{Unit, Vector};

pub fn reflect(vector: &Vector, about: &Unit<Vector>) -> Vector {
    let about_vector = &about.into_inner();
    return 2.0 * vector.dot(about_vector) * about_vector - vector;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use nalgebra::Matrix;
    use rstest::rstest;

    #[rstest(
    direction,
    about,
    expected_reflected_direction,
    case(Vector::x(), Unit::new_normalize(Vector::new(1.0, 0.0, 1.0)), Vector::z()),
    case(Vector::z(), Unit::new_normalize(Vector::new((45.0f32 / 2.0).to_radians().tan(), 0.0, 1.0)), Matrix::normalize(&Vector::new(1.0, 0.0, 1.0)))
    )]
    fn reflect(direction: Vector, about: Unit<Vector>, expected_reflected_direction: Vector) {
        let reflection = super::reflect(&direction, &about);

        let expected = expected_reflected_direction;

        assert_abs_diff_eq!(reflection.x, expected.x, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.y, expected.y, epsilon = 0.01);
        assert_abs_diff_eq!(reflection.z, expected.z, epsilon = 0.01);
    }
}
