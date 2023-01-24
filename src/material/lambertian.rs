use crate::ray::Ray;
use crate::colour::Colour;
use crate::object::Intersection;
use crate::vec3::Vec3;
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
    ) -> bool {
        
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        // Don't want a zero scatter direction.
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.incidence_point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}