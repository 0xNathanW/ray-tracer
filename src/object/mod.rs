use std::fmt::Debug;
use std::sync::Arc;
use crate::{Vec3, Point3};
use crate::material::Material;
use crate::ray::Ray;
use crate::math;
use crate::transform::Transformable;

mod sphere;
mod plane;
mod bbox;

pub use sphere::Sphere;
pub use plane::{Plane, Disk};
pub use bbox::{AxisAlignedBoundingBox, BoundingBox};

pub struct Intersection {
    // The point at which the ray hit the object.
    pub point: Point3,
    // Point hit in object space.
    pub obj_point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,
    // Material will be shared between threads.
    pub material: Arc<Material>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
    // Direction to camera.
    pub eye: Vec3,
    // Reflection vector.
    pub reflect: Vec3,
    // Point slightly above the surface.
    pub over_point: Point3,
}

impl Intersection {

    pub fn new(
        obj_point: Point3,
        material: Arc<Material>,
        t: f64,
        world_ray: &Ray,
        outward_normal: Vec3,
    ) -> Self {
        
        let point = world_ray.at(t);

        let front_face = world_ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        
        let eye = -world_ray.direction;
        
        let reflect = math::reflect(&world_ray.direction, &normal);
        
        let over_point = point + normal * 0.00001;

        Self {
            point,
            obj_point,
            normal,
            material,
            t,
            front_face,
            eye,
            reflect,
            over_point,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// An object is something that can be hit by a ray.
pub trait Object: Transformable + Send + Sync + Debug {

    fn hit_obj(&self, obj_ray: &Ray, world_ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn normal_obj(&self, point: &Point3) -> Vec3;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let obj_ray = ray.transform(self.inverse()); // Convert ray to object space.
        self.hit_obj(&obj_ray, &ray, t_min, t_max)
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        let obj_point = self.inverse().transform_point(point);
        let obj_normal = self.normal_obj(&obj_point);
        let world_normal = self.inverse().transpose() * obj_normal.to_homogeneous();
        let world_normal = Vec3::new(world_normal.x, world_normal.y, world_normal.z);
        world_normal.normalize()
    }
}