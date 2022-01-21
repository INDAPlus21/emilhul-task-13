use crate::{vector::{Vector3, Color}, hitables::{HitRecord, Hitable, scene::Scene}};

/// ## Ray
/// Representation of a ray on the form p(t) = A + tB.
/// Where A and B are 3-vectors and t is a real number (represented with f32)
#[derive(PartialEq, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    //7 ## new
    /// Returns a Ray with origin and direction given as arguments
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { 
            origin: origin,
            direction: direction
        }
    }

    /// ## point at
    /// Returns a Vector3 for the point the Ray is at given a position 
    /// parameter (t).
    pub fn point_at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }

    /// ## color
    /// Returns a Color (Vector3 type) depending on if the ray hits and how it bounces.. 
    pub fn color(ray: &Ray, scene: &Scene, depth: usize) -> Color {
        let mut hit_rec: HitRecord = HitRecord::new();
        if depth == 0 {return Vector3::new(0.0, 0.0, 0.0);}
        if scene.hit(ray, 0.001, f32::MAX, &mut hit_rec) {
            let target: Vector3 = hit_rec.p + hit_rec.normal + Vector3::random_in_unit();
            Ray::color(&Ray::new(hit_rec.p, target - hit_rec.p),  scene, depth-1) * 0.5
        } else {
            let unit_dir: Vector3 = ray.direction.unit_vec();
            let t: f32 = 0.5*(unit_dir.y + 1.0);
            Vector3::new(1.0, 1.0, 1.0) * (1.0-t) + Vector3::new(0.5, 0.7, 1.0) * t

        }
    }
}

/// Tests for Ray struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_new() {
        let a: Ray = Ray {
            origin: Vector3::new(1.0, 0.0, 0.0),
            direction: Vector3::new(-1.0, -1.0, 0.0),
        };
        let b: Ray = Ray::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0));

        assert_eq!(a, b)
    }
    
    #[test]
    fn ray_point_at() {
        let a: Vector3 = Ray::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(-1.0, -1.0, 0.0)).point_at(2.0);
        let b: Vector3 = Vector3::new(-1.0, -2.0, 0.0);

        assert_eq!(a, b);
    }
}