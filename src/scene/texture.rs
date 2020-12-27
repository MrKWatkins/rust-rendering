use crate::maths::{Point, Scalar};
use crate::scene::Material;

pub enum Texture {
    Solid { material: Material },
    Chequerboard { material1: Material, material2: Material, size: Scalar },
}

impl Texture {
    pub fn material_at_point(&self, point: &Point) -> &Material {
        return match self {
            Texture::Solid { material } => material,
            Texture::Chequerboard { material1, material2, size } => {
                let x = (point.x / size).round() as isize;
                let y = (point.y / size).round() as isize;
                let z = (point.z / size).round() as isize;

                if ((x + y + z) & 1) == 0 {
                    return material1;
                }
                return material2;
            }
        };
    }
}
