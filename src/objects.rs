use crate::hit::Hit;
use crate::ray::Ray;

/// Represents a render-able object that can be hit by a ray
pub trait Object {
    /// Finds and returns the first collision of the
    /// ray with the object (if there is one)
    fn get_hit(&self, ray: &Ray) -> Option<Hit>;
}
