use std::sync::Arc;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Intersection {
    // The point at which the ray hit the object.
    pub incidence_point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,
    // Material will be shared between threads.
    pub material: Arc<dyn Material>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
}

impl Intersection {

    pub fn new(
        incidence_point: Point3,
        material: Arc<dyn Material>,
        t: f64,
    ) -> Self {
        Self {
            incidence_point,
            normal: Vec3::default(),
            material,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// An object is something that can be hit by a ray.
pub trait Object: Send + Sync {
    // Returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}