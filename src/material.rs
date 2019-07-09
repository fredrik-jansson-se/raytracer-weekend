use rand::Rng;

use super::hitable::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut rand::ThreadRng)
        -> Option<ScatterRec>;
}

fn random_in_unit_sphere(rng: &mut rand::ThreadRng) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: f32, b: f32, c: f32) -> Box<Material> {
        Box::new(Lambertian {
            albedo: Vec3::new(a, b, c),
        })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::ThreadRng,
    ) -> Option<ScatterRec> {
        let target = rec.p + rec.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(&rec.p, &(target - rec.p));
        Some(ScatterRec {
            attenuation: self.albedo.clone(),
            scattered,
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(a: f32, b: f32, c: f32, fuzz: f32) -> Box<Material> {
        Box::new(Metal {
            albedo: Vec3::new(a, b, c),
            fuzz,
        })
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * n
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::ThreadRng,
    ) -> Option<ScatterRec> {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(reflected + self.fuzz * random_in_unit_sphere(rng)),
        );
        Some(ScatterRec {
            attenuation: self.albedo.clone(),
            scattered,
        })
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - f32::sqrt(discriminant) * n)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Box<Material> {
        Box::new(Dielectric { ref_idx })
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        rng: &mut rand::ThreadRng,
    ) -> Option<ScatterRec> {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(&rec.normal) > 0.0 {
            (
                -rec.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction().dot(&rec.normal) / r_in.direction().length(),
            )
        } else {
            (
                rec.normal,
                1.0 / self.ref_idx,
                -r_in.direction().dot(&rec.normal) / r_in.direction().length(),
            )
        };

        let (s_rec_1, reflect_prob) = match refract(&r_in.direction(), &outward_normal, ni_over_nt)
        {
            Some(refracted) => (
                ScatterRec {
                    attenuation,
                    scattered: Ray::new(&rec.p, &refracted),
                },
                schlick(cosine, self.ref_idx),
            ),
            None => (
                ScatterRec {
                    attenuation,
                    scattered: Ray::new(&rec.p, &reflected),
                },
                1.0,
            ),
        };

        if rng.gen::<f32>() < reflect_prob {
            Some(ScatterRec {
                attenuation: s_rec_1.attenuation,
                scattered: Ray::new(&rec.p, &reflected),
            })
        } else {
            Some(s_rec_1)
        }
    }
}
