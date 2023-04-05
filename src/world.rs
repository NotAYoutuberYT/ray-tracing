use crate::camera::Camera;
use crate::constants::{ASPECT_RATIO, HORIZONTAL_FOV_DEGREES};
use crate::material::Material;
use crate::objects::Object;
use crate::quaternion::Quaternion;
use crate::sphere::Sphere;
use crate::vector3::Vector3;
use crate::vector3::Vector3 as Color;

pub const OBJECTS: [&dyn Object; 3] = [
    &Sphere::new(
        Vector3::new(00.0, 0.0, 2.0),
        1.25,
        Material::new(
            Color::new(0.0, 0.0, 0.0),
            0.0,
            Color::new(0.93, 0.95, 0.2),
            10.0,
        ),
    ),
    &Sphere::new(
        Vector3::new(0.0, -7.0, 0.0),
        4.0,
        Material::new_lightless(Color::new(0.8, 0.45, 0.45), 1.0),
    ),
    &Sphere::new(
        Vector3::new(0.0, 7.0, 0.0),
        4.0,
        Material::new_lightless(Color::new(0.45, 0.45, 0.8), 1.0),
    ),
];

pub fn get_camera() -> Camera {
    Camera::new(
        Vector3::new(-11.0, -10.0, 0.0),
        Quaternion::new_from_angles(0.0, 0.0, 60.0),
        HORIZONTAL_FOV_DEGREES,
        ASPECT_RATIO,
    )
}
