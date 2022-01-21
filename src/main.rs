mod vector;
mod ray;
mod hitables;
mod camera;

use vector::*;
use ray::Ray;
use hitables::scene::Scene;
use camera::Camera;

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    //Setting up initial variables
    let width: usize = 400; //X pixel count
    let height: usize = 200; //Y pixel count
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    let mut p3: String = String::new(); //String holding ppm information
    p3.push_str(&format!("P3\n{} {}\n255\n", width, height));

    let cam: Camera = Camera::new();
    let scene: Scene = Scene::new();
    // Action

    for row in (0..height).rev() {
        for col in 0..width {
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for _sample in 0..samples_per_pixel {
                let u: f32 = (col as f32 + rand::thread_rng().gen_range(0.0..1.0)) / width as f32;
                let v: f32 = (row as f32 + rand::thread_rng().gen_range(0.0..1.0)) / height as f32;
                let ray: Ray = cam.get_ray(u, v);
                let _p = ray.point_at(2.0); // Why?
                color += Ray::color(&ray, &scene, max_depth);
            }

            color /= samples_per_pixel as f32;
            color = Vector3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt()); 
            let ir: usize = (255.99*color.x) as usize;
            let ig: usize = (255.99*color.y) as usize;
            let ib: usize = (255.99*color.z) as usize;

            p3.push_str(&format!("{} {} {}\n", ir, ig, ib))
        }
    }

    let mut file = File::create("result.ppm").expect("Failed to create file");
    writeln!(&mut file, "{}", p3).expect("Failed to write to file");
}
