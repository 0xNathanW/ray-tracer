use rand::prelude::*;
use crate::math::{rand_unit_vec, near_zero};
use crate::ray::Ray;
use crate::colour::Colour;
use crate::object::Intersection;
use super::Material;

pub struct Lambertian {
    // How reflective the surface is.
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self,
            _ray_in: &Ray,
            hit_record: &Intersection,
            attenuation: &mut Colour,
            scattered: &mut Ray,
            rng: &mut ThreadRng,
    ) -> bool {
        
        let mut scatter_direction = hit_record.normal + rand_unit_vec(rng).as_ref(); 
        // Don't want a zero scatter direction.
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.incidence_point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}