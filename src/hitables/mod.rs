use crate::ray::Ray;
use crate::vector::Vector3;

pub mod objects;
use objects::Sphere;
pub mod scene;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vector3,
    pub normal: Vector3,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool;
}
