use crate::image::Colour;
use crate::maths::{Point, Scalar};

pub enum Texture {
    Solid { colour: Colour },
    Chequerboard { colour1: Colour, colour2: Colour, size: Scalar },
}

impl Texture {
    pub fn colour_at(&self, point: &Point) -> &Colour {
        return match self {
            Texture::Solid { colour } => colour,
            Texture::Chequerboard { colour1, colour2, size } => {
                let x = (point.x / size).round() as isize;
                let y = (point.y / size).round() as isize;
                let z = (point.z / size).round() as isize;

                if ((x + y + z) & 1) == 0 {
                    return colour1;
                }
                return colour2;
            }
        };
    }
}
