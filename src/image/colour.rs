use crate::image::Rgb;
use crate::maths::Scalar;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Default, Clone)]
pub struct Colour {
    pub r: Scalar,
    pub g: Scalar,
    pub b: Scalar,
}

impl Colour {
    pub fn new(r: Scalar, g: Scalar, b: Scalar) -> Colour {
        return Colour { r, g, b };
    }

    pub fn average(colours: &[Colour]) -> Colour {
        return colours.iter().sum::<Colour>() / colours.len() as f32;
    }

    pub fn to_rgb(&self) -> Rgb {
        let rgb = self.clamp() * 255.0;
        return Rgb {
            r: rgb.r as u8,
            g: rgb.g as u8,
            b: rgb.b as u8,
        };
    }

    pub fn clamp(&self) -> Colour {
        return Colour {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        };
    }

    pub fn black() -> Colour {
        return Colour::new(0.0, 0.0, 0.0);
    }
}

// TODO: Clamp on operators.
macro_rules! colour_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<Colour> for Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: Colour) -> Colour {
                return Colour {
                    r: self.r $operator other.r,
                    g: self.g $operator other.g,
                    b: self.b $operator other.b,
                };
            }
        }

        impl $trait<&Colour> for Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: &Colour) -> Colour {
                return Colour {
                    r: self.r $operator other.r,
                    g: self.g $operator other.g,
                    b: self.b $operator other.b,
                };
            }
        }

        impl $trait<Colour> for &Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: Colour) -> Colour {
                return Colour {
                    r: self.r $operator other.r,
                    g: self.g $operator other.g,
                    b: self.b $operator other.b,
                };
            }
        }

        impl $trait<&Colour> for &Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: &Colour) -> Colour {
                return Colour {
                    r: self.r $operator other.r,
                    g: self.g $operator other.g,
                    b: self.b $operator other.b,
                };
            }
        }
    };
}

colour_operator! { Add, add, + }
colour_operator! { Sub, sub, - }
colour_operator! { Mul, mul, * }

macro_rules! colour_scalar_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<Scalar> for Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: Scalar) -> Colour {
                return Colour {
                    r: self.r $operator other,
                    g: self.g $operator other,
                    b: self.b $operator other,
                };
            }
        }

        impl $trait<Scalar> for &Colour {
            type Output = Colour;

            #[inline]
            fn $method(self, other: Scalar) -> Colour {
                return Colour {
                    r: self.r $operator other,
                    g: self.g $operator other,
                    b: self.b $operator other,
                };
            }
        }
    };
}

colour_scalar_operator! { Mul, mul, * }
colour_scalar_operator! { Div, div, / }

macro_rules! scalar_colour_operator {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<Colour> for Scalar {
            type Output = Colour;

            #[inline]
            fn $method(self, other: Colour) -> Colour {
                return Colour {
                    r: other.r $operator self,
                    g: other.g $operator self,
                    b: other.b $operator self,
                };
            }
        }

        impl $trait<&Colour> for Scalar {
            type Output = Colour;

            #[inline]
            fn $method(self, other: &Colour) -> Colour {
                return Colour {
                    r: other.r $operator self,
                    g: other.g $operator self,
                    b: other.b $operator self,
                };
            }
        }
    };
}

scalar_colour_operator! { Mul, mul, * }

impl std::iter::Sum for Colour {
    fn sum<I: Iterator<Item = Colour>>(iter: I) -> Colour {
        return iter.fold(Colour::black(), |x, y| x + y);
    }
}

impl<'a> std::iter::Sum<&'a Colour> for Colour {
    fn sum<I: Iterator<Item = &'a Colour>>(iter: I) -> Colour {
        return iter.fold(Colour::black(), |x, y| x + y);
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
    use approx::assert_abs_diff_eq;
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
    fn to_rgb(x: Scalar, y: Scalar, z: Scalar, expected_r: u8, expected_g: u8, expected_b: u8) {
        let colour = Colour::new(x, y, z);
        let rgb = colour.to_rgb();

        assert_eq!(rgb.r, expected_r);
        assert_eq!(rgb.g, expected_g);
        assert_eq!(rgb.b, expected_b);
    }

    #[test]
    fn add_value_value() {
        let actual = Colour::new(0.1, 0.2, 1.5) + Colour::new(0.5, 0.1, -0.3);
        let expected = Colour::new(0.6, 0.3, 1.2);
        assert_eq(actual, expected);
    }

    #[test]
    fn add_value_reference() {
        let actual = Colour::new(0.1, 0.2, 1.5) + &Colour::new(0.5, 0.1, -0.3);
        let expected = Colour::new(0.6, 0.3, 1.2);
        assert_eq(actual, expected);
    }

    #[test]
    fn add_reference_value() {
        let actual = &Colour::new(0.1, 0.2, 1.5) + Colour::new(0.5, 0.1, -0.3);
        let expected = Colour::new(0.6, 0.3, 1.2);
        assert_eq(actual, expected);
    }

    #[test]
    fn add_reference_reference() {
        let actual = &Colour::new(0.1, 0.2, 1.5) + &Colour::new(0.5, 0.1, -0.3);
        let expected = Colour::new(0.6, 0.3, 1.2);
        assert_eq(actual, expected);
    }

    #[test]
    fn mul_value_value() {
        let actual = Colour::new(0.1, 0.2, 1.5) * Colour::new(0.5, 1.0, 0.0);
        let expected = Colour::new(0.05, 0.2, 0.0);
        assert_eq(actual, expected);
    }

    #[test]
    fn sub_value_value() {
        let actual = Colour::new(0.1, 1.0, 1.5) - Colour::new(0.5, 0.2, 0.0);
        let expected = Colour::new(-0.4, 0.8, 1.5);
        assert_eq(actual, expected);
    }

    #[test]
    fn sum_by_value() {
        let colours = vec![Colour::new(0.1, 0.2, 1.5), Colour::new(0.5, 0.1, -0.3), Colour::new(0.3, 0.2, 0.3)];
        let actual: Colour = colours.into_iter().sum();
        let expected = Colour::new(0.9, 0.5, 1.5);
        assert_eq(actual, expected);
    }

    #[test]
    fn sum_by_reference() {
        let colours = vec![Colour::new(0.1, 0.2, 1.5), Colour::new(0.5, 0.1, -0.3), Colour::new(0.3, 0.2, 0.3)];
        let actual: Colour = colours.iter().sum();
        let expected = Colour::new(0.9, 0.5, 1.5);
        assert_eq(actual, expected);
    }

    #[test]
    fn div() {
        let actual = Colour::new(0.1, 0.2, 1.5) / 10.0;
        let expected = Colour::new(0.01, 0.02, 0.15);
        assert_eq(actual, expected);
    }

    #[test]
    fn average() {
        let colours = vec![Colour::new(0.1, 0.2, 1.5), Colour::new(0.5, 0.3, -0.3), Colour::new(0.3, 0.1, 0.3)];
        let actual: Colour = Colour::average(&colours);
        let expected = Colour::new(0.3, 0.2, 0.5);
        assert_eq(actual, expected);
    }

    fn assert_eq(actual: Colour, expected: Colour) {
        assert_abs_diff_eq!(actual.r, expected.r, epsilon = 0.01);
        assert_abs_diff_eq!(actual.g, expected.g, epsilon = 0.01);
        assert_abs_diff_eq!(actual.b, expected.b, epsilon = 0.01);
    }
}
