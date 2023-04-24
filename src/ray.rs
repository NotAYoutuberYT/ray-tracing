use crate::constants::{BRIGHTNESS, LOWER_SKY_COLOR, MAX_BOUNCES, UPPER_SKY_COLOR};
use crate::hit::Hit;
use crate::objects::Object;
use crate::random::random_unit_vector;
use crate::vector3::Vector3;
use rand::rngs::ThreadRng;
use std::sync::Arc;

/// A simple ray representing a ray of light
#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    inverse_direction: Vector3,
}

impl Ray {
    /// Creates a new ray from an origin and a direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        let direction_normalized = direction.normalized();
        Ray {
            origin,
            direction: direction_normalized,
            inverse_direction: Vector3::default() / direction_normalized,
        }
    }

    /// Samples the ray `distance` units away from the origin
    pub fn at(self, distance: f64) -> Vector3 {
        self.origin + self.direction * distance
    }

    /// Returns the origin of the ray
    pub fn origin(self) -> Vector3 {
        self.origin
    }

    /// Returns the direction of the ray
    pub fn direction(self) -> Vector3 {
        self.direction
    }

    /// Returns the inverse of the direction of the ray
    pub fn inverse_direction(self) -> Vector3 {
        self.inverse_direction
    }

    /// Gets the environment light of a ray
    fn get_environment_light(self) -> Vector3 {
        let lerp_amount: f64 = (self.direction.z() + 1.0) * 0.5;
        LOWER_SKY_COLOR.lerp(&UPPER_SKY_COLOR, lerp_amount)
    }

    /// Returns the closest valid hit for this ray
    pub fn find_first_hit(&self, objects: &[Arc<dyn Object + Send + Sync>]) -> Option<Hit> {
        // keeps track of the closest hit to the ray
        let mut closest_hit: Option<Hit> = None;

        for object in objects {
            // holds the hit of the current object
            let object_hit = object.get_hit(self);

            match object_hit {
                // if there is no new hit, do nothing
                None => (),

                // if there is a new hit, make it the new closest
                // hit if it's closer than the current closest hit
                // or there isn't any closest hit
                Some(new_hit) => match closest_hit {
                    None => closest_hit = Some(new_hit),
                    Some(current_hit) => {
                        if current_hit.distance > new_hit.distance {
                            closest_hit = Some(new_hit)
                        }
                    }
                },
            }
        }

        closest_hit
    }

    /// Returns the closest valid hit in a vector of hits
    pub fn get_best_hit(&self, hits: Vec<Hit>) -> Option<Hit> {
        // keeps track of the closest hit to the ray
        let mut closest_hit: Option<Hit> = None;

        for hit in hits {
            match closest_hit {
                None => closest_hit = Some(hit),
                Some(current_closest_hit) => {
                    if current_closest_hit.distance > hit.distance {
                        closest_hit = Some(hit)
                    }
                }
            }
        }

        closest_hit
    }

    /// Traces a vector and returns the calculated color
    pub fn trace(self, objects: &[Arc<dyn Object + Send + Sync>], rng: &mut ThreadRng) -> Vector3 {
        // variables to collect color and light of the ray
        let mut color = Vector3::new(1.0, 1.0, 1.0);
        let mut light = Vector3::default();

        // stores the current ray
        let mut ray = self;

        for _ in 0..MAX_BOUNCES {
            let optional_hit = ray.find_first_hit(objects);

            match optional_hit {
                None => {
                    light += ray.get_environment_light() * color;
                    break;
                }

                Some(hit) => {
                    // calculate diffuse direction
                    let diffuse_direction = random_unit_vector(rng) + hit.normal;

                    // calculate reflect direction
                    let reflect_direction = ray.direction.reflect_across(&hit.normal);

                    // account for smoothness
                    let new_ray_direction =
                        diffuse_direction.lerp(&reflect_direction, hit.material.smoothness);

                    // create new ray
                    ray = Ray::new(hit.point, new_ray_direction);

                    let material = hit.material;
                    let emitted_light = material.emission_color * material.emission_strength;

                    // update light and color
                    light += emitted_light * color;
                    color *= material.color * BRIGHTNESS;
                }
            }
        }

        light
    }
}
