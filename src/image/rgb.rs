use crate::image::Colour;
use crate::maths::Scalar;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        return Rgb { r, g, b };
    }

    pub fn from_hex(hex: u32) -> Rgb {
        return Rgb {
            r: ((hex >> 16) & 0xff) as u8,
            g: ((hex >> 8) & 0xff) as u8,
            b: (hex & 0xff) as u8,
        };
    }

    pub fn to_colour(&self) -> Colour {
        return Colour::new(self.r as Scalar / 255.0, self.g as Scalar / 255.0, self.b as Scalar / 255.0);
    }

    pub fn to_u8_array(&self) -> [u8; 3] {
        return [self.r, self.g, self.b];
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "0x{:02x?}{:02x?}{:02x?}", self.r, self.g, self.b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use rstest::rstest;

    #[test]
    fn new() {
        let rgb = Rgb::new(0x12, 0x78, 0xde);

        assert_eq!(rgb.r, 0x12);
        assert_eq!(rgb.g, 0x78);
        assert_eq!(rgb.b, 0xde);
    }

    #[test]
    fn from_hex() {
        let rgb = Rgb::from_hex(0x1278de);

        assert_eq!(rgb, Rgb::new(0x12, 0x78, 0xde));
        assert_eq!(rgb.r, 0x12);
        assert_eq!(rgb.g, 0x78);
        assert_eq!(rgb.b, 0xde);
    }

    #[rstest(
        r,
        g,
        b,
        expected_x,
        expected_y,
        expected_z,
        case(0x00, 0x00, 0x00, 0.0, 0.0, 0.0),
        case(0xff, 0xff, 0xff, 1.0, 1.0, 1.0),
        case(0x3f, 0x7f, 0xbf, 0.25, 0.5, 0.75)
    )]
    fn to_colour(r: u8, g: u8, b: u8, expected_x: Scalar, expected_y: Scalar, expected_z: Scalar) {
        let rgb = Rgb { r, g, b };
        let colour = rgb.to_colour();

        assert_abs_diff_eq!(colour.r, expected_x, epsilon = 0.01);
        assert_abs_diff_eq!(colour.g, expected_y, epsilon = 0.01);
        assert_abs_diff_eq!(colour.b, expected_z, epsilon = 0.01);
    }

    #[test]
    fn fmt() {
        let rgb = Rgb::new(0x12, 0x78, 0xde);

        assert_eq!(rgb.to_string(), "0x1278de");
    }

    #[test]
    fn to_u8_array() {
        let rgb = Rgb::new(0x12, 0x78, 0xde);

        assert_eq!(rgb.to_u8_array(), [0x12, 0x78, 0xde]);
    }
}
