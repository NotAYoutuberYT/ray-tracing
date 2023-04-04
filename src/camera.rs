use crate::quaternion::Quaternion;
use crate::ray::Ray;
use crate::vector3::Vector3;
use std::f64::consts::PI;

/// Represents a camera to render an image from
#[derive(Copy, Clone)]
pub struct Camera {
    position: Vector3,
    rotation: Quaternion,
    unrotated_viewport_lower_left_corner: Vector3,
    unrotated_viewport_width_vector: Vector3,
    unrotated_viewport_height_vector: Vector3,
}

impl Camera {
    /// Creates a new camera from a position
    pub fn new(
        position: Vector3,
        rotation: Quaternion,
        horizontal_fov_degrees: f64,
        aspect_ratio: f64,
    ) -> Camera {
        // angle used for calculations (degrees -> radians / 2)
        let theta = horizontal_fov_degrees * PI / 360.0;

        // make calculations (because we're given an fov and not
        // viewport info, we can assume a focal length of 1)
        let viewport_width = theta.tan() * 2.0;
        let viewport_height = viewport_width / aspect_ratio;
        let unrotated_viewport_width_vector =
            Vector3::new(0.0, viewport_width, 0.0).rotate_by(rotation);
        let unrotated_viewport_height_vector =
            Vector3::new(0.0, 0.0, viewport_height).rotate_by(rotation);

        let focal_length_vector = Vector3::new(1.0, 0.0, 0.0).rotate_by(rotation);
        let unrotated_viewport_lower_left_corner = position
            - unrotated_viewport_width_vector / 2.0
            - unrotated_viewport_height_vector / 2.0
            + focal_length_vector;

        // return the created camera
        Camera {
            position,
            rotation,
            unrotated_viewport_lower_left_corner,
            unrotated_viewport_width_vector,
            unrotated_viewport_height_vector,
        }
    }

    /// Gets a ray from the camera given an f64 from 0 to 1 representing how
    /// far across the width or height the image is (0, 0 is bottom left)
    pub fn get_ray(&self, width_progress: f64, height_progress: f64) -> Ray {
        Ray::new(
            self.position,
            (self.unrotated_viewport_lower_left_corner
                + width_progress * self.unrotated_viewport_width_vector
                + height_progress * self.unrotated_viewport_height_vector
                - self.position),
        )
    }
}
