use crate::maths::{Coordinates, Scalar};

pub struct RasterSpace {
    pub width: Scalar,
    pub height: Scalar,
    pub aspect_ratio: Scalar,
    _private: (),
}

impl RasterSpace {
    pub fn new(width: u32, height: u32) -> RasterSpace {
        return RasterSpace {
            width: width as Scalar,
            height: height as Scalar,
            aspect_ratio: width as Scalar / height as Scalar,
            _private: (),
        };
    }

    pub fn to_screen_space(&self, coordinates: Coordinates) -> Coordinates {
        // Transform to normalized device co-ordinates first.
        // 0.5 to put the point in the middle of the pixel.
        let normalized_pixel_x = coordinates.x / self.width;
        let normalized_pixel_y = coordinates.y / self.height;

        let screen_space_x = (2.0 * normalized_pixel_x - 1.0) * self.aspect_ratio;
        let screen_space_y = 1.0 - 2.0 * normalized_pixel_y;

        return Coordinates::new(screen_space_x, screen_space_y);
    }
}
