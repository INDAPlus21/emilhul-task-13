use crate::{vector::*, ray::Ray};

/// ## Camera
/// Representation of a camera containing information about what is captured in the scene.
pub struct Camera {
    low_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    origin: Vector3,
}

impl Camera {
    /// ## new
    /// Returns a new Camera with standard values
    pub fn new() -> Camera {
        Camera {
            low_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
            origin: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    /// ## get_ray
    /// Returns a ray from the origin towards a direction given by how much moved in horizontal and vertical given with u respective v
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.low_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}