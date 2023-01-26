use rand::prelude::*;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::object::Intersection;
use crate::math::rand_in_unit_sphere;
use super::{Material, reflect};

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, f: f64) -> Self { 
        Self { 
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        } 
    }
}

impl Material for Metal {
    fn scatter(&self,ray_in: &Ray,
            hit_record: &Intersection,
            attenuation: &mut Colour,
            scattered: &mut Ray,
            rng: &mut ThreadRng,
    ) -> bool {

        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
        // Set endpoint of scattered ray to be the reflected ray plus a random vector (random point in unit sphere).
        *scattered = Ray::new(hit_record.incidence_point, reflected + self.fuzz * rand_in_unit_sphere(rng));
        *attenuation = self.albedo;
        scattered.direction.dot(&hit_record.normal) > 0.0
    }
}