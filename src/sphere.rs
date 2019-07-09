use super::hitable::*;
use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<Material>,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32, material: Box<Material>) -> Box<Self> {
        Box::new(Sphere {
            center: center.clone(),
            radius,
            material,
        })
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = f32::min(
                (-b - f32::sqrt(b * b - a * c)) / a,
                (-b + f32::sqrt(b * b - a * c)) / a,
            );

            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }
}
