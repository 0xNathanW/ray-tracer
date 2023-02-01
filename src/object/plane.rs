use std::sync::Arc;
use crate::{Vec3, Point3, Matrix4};
use crate::material::Material;
use crate::ray::Ray;
use crate::object::Object;
use crate::object::Intersection;

// A plane can be defined as a point representing how far the plane is from the world's origin and a normal (defining the orientation of the plane).
// We start by defining the point as the origin and the normal as the z-axis, then we can transform this to our liking.
pub struct Plane {
    transform: Matrix4,
    inverse:   Matrix4,
    material: Arc<dyn Material>,
}

// Non-transformed plane has its origin at the world's origin and its normal is the y-axis.
impl Plane {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self {
            transform: Matrix4::identity(),
            inverse:   Matrix4::identity(),
            material,
        }
    }
}

impl Object for Plane {
    fn hit_obj(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        // Infinite solutions (div by 0).
        if ray.direction.z.abs() < 1e-6 {
            return None;
        }
        
        let t = -ray.origin.z / ray.direction.z;
        if t < t_min || t > t_max {
            None
        } else {
            let point = ray.at(t);
            Some(Intersection::new(
                self.inverse.transform_point(&point),
                self.material.clone(),
                t,
                ray,
                self.normal_at(&self.inverse.transform_point(&point)),
            ))
        }
    }
    
    fn normal_obj(&self, _point: &Point3) -> Vec3 {
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

pub struct Disk{
    transform: Matrix4,
    inverse:   Matrix4,
    material: Arc<dyn Material>,
}

// A disk is a plane with a radius.
impl Disk {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self { 
            transform: Matrix4::identity(),
            inverse:   Matrix4::identity(),
            material ,
        }
    }
}

impl Object for Disk {
    fn hit_obj(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {

        // Infinite solutions (div by 0).
        if ray.direction.z.abs() < 1e-6 {
            return None;
        }
        
        let t = -ray.origin.z / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.at(t);
        let distance = (point - Point3::origin()).magnitude();
        if distance > 1.0 {
            return None;
        }

        Some(Intersection::new(
            self.inverse.transform_point(&point),
            self.material.clone(),
            t,
            ray,
            self.normal_at(&self.inverse.transform_point(&point)),
        ))
    }

    fn normal_obj(&self, _point: &Point3) -> Vec3 {
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
