use std::rc::Rc;

use crate::vec3::Point3;
use crate::object::{Object, HitRecord};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self { center, radius, material }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {

        let oc = ray.origin - self.center;
        // Equation to solve: t^2 * dot(B, B) + 2t * dot(B, A-C) + dot(A-C, A-C) - R^2 = 0
        // a = dot(B, B)
        let a = ray.direction.squared_length();
        // b = 2 * dot(B, A-C) .. half_b = dot(B, A-C)
        let half_b = oc.dot(ray.direction);
        // c = dot(A-C, A-C) - R^2
        let c = oc.dot(oc) - self.radius * self.radius;
        // Discriminant tells us how many roots there are.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return false; }

        // Find nearest root that t_min < root < t_max
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if (root < t_min) || (root > t_max) {
            // Try the other root.
            root = (-half_b + discriminant.sqrt()) / a;
            if (root < t_min) || (root > t_max) {
                // Both roots are outside the range.
                return false;
            }
        }

        hit_record.t = root;
        hit_record.incidence_point = ray.at(hit_record.t);
        let outward_normal = (hit_record.incidence_point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(Rc::clone(&self.material));

        true
    }
}