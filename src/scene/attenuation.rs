use crate::maths::Scalar;

pub enum Attenuation {
    None,
    Linear { multiplier: Scalar, maximum_range: Scalar },
    Inverse { multiplier: Scalar },
    InverseSquared { multiplier: Scalar },
}

impl Attenuation {
    pub fn new_none() -> Attenuation {
        return Attenuation::None;
    }

    pub fn new_linear(half_intensity_distance: Scalar) -> Attenuation {
        // Half intensity at distance d. Therefore 0.5 = kd => k = 0.5/d.
        return Attenuation::Linear {
            multiplier: 0.5 / half_intensity_distance,
            maximum_range: 2.0 * half_intensity_distance,
        };
    }

    pub fn new_inverse(half_intensity_distance: Scalar) -> Attenuation {
        // Half intensity at distance d. Therefore 0.5 = k/d => k = 0.5 d.
        return Attenuation::Inverse {
            multiplier: 0.5 * half_intensity_distance,
        };
    }

    pub fn new_inverse_squared(half_intensity_distance: Scalar) -> Attenuation {
        // Half intensity at distance d. Therefore 0.5 = k/d^2 => k = 0.5 d^2.
        return Attenuation::Inverse {
            multiplier: 0.5 * half_intensity_distance * half_intensity_distance,
        };
    }

    pub fn get_intensity(&self, distance_from_light: Scalar) -> Scalar {
        return match self {
            Attenuation::None => 1.0,

            Attenuation::Linear { multiplier, maximum_range } => {
                if distance_from_light >= *maximum_range {
                    return 0.0;
                }

                return 1.0 - multiplier * distance_from_light;
            }

            Attenuation::Inverse { multiplier } => {
                if distance_from_light == 0.0 {
                    return 1.0;
                }

                return multiplier / distance_from_light;
            }

            Attenuation::InverseSquared { multiplier } => {
                if distance_from_light == 0.0 {
                    return 1.0;
                }

                return multiplier / (distance_from_light * distance_from_light);
            }
        };
    }
}
