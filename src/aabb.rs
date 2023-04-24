use crate::hit::Hit;
use crate::material::Material;
use crate::objects::Object;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::vector3::Vector3;

/// Represents a single axis-aligned bounding box
pub struct Aabb {
    sides: [Plane; 6],
    center: Vector3,
    min_corner: Vector3,
    max_corner: Vector3,
    half_x_axis_span: f64,
    half_y_axis_span: f64,
    half_z_axis_span: f64,
}

impl Aabb {
    /// Creates a new axis-aligned bounding box from a
    /// center, length, width, height, and material
    pub fn new(
        center: Vector3,
        x_axis_length: f64,
        y_axis_width: f64,
        z_axis_height: f64,
        material: Material,
    ) -> Aabb {
        // because we're working from the center of the object, half the
        // dimensions will be more useful than the actual dimensions
        // (the abs makes finding max and min corners trivial)
        let half_x_span_vector = Vector3::new(x_axis_length.abs() / 2.0, 0.0, 0.0);
        let half_y_span_vector = Vector3::new(0.0, y_axis_width.abs() / 2.0, 0.0);
        let half_z_span_vector = Vector3::new(0.0, 0.0, z_axis_height.abs() / 2.0);

        // create all the planes
        let front_plane = Plane::new(
            center + half_x_span_vector,
            Vector3::new(1.0, 0.0, 0.0),
            material,
        );
        let back_plane = Plane::new(
            center - half_x_span_vector,
            Vector3::new(-1.0, 0.0, 0.0),
            material,
        );
        let right_plane = Plane::new(
            center + half_y_span_vector,
            Vector3::new(0.0, 1.0, 0.0),
            material,
        );
        let left_plane = Plane::new(
            center - half_y_span_vector,
            Vector3::new(0.0, -1.0, 0.0),
            material,
        );
        let top_plane = Plane::new(
            center + half_z_span_vector,
            Vector3::new(0.0, 0.0, 1.0),
            material,
        );
        let bottom_plane = Plane::new(
            center - half_z_span_vector,
            Vector3::new(0.0, 0.0, -1.0),
            material,
        );

        // create side array
        let sides = [
            front_plane,
            back_plane,
            right_plane,
            left_plane,
            top_plane,
            bottom_plane,
        ];

        Aabb {
            sides,
            center,
            min_corner: center - half_x_span_vector - half_y_span_vector - half_z_span_vector,
            max_corner: center + half_x_span_vector + half_y_span_vector + half_z_span_vector,
            half_x_axis_span: x_axis_length / 2.0,
            half_y_axis_span: y_axis_width / 2.0,
            half_z_axis_span: z_axis_height / 2.0,
        }
    }

    fn intersected_by_ray(&self, ray: &Ray) -> bool {
        let mut tmin: f64 = f64::NEG_INFINITY;
        let mut tmax: f64 = f64::INFINITY;

        for d in 0..3 {
            let t1 = (self.min_corner[d] - ray.origin()[d]) * ray.inverse_direction()[d];
            let t2 = (self.max_corner[d] - ray.origin()[d]) * ray.inverse_direction()[d];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmin < tmax
    }
}

impl Object for Aabb {
    fn get_hit(&self, ray: &Ray) -> Option<Hit> {
        // don't do complex calculations if no ray is intersected
        if !self.intersected_by_ray(ray) {
            return None;
        }

        // these will store the ray intersections for each plane
        let mut face_hits: Vec<Hit> = Vec::new();

        // collect the valid intersections
        for plane in self.sides {
            let plane_hit = match plane.get_hit(ray) {
                Some(hit) => hit,
                None => continue,
            };

            // verify the hit is in the box, and if it is, add it to the
            // count of valid face intersections
            let hit_distance_from_center = plane_hit.point - self.center;

            if hit_distance_from_center.x().abs() <= self.half_x_axis_span
                && hit_distance_from_center.y().abs() <= self.half_y_axis_span
                && hit_distance_from_center.z().abs() <= self.half_z_axis_span
            {
                face_hits.push(plane_hit);
            }
        }

        // get the closest hit to the ray and return it
        ray.get_best_hit(face_hits)
    }
}
