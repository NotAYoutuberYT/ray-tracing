use crate::aabb::AABB;
use crate::hit::Hit;
use crate::material::Material;
use crate::objects::Object;
use crate::quaternion::Quaternion;
use crate::ray::Ray;
use crate::vector3::Vector3;

/// Represents a rectangular box
pub struct BoxObject {
    // this box works by having a "fake" aabb on the
    // origin. in order to find ray hits, it shifts
    // the ray to match what the scene would look
    // like if the box were properly setup in the
    // first place relative to the ray
    adjusted_aabb: AABB,
    rotation: Quaternion,
    rotation_inverse: Quaternion,
    true_center: Vector3,
}

impl BoxObject {
    /// Creates a new box from a center, length, width, height, rotation, and material
    pub fn new(
        center: Vector3,
        x_axis_length: f64,
        y_axis_width: f64,
        z_axis_height: f64,
        rotation: Quaternion,
        material: Material,
    ) -> BoxObject {
        BoxObject {
            adjusted_aabb: AABB::new(
                Vector3::default(),
                x_axis_length,
                y_axis_width,
                z_axis_height,
                material,
            ),
            rotation,
            rotation_inverse: rotation.inverse(),
            true_center: center,
        }
    }
}

impl Object for BoxObject {
    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        // rotate the ray by the opposite rotation of
        // this box to emulate the box being rotated
        let adjusted_ray = Ray::new(
            (ray.origin() - self.true_center).rotate_by(self.rotation_inverse),
            ray.direction().rotate_by(self.rotation_inverse),
        );

        // find the hit with the adjusted ray in the
        // correct relative position and return it
        let offset_hit = self.adjusted_aabb.get_hit(&adjusted_ray);

        // correct the hit and return it (assuming it exists)
        offset_hit.map(|hit| {
            Hit::new(
                hit.distance,
                ray.at(hit.distance),
                hit.normal.rotate_by(self.rotation),
                hit.outside_face,
                hit.material,
            )
        })
    }
}
