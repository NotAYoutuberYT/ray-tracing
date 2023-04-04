use crate::vector3::Vector3;
use std::f64::consts::PI;
use std::ops;

/// Represents a rotation in 3D space
#[derive(Copy, Clone)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Quaternion {
    /// Creates a new Quaternion from a direction vector
    /// and a rotation around that vector
    pub fn new(direction: Vector3, rotation_degrees: f64) -> Quaternion {
        // degree to quaternion constant divided by 2
        let angle_constant: f64 = PI / 360.0;

        Quaternion {
            w: (rotation_degrees * angle_constant).cos(),
            x: (rotation_degrees * angle_constant).sin() * direction.x(),
            y: (rotation_degrees * angle_constant).sin() * direction.y(),
            z: (rotation_degrees * angle_constant).sin() * direction.z(),
        }
    }

    /// Creates a new Quaternion directly from w, x, y, and z components
    /// (useful for hamiltonian products)
    pub fn new_raw(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, x, y, z }
    }

    /// Returns the w component of the Quaternion
    pub fn w(self) -> f64 {
        self.w
    }

    /// Returns the x component of the Quaternion
    pub fn x(self) -> f64 {
        self.x
    }

    /// Returns the y component of the Quaternion
    pub fn y(self) -> f64 {
        self.y
    }

    /// Returns the z component of the Quaternion
    pub fn z(self) -> f64 {
        self.z
    }
    
    /// Negates the x, y, and z components of the Quaternion
    pub fn negated_vector(self) -> Quaternion {
        Quaternion::new_raw(self.w, -self.x, -self.y, -self.z)
    }

    /// Computes the hamiltonian product between two Quaternions
    pub fn hamiltonian_product(self, other: &Quaternion) -> Quaternion {
        Quaternion::new_raw(
            self.w * other.w() - self.x * other.x() - self.y * other.y() - self.z * other.z(),
            self.w * other.x() + self.x * other.w + self.y * other.z() - self.z * other.y(),
            self.w * other.y() - self.x * other.z() + self.y * other.w() + self.z * other.x(),
            self.w * other.z() + self.x * other.y() - self.y * other.x() + self.z * other.w(),
        )
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        self.hamiltonian_product(&rhs)
    }
}

impl ops::MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        *self = *self * rhs
    }
}