use crate::vector3::Vector3;

/// Represents a material that can be applied to an Object
#[derive(Copy, Clone)]
pub struct Material {
    pub color: Vector3,
    pub smoothness: f64,
    pub emission_color: Vector3,
    pub emission_strength: f64,
}

impl Material {
    /// Creates a new Material
    pub const fn new(
        color: Vector3,
        smoothness: f64,
        emission_color: Vector3,
        emission_strength: f64,
    ) -> Material {
        Material {
            color,
            smoothness,
            emission_color,
            emission_strength,
        }
    }

    /// Creates a new Material with no emission
    pub const fn new_lightless(color: Vector3, smoothness: f64) -> Material {
        Material {
            color,
            smoothness,
            emission_color: Vector3::default(),
            emission_strength: 0.0,
        }
    }
}
