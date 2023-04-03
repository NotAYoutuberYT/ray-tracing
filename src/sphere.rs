use crate::hit::Hit;
use crate::material::Material;
use crate::objects::Object;
use crate::ray::Ray;
use crate::vector3::Vector3;

/// A basic sphere
pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: Material,
}

impl Sphere {
    /// Creates a new sphere from a center, radius, and material
    pub fn new(center: Vector3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn get_hit(self, ray: &Ray) -> Option<Hit> {
        // get the origin of the ray relative to if this sphere's center
        let relative_ray_origin = ray.origin() - self.center;

        // get the the coefficients for the quadratic equation that needs
        // to be solved to get the distance for the hit
        let quadratic_a = ray.direction().length_squared();
        let half_quadratic_b = relative_ray_origin.dot(&ray.direction());
        let quadratic_c = relative_ray_origin.length_squared() - self.radius * self.radius;

        // solve for the inside of the square root in the quadratic formula
        let quadratic_root = half_quadratic_b * half_quadratic_b - quadratic_a * quadratic_c;

        // if there are no solutions, return None
        if quadratic_root < 0.0 {
            return None;
        }

        // calculate the nearest hit distance from the quadratic formula
        let distance = -half_quadratic_b - quadratic_root.sqrt() / quadratic_a;

        // extrapolate the point of the hit, the normal vector, and if
        // the hit was from the outside of the object or not
        let hit_point = ray.at(distance);
        let normal_vector = (hit_point - self.center) / self.radius;
        let outside_hit = normal_vector.dot(&ray.direction()) <= 0.0;

        Some(Hit::new(distance, hit_point, normal_vector, outside_hit))
    }

    fn material(self) -> Material {
        self.material
    }
}
