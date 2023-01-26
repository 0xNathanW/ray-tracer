use rand::prelude::*;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::object::Intersection;
use super::{
    Material,
    reflect,
    refract,
};

pub struct Dielectric {
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self { Self { refraction_idx } }
}

impl Dielectric {
    fn reflectance(cos: f64, refraction_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_idx) / (1.0 + refraction_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self,
        ray_in: &Ray,
        hit_record: &Intersection,
        attenuation: &mut Colour,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / self.refraction_idx } else { self.refraction_idx };
        let unit_direction = ray_in.direction.normalize();
        
        let cos_theta = ((-unit_direction).dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // If the refraction ratio is greater than the sin of the angle of incidence, then total internal reflection occurs.
        let direction = if 
            refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };
        
        *scattered = Ray::new(hit_record.incidence_point, direction);
        true
    }
}