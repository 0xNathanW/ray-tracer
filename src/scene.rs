use crate::{Vec3, Matrix4};
use crate::object::{Object, Intersection};
use crate::ray::Ray;

#[derive(Default)]
pub struct Scene {
    pub transform: Matrix4,
    pub inverse:   Matrix4,
    pub objects: Vec<Box<dyn Object>>
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self {
            transform: Matrix4::identity(),
            inverse:   Matrix4::identity(),
            objects
        }
    }
}

impl Object for Scene {
    fn hit_obj(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            // If ray hits object before closest_so_far, update hit_record.
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit = Some(rec);
            }
        }
        hit
    }

    fn normal_obj(&self, _point: &crate::Point3) -> crate::Vec3 {
        Vec3::new(0.0, 0.0, 1.0)
    }

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
