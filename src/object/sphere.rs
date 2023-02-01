use std::sync::Arc;
use crate::{Point3, Matrix4, Vec3};
use crate::object::{Object, Intersection};
use crate::material::Material;
use crate::ray::Ray;

pub struct Sphere {
    transform: Matrix4,
    inverse:   Matrix4,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self { 
            transform: Matrix4::identity(), 
            inverse:   Matrix4::identity(),            
            material,
        }
    }
}

impl Object for Sphere {

    fn hit_obj(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {

        let oc = ray.origin - Point3::origin();
        // Equation to solve: t^2 * dot(B, B) + 2t * dot(B, A-C) + dot(A-C, A-C) - R^2 = 0
        // a = dot(B, B)
        let a = ray.direction.magnitude_squared();
        // b = 2 * dot(B, A-C) .. half_b = dot(B, A-C)
        let half_b = oc.dot(&ray.direction);
        // c = dot(A-C, A-C) - R^2
        let c = oc.dot(&oc) - 1.0;
        // Discriminant tells us how many roots there are.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        // Find nearest root that t_min < root < t_max
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if (root < t_min) || (root > t_max) {
            // Try the other root.
            root = (-half_b + discriminant.sqrt()) / a;
            if (root < t_min) || (root > t_max) {
                // Both roots are outside the range.
                return None;
            }
        }

        let incidence_point = self.inverse.transform_point(&ray.at(root));
        let mut hit = Intersection::new(
            incidence_point,
            self.material.clone(),
            root,
            ray,
            self.normal_at(&incidence_point),
        );
        let outward_normal = hit.incidence_point - Point3::origin();
        hit.set_face_normal(ray, outward_normal);
        Some(hit)
    }

    fn normal_obj(&self, point: &Point3) -> Vec3 {
        (point - Point3::origin()).normalize()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::colour::Colour;
    use crate::{Vec3, Axis};
    use crate::math::fuzzy_eq;

    #[test]
    fn test_sphere_new() {
        let material = Arc::new(Lambertian::new(Colour::new(0.0, 0.0, 0.0)));
        let sphere = Sphere::new(material);
        assert_eq!(sphere.transform, Matrix4::identity());
        assert_eq!(sphere.inverse, Matrix4::identity());
    }

    #[test]
    fn test_sphere_normal_obj() {
        let sphere = Sphere::new(Arc::new(Lambertian::new(Colour::new(0.0, 0.0, 0.0))));
        let normal = sphere.normal_obj(&Point3::new(1.0, 0.0, 0.0));
        assert_eq!(normal, Vec3::new(1.0, 0.0, 0.0));

        let normal = sphere.normal_obj(&Point3::new(0.0, 1.0, 0.0));
        assert_eq!(normal, Vec3::new(0.0, 1.0, 0.0));

        let normal = sphere.normal_obj(&Point3::new(0.0, 0.0, 1.0));
        assert_eq!(normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_sphere_normal() {
        let mut sphere = Sphere::new(Arc::new(Lambertian::new(Colour::new(0.0, 0.0, 0.0))));
        sphere.translate(0.0, 1.0, 0.0);
        let n = sphere.normal_at(&Point3::new(0.0, 1.70711, -0.70711));
        assert!(fuzzy_eq(&n, &Vec3::new(0.0, 0.70711, -0.70711)));
    
        let mut sphere1 = Sphere::new(Arc::new(Lambertian::new(Colour::new(0.0, 0.0, 0.0))));
        sphere1.scale(1.0, 0.5, 1.0);
        sphere1.rotate(Axis::Z, std::f64::consts::PI / 5.0);
        let n = sphere1.normal_at(&Point3::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert!(fuzzy_eq(&n, &Vec3::new(0.0, 0.97014, -0.24254)));
    }

    #[test]
    fn test_sphere_hit_obj() {
        let material = Arc::new(Lambertian::new(Colour::new(0.0, 0.0, 0.0)));
        let sphere = Sphere::new(material);
        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.hit_obj(&ray, 0.0, std::f64::MAX);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert_eq!(hit.incidence_point, Point3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(hit.t, 4.0);
    }
}