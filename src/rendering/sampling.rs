use crate::maths::{Coordinates, Scalar};

pub enum SubPixelSampling {
    None,
    Square(u8),
}

impl SubPixelSampling {
    pub fn pixel_offsets(&self) -> Vec<Coordinates> {
        match self {
            Self::None => vec![Coordinates::new(0.5, 0.5)],
            Self::Square(n) => {
                let number = *n;
                let size_of_sub_pixel = 1.0 / number as Scalar;
                let offset_from_side = size_of_sub_pixel / 2.0;

                let mut result = Vec::with_capacity(number as usize * number as usize);

                for x in 0..number {
                    for y in 0..number {
                        result.push(Coordinates::new(
                            x as f32 * size_of_sub_pixel + offset_from_side,
                            y as f32 * size_of_sub_pixel + offset_from_side,
                        ));
                    }
                }

                return result;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
    sampling,
    expected,
    case(SubPixelSampling::None, vec![Coordinates::new(0.5, 0.5)]),
    case(SubPixelSampling::Square(2), vec![Coordinates::new(0.25, 0.25), Coordinates::new(0.25, 0.75), Coordinates::new(0.75, 0.25), Coordinates::new(0.75, 0.75)])
    )]
    fn pixel_offsets(sampling: SubPixelSampling, expected: Vec<Coordinates>) {
        let actual = sampling.pixel_offsets();

        assert_eq!(actual, expected);
    }
}
