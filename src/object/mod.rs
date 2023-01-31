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

    fn inverse(&self) -> &Transform;

    fn set_transform(&mut self, transform: Transform);

    // Inversion rule: (A * B)^-1 = B^-1 * A^-1
    fn set_inverse(&mut self, inverse: Transform);

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let inv_ray = ray.transform(self.inverse()); // Convert ray to object space.
        self.hit_world(&inv_ray, t_min, t_max)
    }

    fn rotate(&mut self, axis: Axis, angle: f64) {
        let rotation = match axis {
            Axis::X => Rotation::from_axis_angle(&Vec3::x_axis(), angle),
            Axis::Y => Rotation::from_axis_angle(&Vec3::y_axis(), angle),
            Axis::Z => Rotation::from_axis_angle(&Vec3::z_axis(), angle),
        };
        self.set_transform(self.transform() * rotation);
        self.set_inverse(rotation.inverse() * self.inverse());
    }

    fn translate(&mut self, x: f64, y: f64, z: f64) {
        let translation = Translation::new(x, y, z);
        self.set_transform(self.transform() * translation);
        self.set_inverse(translation.inverse() * self.inverse());
    }

    fn scale(&mut self, x: f64, y: f64, z: f64) {
        let scale = Scale::new(x, y, z).to_homogeneous();
        let inv = scale.try_inverse().expect("Scale matrix is not invertible.");

        let new_transform = self.transform().matrix() * scale;
        self.set_transform(Transform::from_matrix_unchecked(new_transform));
        
        let new_inverse = inv * self.inverse().matrix();
        self.set_inverse(Transform::from_matrix_unchecked(new_inverse));
    }

    fn scale_uniform(&mut self, scale: f64) {
        self.scale(scale, scale, scale);
    }
}