use crate::vector3::Vector3;

/// Represents a material that can be applied to an Object
pub struct Material {
    color: Vector3,
}

impl Material {
    /// Creates a new Material
    pub fn new(color: Vector3) -> Material {
        Material { color }
    }
}
