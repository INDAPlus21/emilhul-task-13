use super::*;

/// ## Scene
/// Reptesentation of the scene. 
/// Contains a list of all hitable objects in the scene.
pub struct Scene {
    pub object_list: Vec<Box<dyn Hitable>>
}

impl Scene {
    /// ## new
    /// Creates a new scene with standard values.
    pub fn new() -> Scene {
        Scene {
            object_list: vec![
                Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
                Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
                ],
        }
    }
}

impl Hitable for Scene {
    /// ## hit
    /// Goes through all objects in the scene and cheks wheter they are hit by a given ray.
    /// If it hits a object store information regarding that in HitRecord
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_yet: f32 = t_max;

        for object in self.object_list.iter() {
            if object.hit(ray, t_min, closest_yet, &mut temp_rec) {
                hit_anything = true;
                closest_yet = temp_rec.t;
                *hit_rec = temp_rec;
            }
        }
        return hit_anything;
    }
}