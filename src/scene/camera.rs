use crate::maths::{Isometry, Perspective, Point, Scalar, TransformationMatrix, Vector};
use crate::rendering::ScreenSpaceCoords;
use nalgebra::Point4;

pub struct Camera {
    pub position: Point,
    camera_to_world: TransformationMatrix,
}

impl Camera {
    pub fn new(position: Point, looking_at: &Point, up: &Vector, field_of_view_degrees: Scalar) -> Camera {
        let projection = Perspective::new(1.0, field_of_view_degrees.to_radians(), 1.0, 100000.0);

        let transformation = Isometry::look_at_rh(&position, looking_at, up);

        let camera_to_world = (projection.as_matrix() * transformation.to_homogeneous()).try_inverse().unwrap();

        return Camera { position, camera_to_world };
    }

    pub fn to_world_space(&self, camera_space: &ScreenSpaceCoords) -> Point {
        let homogeneous = self.camera_to_world * Point4::new(camera_space.x, camera_space.y, -1.0, 1.0);
        return Point::from_homogeneous(homogeneous.coords).unwrap();
    }
}

impl Default for Camera {
    fn default() -> Camera {
        return Camera::new(Point::new(0.0, 0.0, -1.0), &Point::origin(), &Vector::y(), 30.0);
    }
}
