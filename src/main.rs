mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;
use sphere::Sphere;
use vec3::Vec3;

use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;

pub fn color(
    r: &ray::Ray,
    world: &hitable::Hitable,
    rng: &mut rand::ThreadRng,
    depth: u32,
) -> vec3::Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth < 50 {
                match rec.material.scatter(r, &rec, rng) {
                    Some(s_rec) => {
                        return s_rec.attenuation * color(&s_rec.scattered, world, rng, depth + 1);
                    }
                    _ => (),
                }
            }
            Vec3::zero()
        }
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let div = 2;
    let nx = 2000 / div;
    let ny = 1000 / div;
    let ns = 100;

    let cam = camera::Camera::new();

    let mut window = Window::new("RayTracer", nx, ny, WindowOptions::default()).unwrap();
    let mut buffer: Vec<u32> = vec![0; nx * ny];

    let world = hitable::Hitables::new(vec![
        Sphere::new(
            &Vec3::new(0.0, 0.0, -1.0),
            0.5,
            material::Lambertian::new(0.8, 0.3, 0.3),
        ),
        Sphere::new(
            &Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material::Lambertian::new(0.8, 0.8, 0.0),
        ),
        Sphere::new(
            &Vec3::new(1.0, 0.0, -1.0),
            0.5,
            material::Metal::new(0.8, 0.6, 0.2, 0.3),
        ),
        Sphere::new(
            &Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            material::Dielectric::new(1.5),
        ),
        Sphere::new(
            &Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            material::Dielectric::new(1.5),
        ),
    ]);

    for j in 0..ny {
        let line: Vec<u32> = (0..nx)
            .into_par_iter()
            .map(|i| {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                col += (0..ns)
                    .map(|_| {
                        let mut rng = rand::thread_rng();
                        let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                        let r = cam.get_ray(u, v);
                        color(&r, &world, &mut rng, 0)
                    })
                    .sum();
                col /= ns as f32;
                col = Vec3::new(f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2]));
                let ir = (255.99 * col[0]) as u32;
                let ig = (255.99 * col[1]) as u32;
                let ib = (255.99 * col[2]) as u32;
                (ir << 16) + (ig << 8) + (ib << 0)
            })
            .collect();
        let buf_start = (ny - j - 1) * nx;
        buffer[buf_start..(buf_start + nx)].clone_from_slice(&line);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
