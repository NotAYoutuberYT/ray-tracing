use crate::constants::{FOCAL_LENGTH, VIEWPORT_HEIGHT_VECTOR, VIEWPORT_WIDTH_VECTOR};
use crate::ray::Ray;
use crate::vector3::Vector3;

/// Represents a camera to render an image from
pub struct Camera {
    position: Vector3,
    viewport_lower_left_corner: Vector3,
}

impl Camera {
    /// Creates a new camera from a position
    pub fn new(position: Vector3) -> Camera {
        Camera {
            position,
            viewport_lower_left_corner: position
                - VIEWPORT_WIDTH_VECTOR / 2.0
                - VIEWPORT_HEIGHT_VECTOR / 2.0
                - Vector3::new(0.0, 0.0, FOCAL_LENGTH),
        }
    }

    /// Gets a ray from the camera given an f64 from 0 to 1 representing how
    /// far across the width or height the image is (0, 0 is bottom left)
    pub fn get_ray(&self, width_progress: f64, height_progress: f64) -> Ray {
        Ray::new(
            self.position,
            self.viewport_lower_left_corner
                + width_progress * VIEWPORT_WIDTH_VECTOR
                + height_progress * VIEWPORT_HEIGHT_VECTOR
                - self.position,
        )
    }
}
