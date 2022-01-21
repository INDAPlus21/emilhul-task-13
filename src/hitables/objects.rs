use super::*;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
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