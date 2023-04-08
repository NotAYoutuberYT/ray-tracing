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
    pub fn new(axis: Vector3, angle_degrees: f64) -> Quaternion {
        // axis must be normalized
        let axis_normalized = axis.normalized();

        // degree to quaternion constant divided by 2
        let angle_constant: f64 = PI / 360.0;

        Quaternion {
            w: (angle_degrees * angle_constant).cos(),
            x: (angle_degrees * angle_constant).sin() * axis_normalized.x(),
            y: (angle_degrees * angle_constant).sin() * axis_normalized.y(),
            z: (angle_degrees * angle_constant).sin() * axis_normalized.z(),
        }
    }

    /// Creates a new Quaternion from a direction to point in
    /// and a rotation about that vector
    pub fn new_from_angles(roll_degrees: f64, pitch_degrees: f64, yaw_degrees: f64) -> Quaternion {
        // converts angles to degrees then divides by two
        let angle_constant = PI / 360.0;

        // who knows what these are I got this function from
        // https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
        let cr = (roll_degrees * angle_constant).cos();
        let sr = (roll_degrees * angle_constant).sin();
        let cp = (pitch_degrees * angle_constant).cos();
        let sp = (pitch_degrees * angle_constant).sin();
        let cy = (yaw_degrees * angle_constant).cos();
        let sy = (yaw_degrees * angle_constant).sin();

        Quaternion::new_raw(
            cr * cp * cy + sr * sp * sy,
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
        )
    }

    /// Creates a new Quaternion directly from w, x, y, and z components
    /// (useful for hamiltonian products)
    pub fn new_raw(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, x, y, z }
    }

    /// Returns a Quaternion facing directly forwards with no rotation
    pub fn default() -> Quaternion {
        Quaternion::new(Vector3::new(0.0, 0.0, 1.0), 0.0)
    }

    /// Returns the scalar component of the Quaternion
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

    /// Returns the conjugate of the Quaternion
    pub fn conjugate(self) -> Quaternion {
        Quaternion::new_raw(self.w, -self.x, -self.y, -self.z)
    }

    /// Returns the magnitude of the Quaternion squared
    /// (Note: this is much faster than actual magnitude)
    pub fn magnitude_squared(self) -> f64 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the magnitude of the Quaternion
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Returns the inverse of the Quaternion
    pub fn inverse(self) -> Quaternion {
        self.conjugate() / self.magnitude_squared()
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

impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion::new_raw(self.w * rhs, self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
