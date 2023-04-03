use crate::vector3::Vector3;

/// Represents a material that can be applied to an Object
#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector3,
    pub emission_color: Vector3,
    pub emission_strength: f64,
}

impl Material {
    /// Creates a new Material
    pub fn new(color: Vector3, emission_color: Vector3, emission_strength: f64) -> Material {
        Material {
            color,
            emission_color,
            emission_strength,
        }
    }

    /// Creates a new Material with no emission
    pub fn new_lightless(color: Vector3) -> Material {
        Material {
            color,
            emission_color: Vector3::default(),
            emission_strength: 0.0,
        }
    }
}
