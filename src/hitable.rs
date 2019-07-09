use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Hitables {
    list: Vec<Box<Hitable>>,
}

impl Hitables {
    pub fn new(list: Vec<Box<Hitable>>) -> Hitables {
        Hitables { list }
    }
}

impl Hitable for Hitables {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (_, res) = self
            .list
            .iter()
            .fold((t_max, None), |(closest_so_far, rec), h| {
                match h.hit(r, t_min, closest_so_far) {
                    Some(new_rec) => (new_rec.t, Some(new_rec)),
                    None => (closest_so_far, rec),
                }
            });
        res
    }
}
