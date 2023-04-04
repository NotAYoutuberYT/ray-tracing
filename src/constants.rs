use crate::vector3::Vector3;

// image resolution
pub const IMAGE_WIDTH: u32 = 1200;
pub const IMAGE_HEIGHT: u32 = 675;

// viewport
pub const VIEWPORT_WIDTH: f64 = 3.5;
pub const FOCAL_LENGTH: f64 = 2.0;

// image quality
pub const MAX_BOUNCES: u32 = 10;
pub const RAYS_PER_PIXEL: u32 = 1000;
pub const ANTIALIASING_STRENGTH: f64 = 1.0;

// sky colors
pub const LOWER_SKY_COLOR: Vector3 = Vector3::new(1.0, 1.0, 1.0);
pub const UPPER_SKY_COLOR: Vector3 = Vector3::new(0.5, 0.7, 1.0);

//
// Don't manually modify these
//

pub const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

pub const VIEWPORT_HEIGHT: f64 = VIEWPORT_WIDTH / ASPECT_RATIO;
pub const VIEWPORT_WIDTH_VECTOR: Vector3 = Vector3::new(VIEWPORT_WIDTH, 0.0, 0.0);
pub const VIEWPORT_HEIGHT_VECTOR: Vector3 = Vector3::new(0.0, VIEWPORT_HEIGHT, 0.0);
