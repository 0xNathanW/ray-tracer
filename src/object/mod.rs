use std::sync::Arc;
use crate::{Vec3, Point3, Rotation, Translation, Scale, Axis, Matrix4};
use crate::material::Material;
use crate::ray::Ray;
use crate::math;

mod sphere;
mod plane;
mod bbox;

pub use sphere::Sphere;
pub use plane::{Plane, Disk};
pub use bbox::{AxisAlignedBoundingBox, BoundingBox};

pub struct Intersection {
    // The point at which the ray hit the object.
    pub point: Point3,
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
    pub over_point: Point3,
}

impl Intersection {

    pub fn new(
        point: Point3, 
        material: Arc<Material>,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
    ) -> Self {
        
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        let eye = -ray.direction;
        let reflect = math::reflect(&ray.direction, &normal);
        let over_point = point + normal * 0.00001;

        Self {
            point,
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
pub trait Object: Send + Sync {

    fn hit_obj(&self, obj_ray: &Ray, world_ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn normal_obj(&self, point: &Point3) -> Vec3;

    fn transform(&self) -> &Matrix4;

    fn inverse(&self) -> &Matrix4;

    fn set_transform(&mut self, transform: Matrix4);

    // Inversion rule: (A * B)^-1 = B^-1 * A^-1
    fn set_inverse(&mut self, inverse: Matrix4);

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

    fn rotate(&mut self, axis: Axis, angle: f64) {
        let rotation = match axis {
            Axis::X => Rotation::from_axis_angle(&Vec3::x_axis(), angle),
            Axis::Y => Rotation::from_axis_angle(&Vec3::y_axis(), angle),
            Axis::Z => Rotation::from_axis_angle(&Vec3::z_axis(), angle),
        }.to_homogeneous();
        
        let inv = rotation.try_inverse().expect("Rotation matrix is not invertible.");
        self.set_transform(self.transform() * rotation);
        self.set_inverse(inv * self.inverse());
    }

    fn translate(&mut self, x: f64, y: f64, z: f64) {
        let translation = Translation::new(x, y, z).to_homogeneous();
        self.set_transform(self.transform() * translation);
        
        let inv = translation.try_inverse().expect("Translation matrix is not invertible.");
        self.set_inverse(inv * self.inverse());
    }

    fn scale(&mut self, x: f64, y: f64, z: f64) {
        let scale = Scale::new(x, y, z).to_homogeneous();
        let inv = scale.try_inverse().expect("Scale matrix is not invertible.");

        self.set_transform(self.transform() * scale);
        self.set_inverse(inv * self.inverse());
    }

    fn scale_uniform(&mut self, scale: f64) {
        self.scale(scale, scale, scale);
    }
}