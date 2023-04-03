use crate::vector3::Vector3;

/// A simple ray representing a ray of light
#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    /// Creates a new ray from an origin and a direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }

    /// Samples the ray `distance` units away from the origin
    pub fn at(self, distance: f64) -> Vector3 {
        self.origin + self.direction * distance
    }

    /// Returns the origin of the ray
    pub fn origin(self) -> Vector3 {
        self.origin
    }

    /// Returns the direction of the ray
    pub fn direction(self) -> Vector3 {
        self.direction
    }
}
