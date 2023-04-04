use crate::constants::{FOCAL_LENGTH, VIEWPORT_HEIGHT_VECTOR, VIEWPORT_WIDTH_VECTOR};
use crate::ray::Ray;
use crate::vector3::Vector3;

/// Represents a camera to render an image from
#[derive(Copy, Clone)]
pub struct Camera {
    position: Vector3,
    viewport_lower_left_corner: Vector3,
    viewport_width_vector: Vector3,
    viewport_height_vector: Vector3,
}

impl Camera {
    /// Creates a new camera from a position
    pub fn new(position: Vector3, direction: Vector3, horizontal_fov_degrees: f64) -> Camera {
        let direction_normalized = direction.normalized();

        let viewport_width_vector = Vector3::new()
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
