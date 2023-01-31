use crate::Transform;
use crate::object::{Object, Intersection};
use crate::ray::Ray;

#[derive(Default)]
pub struct Scene {
    pub transform: Transform,
    pub inverse:   Transform,
    pub objects: Vec<Box<dyn Object>>
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self {
            transform: Transform::identity(),
            inverse:   Transform::identity(),
            objects
        }
    }
}

impl Object for Scene {
    fn hit_world(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
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
