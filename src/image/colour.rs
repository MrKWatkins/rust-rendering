use crate::image::Rgb;
use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        return Colour { r, g, b };
    }

    pub fn to_rgb(&self) -> Rgb {
        return Rgb {
            r: (self.r.min(1.0).max(0.0) * 255.0) as u8,
            g: (self.g.min(1.0).max(0.0) * 255.0) as u8,
            b: (self.b.min(1.0).max(0.0) * 255.0) as u8,
        };
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "0x{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
    x,
    y,
    z,
    expected_r,
    expected_g,
    expected_b,
    case(0.0, 0.0, 0.0, 0x00, 0x00, 0x00),
    case(1.0, 1.0, 1.0, 0xff, 0xff, 0xff),
    case(-1.0, -0.1, -50.0, 0x00, 0x00, 0x00),
    case(2.0, 1.1, 100.0, 0xff, 0xff, 0xff),
    case(0.25, 0.5, 0.75, 0x3f, 0x7f, 0xbf)
    )]
    fn to_rgb(x: f64, y: f64, z: f64, expected_r: u8, expected_g: u8, expected_b: u8) {
        let colour = Colour::new(x, y, z);
        let rgb = colour.to_rgb();

        assert_eq!(rgb.r, expected_r);
        assert_eq!(rgb.g, expected_g);
        assert_eq!(rgb.b, expected_b);
    }
}
