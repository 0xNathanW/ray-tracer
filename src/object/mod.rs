use std::fmt::Debug;
use std::sync::Arc;
use crate::{Vec3, Point3, Colour};
use crate::material::Material;
use crate::ray::Ray;
use crate::math::reflect;
use crate::transform::Transformable;

mod sphere;
mod plane;
mod bbox;

pub use sphere::Sphere;
pub use plane::{Plane, Disk};
pub use bbox::{AxisAlignedBoundingBox, BoundingBox};

#[derive(Debug, Default)]
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
    // Point slightly above the surface.
    pub over_point: Point3,
    // Colour of material/pattern.
    pub colour: Colour,
}

// An object is something that can be hit by a ray.
pub trait Object: Transformable + Send + Sync + Debug {

    // Returns the point on ray at t if the ray hits the object.
    fn hit_obj(&self, obj_ray: &Ray, t_min: f64, t_max: f64) -> Option<Vec<f64>>;
    
    fn normal_obj(&self, point: &Point3) -> Vec3;
    
    fn material(&self) -> &Arc<Material>;

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Vec<Intersection>> {
        
        let obj_ray = ray.transform(self.inverse()); // Convert ray to object space.
        let hits = self.hit_obj(&obj_ray, t_min, t_max);
        
        if let Some(hits) = hits {
            let mut intersections = Vec::new();

            for t in hits {

                let point = ray.at(t);
                let outward_normal = self.normal_at(&point);
                let eye = -ray.direction;
                let front_face = ray.direction.dot(&outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { -outward_normal };
                let reflect = reflect(&ray.direction, &normal);
                let over_point = point + normal * 0.0001;
                let colour = self.material().colour_at(&point, self.inverse());
                
                intersections.push(Intersection {
                    point,
                    normal,
                    material: self.material().clone(),
                    t,
                    front_face,
                    eye,
                    reflect,
                    over_point,
                    colour,
                });
            }
            Some(intersections)
        } else {
            None
        }
    }

    fn normal_at(&self, point: &Point3) -> Vec3 {
        let obj_point = self.inverse().transform_point(point);
        let obj_normal = self.normal_obj(&obj_point);
        let world_normal = self.inverse().transpose() * obj_normal.to_homogeneous();
        let world_normal = Vec3::new(world_normal.x, world_normal.y, world_normal.z);
        world_normal.normalize()
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Plane;
    use crate::math::*;

    #[test]
    fn test_hit() {

        let plane = Plane::new(Material::default());
        let ray = Ray::new(Point3::new(0.0, 1.0, -1.0), Vec3::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0));
        let ints = plane.hit(&ray, 0.0, f64::INFINITY).unwrap();
        let int = &ints[0];
        assert_eq!(int.point, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(int.normal, Vec3::new(0.0, 1.0, 0.0));
        assert!(fuzzy_eq_vec(&int.reflect, &Vec3::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)));
    }

}