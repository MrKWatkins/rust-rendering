mod raster_space;
pub use self::raster_space::*;

mod render;
pub use self::render::*;

pub mod algorithms;

use crate::maths::Scalar;

pub type ScreenSpaceCoords = nalgebra::Point2<Scalar>;
