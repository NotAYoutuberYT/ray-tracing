use crate::vector3::Vector3;

// camera info
pub const IMAGE_WIDTH: u32 = 4096;
pub const IMAGE_HEIGHT: u32 = 2160;
pub const HORIZONTAL_FOV_DEGREES: f64 = 80.0;

// image quality
pub const MAX_BOUNCES: u32 = 12;
pub const RAYS_PER_PIXEL: u32 = 750;
pub const ANTIALIASING_STRENGTH: f64 = 1.0;

// sky colors
pub const LOWER_SKY_COLOR: Vector3 = Vector3::new(1.0, 1.0, 1.0);
pub const UPPER_SKY_COLOR: Vector3 = Vector3::new(0.5, 0.7, 1.0);

// image settings
pub const BRIGHTNESS: f64 = 1.7;

//
// Don't manually modify these
//

pub const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
