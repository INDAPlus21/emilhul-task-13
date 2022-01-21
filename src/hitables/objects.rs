use super::*;

/// ## Sphere
/// A representation of a Sphere with a center in a position given by a Vector3 and a radius given as a f32.
#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    /// ## new
    /// Return a Sphere where it's center and radius is given
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    /// ## hit
    /// Checks wheter a given Ray hits the sphere.
    /// If it hits store information regarding that in the HitRecord.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool {
        let oc: Vector3 = ray.origin - self.center;
        let a: f32 = ray.direction.dot(ray.direction);
        let b: f32 = oc.dot(ray.direction);
        let c: f32 = oc.dot(oc) - self.radius*self.radius;
        let discriminant: f32 = b*b - a*c;

        if discriminant > 0.0 {
            let mut temp: f32 = (-b - discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                hit_rec.t = temp;
                hit_rec.p = ray.point_at(temp);
                hit_rec.normal = (hit_rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if t_min < temp && temp < t_max {
                hit_rec.t = temp;
                hit_rec.p = ray.point_at(temp);
                hit_rec.normal = (hit_rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}