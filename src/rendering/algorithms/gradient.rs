use crate::image::Colour;
use crate::maths::Scalar;
use crate::rendering::algorithms::Algorithm;
use crate::scene::Scene;

/// Renders a flat gradient. Useful for testing image file output is correct.
pub struct Gradient {
    pub from: Colour,
    pub to: Colour,
    _private: (),
}

impl Gradient {
    pub fn new(from: Colour, to: Colour) -> Gradient {
        return Gradient { from, to, _private: () };
    }
}

impl Algorithm for Gradient {
    fn render_point(&self, x: Scalar, y: Scalar, _: &Scene) -> Colour {
        let adjusted_x = x + 0.5;
        let adjusted_y = y + 0.5;
        let scale = adjusted_x * adjusted_y;

        return Colour::new(
            interpolate(self.from.r, self.to.r, scale),
            interpolate(self.from.g, self.to.g, scale),
            interpolate(self.from.b, self.to.b, scale),
        );
    }
}

fn interpolate(from: Scalar, to: Scalar, position: Scalar) -> Scalar {
    return from + position * (to - from);
}
