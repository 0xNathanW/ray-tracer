use crate::vec3::{Colour, Vec3};
use crate::ray::Ray;
use crate::object::HitRecord;

// Material chnages how rays interact with object surfaces.
pub trait Material {
    fn scatter(&self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray
    ) -> bool;
}

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
            hit_record: &HitRecord,
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
            hit_record: &HitRecord,
            attenuation: &mut Colour,
            scattered: &mut Ray
    ) -> bool {

        let reflected = Vec3::reflect(ray_in.direction.normalise(), hit_record.normal);
        // Set endpoint of scattered ray to be the reflected ray plus a random vector (random point in unit sphere).
        *scattered = Ray::new(hit_record.incidence_point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        scattered.direction.dot(hit_record.normal) > 0.0
    }
}

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
            hit_record: &HitRecord,
            attenuation: &mut Colour,
            scattered: &mut Ray
    ) -> bool {

        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face { 1.0 / self.refraction_idx } else { self.refraction_idx };
        let unit_direction = ray_in.direction.normalise();
        
        let cos_theta = ((-unit_direction).dot(hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // If the refraction ratio is greater than the sin of the angle of incidence, then total internal reflection occurs.
        let direction = if 
            refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };
        
        *scattered = Ray::new(hit_record.incidence_point, direction);
        true
    }
}