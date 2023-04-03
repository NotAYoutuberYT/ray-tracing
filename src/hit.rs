use crate::material::Material;
use crate::vector3::Vector3;

/// Represents a hit of an object by a ray
/// Note: the normal will always face outwards
#[derive(Copy, Clone)]
pub struct Hit {
    pub distance: f64,
    pub point: Vector3,
    pub normal: Vector3,
    pub outside_face: bool,
    pub material: Material,
}

impl Hit {
    /// Creates a new Hit from a distance, point, and normal
    pub fn new(
        distance: f64,
        point: Vector3,
        normal: Vector3,
        outside_face: bool,
        material: Material,
    ) -> Hit {
        Hit {
            distance,
            point,
            normal,
            outside_face,
            material,
        }
    }
}
