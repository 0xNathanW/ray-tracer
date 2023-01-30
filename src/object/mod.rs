use std::sync::Arc;
use crate::{Vec3, Point3, Rotation, Translation, Transform, Scale, Axis};
use crate::material::Material;
use crate::ray::Ray;

mod sphere;
mod plane;
mod bbox;

pub use sphere::Sphere;
pub use plane::{Plane, Disk};
pub use bbox::{AxisAlignedBoundingBox, BoundingBox};

pub struct Intersection {
    // The point at which the ray hit the object.
    pub incidence_point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,
    // Material will be shared between threads.
    pub material: Arc<dyn Material>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
}

impl Intersection {

    pub fn new(incidence_point: Point3, material: Arc<dyn Material>, t: f64) -> Self {
        Self {
            incidence_point,
            normal: Vec3::default(),
            material,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// An object is something that can be hit by a ray.
pub trait Object: Send + Sync {
    // Returns true if the ray hits the object in world space.
    fn hit_world(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;

    fn transform(&self) -> &Transform;

    fn set_transform(&mut self, transform: Transform);

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let inverse_transform = self.get_inverse_transform()?;
        let ray = ray.transform(&inverse_transform);
        self.hit_world(&ray, t_min, t_max)
    }

    fn get_inverse_transform(&self) -> Option<Transform> {
        self.transform().try_inverse()
    }

    fn rotate(&mut self, axis: Axis, angle: f64) {
        let rotation = match axis {
            Axis::X => Rotation::from_axis_angle(&Vec3::x_axis(), angle),
            Axis::Y => Rotation::from_axis_angle(&Vec3::y_axis(), angle),
            Axis::Z => Rotation::from_axis_angle(&Vec3::z_axis(), angle),
        };
        self.set_transform(self.transform() * rotation);
    }

    fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.set_transform(self.transform() * Translation::new(x, y, z));
    }

    fn scale(&mut self, x: f64, y: f64, z: f64) {
        let t = self.transform().matrix() * Scale::new(x, y, z).to_homogeneous();
        self.set_transform(Transform::from_matrix_unchecked(t));
    }

    fn scale_uniform(&mut self, scale: f64) {
        self.scale(scale, scale, scale);
    }
}