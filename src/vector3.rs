use std::ops;

/// A basic vector3 that can represent actual
/// vectors, points, or colors
#[derive(Copy, Clone)]
pub struct Vector3 {
    terms: [f64; 3],
}

impl Vector3 {
    /// Returns <0, 0, 0>
    pub fn default() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    /// Creates a new Vector3 from the 3 given terms
    pub const fn new(term1: f64, term2: f64, term3: f64) -> Vector3 {
        Vector3 {
            terms: [term1, term2, term3],
        }
    }

    /// Returns the x-component of the vector
    pub fn x(self) -> f64 {
        self[0]
    }

    /// Returns the y-component of the vector
    pub fn y(self) -> f64 {
        self[1]
    }

    /// Returns the z-component of the vector
    pub fn z(self) -> f64 {
        self[2]
    }

    /// Returns the normalized version of a vector
    pub fn normalized(self) -> Vector3 {
        self / self.length()
    }

    /// Sets a vector to its normalized version
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Gets the length of the vector squared (significantly
    /// faster than getting the exact length)
    pub fn length_squared(self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    /// Gets the length of the vector (use length_squared
    /// if possible, as this is very slow
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product of two vectors
    pub fn dot(self, other: &Vector3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    /// Returns the cross product of two vectors
    pub fn cross(self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }
}

//
// Operator overloading
//

impl ops::Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

impl ops::IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.terms[index]
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self[0], -self[1], -self[2])
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = *self - rhs;
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}

impl ops::Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self[0] / rhs[0], self[1] / rhs[1], self[2] / rhs[2])
    }
}

impl ops::DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        *self = *self / rhs;
    }
}

impl ops::Div<Vector3> for f64 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        rhs / self
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
