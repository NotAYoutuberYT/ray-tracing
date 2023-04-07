use crate::hit::Hit;
use crate::material::Material;
use crate::objects::Object;
use crate::ray::Ray;
use crate::vector3::Vector3;

/// Represents a 2D plane that goes of infinitely in every direction
#[derive(Copy, Clone)]
pub struct Plane {
    point: Vector3,
    normal: Vector3,
    material: Material,
}

impl Plane {
    pub fn new(point: Vector3, normal: Vector3, material: Material) -> Plane {
        Plane {
            point,
            normal: normal.normalized(),
            material,
        }
    }
}

impl Object for Plane {
    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        // numerator and denominator of point on ray of intersection
        let numerator = self.normal.dot(&(self.point - ray.origin()));
        let denominator = self.normal.dot(&ray.direction());

        // no solutions (ray is parallel to plane)
        if denominator.abs() <= f64::EPSILON {
            return None;
        }

        // find distance of hit
        let distance = numerator / denominator;

        // don't take hits behind or too close to the ray (epsilon * 120
        // seems to be the sweet spot for avoiding artifacts)
        if distance <= f64::EPSILON * 120.0 {
            return None;
        }

        let (true_normal, outside_face) = match self.normal.dot(&ray.direction()) <= 0.0 {
            true => (self.normal, false),
            false => (-self.normal, true),
        };

        Some(Hit {
            distance,
            point: ray.at(distance),
            normal: true_normal,
            outside_face,
            material: self.material,
        })
    }
}
