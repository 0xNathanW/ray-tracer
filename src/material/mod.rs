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
        scattered: &mut Ray
    ) -> bool;
}
