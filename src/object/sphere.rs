use std::sync::Arc;
use crate::{Point3, Transform};
use crate::object::{Object, Intersection};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    transform: Transform,
    inverse:   Transform,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self { 
            transform: Transform::identity(), 
            inverse:   Transform::identity(),            
            material 
        }
    }
}

impl Object for Sphere {
    fn hit_world(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {

        let oc = ray.origin - Point3::origin();
        // Equation to solve: t^2 * dot(B, B) + 2t * dot(B, A-C) + dot(A-C, A-C) - R^2 = 0
        // a = dot(B, B)
        let a = ray.direction.magnitude_squared();
        // b = 2 * dot(B, A-C) .. half_b = dot(B, A-C)
        let half_b = oc.dot(&ray.direction);
        // c = dot(A-C, A-C) - R^2
        let c = oc.dot(&oc) - 1.0;
        // Discriminant tells us how many roots there are.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        // Find nearest root that t_min < root < t_max
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if (root < t_min) || (root > t_max) {
            // Try the other root.
            root = (-half_b + discriminant.sqrt()) / a;
            if (root < t_min) || (root > t_max) {
                // Both roots are outside the range.
                return None;
            }
        }

        let mut hit = Intersection::new(
            ray.at(root),
            self.material.clone(),
            root,
        );
        let outward_normal = hit.incidence_point - Point3::origin();
        hit.set_face_normal(ray, outward_normal);
        Some(hit)
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Transform) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn inverse(&self) -> &Transform {
        &self.inverse
    }
}