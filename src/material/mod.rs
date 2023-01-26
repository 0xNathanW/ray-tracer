use rand::prelude::*;
use crate::Vec3;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::object::Intersection;

mod lambertian;
mod metal;
mod dielectric;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;

// Material chnages how rays interact with object surfaces.
pub trait Material: Send + Sync {    
    fn scatter(&self,
        ray_in: &Ray,
        hit_record: &Intersection,
        attenuation: &mut Colour,
        scattered: &mut Ray,
        rng: &mut ThreadRng,
    ) -> bool;
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(&normal) * normal
}

// Use Snell's law to calculate the refracted ray.
fn refract(incident: Vec3, normal: Vec3, refraction_ratio: f64) -> Vec3 {
    let cos_theta = (-incident).dot(&normal).min(1.0);
    let r_out_perp = refraction_ratio * (incident + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}