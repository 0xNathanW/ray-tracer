use std::sync::Arc;
use crate::transform::Transformable;
use crate::{Point3, Matrix4, Vec3};
use crate::object::Object;
use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    id:         usize,
    transform:  Matrix4,
    inverse:    Matrix4,
    material:   Arc<Material>,
}

impl Sphere {
    pub fn new(material: Material) -> Self {
        Self { 
            id:        0,
            transform: Matrix4::identity(), 
            inverse:   Matrix4::identity(),            
            material:  Arc::new(material),
        }
    }
}

impl Object for Sphere {

    fn hit_obj(
        &self, 
        obj_ray: &Ray, 
        t_min: f64, 
        t_max: f64
    ) -> Option<Vec<f64>> {

        let oc = obj_ray.origin - Point3::origin();
        // Equation to solve: t^2 * dot(B, B) + 2t * dot(B, A-C) + dot(A-C, A-C) - R^2 = 0
        // a = dot(B, B)
        let a = obj_ray.direction.magnitude_squared();
        // b = 2 * dot(B, A-C) .. half_b = dot(B, A-C)
        let half_b = oc.dot(&obj_ray.direction);
        // c = dot(A-C, A-C) - R^2
        let c = oc.dot(&oc) - 1.0;
        // Discriminant tells us how many roots there are.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        let mut close_root = (-half_b - discriminant.sqrt()) / a;
        let mut far_root = (-half_b + discriminant.sqrt()) / a;
        if close_root > far_root {
            std::mem::swap(&mut close_root, &mut far_root);
        }

        let mut t = vec![];
        if close_root < t_max && close_root > t_min {
            t.push(close_root);
        }
        if far_root < t_max && far_root > t_min {
            t.push(far_root);
        }

        if t.is_empty() { None } else { Some(t) }
    }

    fn normal_obj(&self, point: &Point3) -> Vec3 {
        (point - Point3::origin()).normalize()
    }

    fn material(&self) -> &Arc<Material> {
        &self.material
    }

    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Transformable for Sphere {

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }
}
