use std::rc::Rc;
use crate::material::Material;
use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct HitRecord {
    // The point at which the ray hit the object.
    pub incidence_point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,

    pub material: Option<Rc<dyn Material>>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// An object is something that can be hit by a ray.
pub trait Object {
    // Returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}