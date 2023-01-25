use std::sync::Arc;
use crate::material::Material;
use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;
use crate::object::Intersection;

// A plane can be defined as a point representing how far the plane is from the world's origin and a normal (defining the orientation of the plane).
pub struct Plane {
    point: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Arc<dyn Material>) -> Self {
        Self {
            point,
            normal,
            material,
        }
    }
}

impl Object for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let denominator = self.normal.dot(ray.direction);
        // Infinite solutions (div by 0).
        if denominator.abs() < 1e-6 {
            return None;
        }
        
        let t = (self.point - ray.origin).dot(self.normal) / denominator;
        if t < t_min || t > t_max {
            None
        } else {
            let mut intersection = Intersection::new(ray.at(t), self.material.clone(), t);
            intersection.set_face_normal(ray, self.normal);
            Some(intersection)
        }
    }
}

pub struct Disk{
    center: Point3,
    normal: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

// A disk is a plane with a radius.
impl Disk {
    pub fn new(center: Point3, normal: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            normal,
            radius,
            material,
        }
    }
}

impl Object for Disk {

    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        
        let denominator = self.normal.dot(ray.direction);
        // Infinite solutions (div by 0).
        if denominator.abs() < 1e-6 {
            return None;
        }
        
        let t = (self.center - ray.origin).dot(self.normal) / denominator;
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let distance = (point - self.center).length();
        if distance > self.radius {
            return None;
        }

        let mut intersection = Intersection::new(point, self.material.clone(), t);
        intersection.set_face_normal(ray, self.normal);
        Some(intersection)
    }

}