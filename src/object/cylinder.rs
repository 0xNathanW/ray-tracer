use std::sync::Arc;
use crate::{Matrix4, Material, Object, ray::Ray, Vec3, Point3};
use crate::transform::Transformable;

#[derive(Debug)]
pub struct Cylinder{
    pub id: usize,
    pub min: f64,
    pub max: f64,
    pub capped: bool,
    pub transform: Matrix4,
    pub inverse: Matrix4,
    pub material: Arc<Material>,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new(Material::default(), -f64::INFINITY, f64::INFINITY, false)
    }
}

impl Cylinder {
    pub fn new(material: Material, min: f64, max: f64, capped: bool) -> Self {
        Self {
            min,
            max,
            capped,
            id: 0,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
            material: Arc::new(material),
        }
    }

    fn check_caps(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        x.powi(2) + z.powi(2) <= 1.0
    }

    fn hit_caps(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<f64> {
        
        if !self.capped || ray.direction.y.abs() < 1e-8 {
            return vec![];
        };
        
        let mut hits = vec![];
        let t0 = (self.min - ray.origin.y) / ray.direction.y;
        if t0 >= t_min && t0 <= t_max && Self::check_caps(ray, t0) {
                hits.push(t0);
        }

        let t1 = (self.max - ray.origin.y) / ray.direction.y;
        if t1 >= t_min && t1 <= t_max && Self::check_caps(ray, t1) {
                hits.push(t1);
        }
        hits
    }
}


impl Object for Cylinder {

    fn hit_obj(&self, obj_ray: &Ray, t_min: f64, t_max: f64) -> Option<Vec<f64>> {
        
        let a = obj_ray.direction.x.powi(2) + obj_ray.direction.z.powi(2);
        // No wall intersections.
        if a.abs() < 1e-8 {
            let t = self.hit_caps(obj_ray, t_min, t_max);
            return if t.is_empty() { None } else { Some(t) }
        }

        let b = 2.0 * obj_ray.origin.x * obj_ray.direction.x + 2.0 * obj_ray.origin.z * obj_ray.direction.z;
        let c = obj_ray.origin.x.powi(2) + obj_ray.origin.z.powi(2) - 1.0;

        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }

        let mut close = (-b - disc.sqrt()) / (2.0 * a);
        let mut far = (-b + disc.sqrt()) / (2.0 * a);
        if close > far {
            std::mem::swap(&mut close, &mut far);
        }

        let mut t = vec![];
        if close > t_min && close < t_max {
            let y0 = obj_ray.origin.y + close * obj_ray.direction.y;
            if y0 < self.max && y0 > self.min {
                t.push(close);
            }
        }
        if far > t_min && far < t_max {
            let y1 = obj_ray.origin.y + far * obj_ray.direction.y;
            if y1 < self.max && y1 > self.min {
                t.push(far);
            }
        }

        t.extend(self.hit_caps(obj_ray, t_min, t_max));
        if t.is_empty() { None } else { Some(t) }
    }

    fn normal_obj(&self, point: &Point3) -> Vec3 {
        let dist = point.x.powi(2) + point.z.powi(2);

        if dist < 1.0 && point.y >= self.max - 1e-8 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.min + 1e-8 {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::new(point.x, 0.0, point.z)
        }
    }

    fn material(&self) -> &Arc<Material> {
        &self.material
    }

    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Transformable for Cylinder{

    fn transform(&self) -> &Matrix4 {
        &self.transform
    }

    fn inverse(&self) -> &Matrix4 {
        &self.inverse
    }

    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
    }

    fn set_inverse(&mut self, inverse: Matrix4) {
        self.inverse = inverse;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_miss() {
        let cyl = Cylinder::default();
        
        let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let t = cyl.hit(&ray, -f64::INFINITY, f64::INFINITY);
        assert!(t.is_none());

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let t = cyl.hit(&ray, -f64::INFINITY, f64::INFINITY);
        assert!(t.is_none());

        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let t = cyl.hit(&ray, -f64::INFINITY, f64::INFINITY);
        assert!(t.is_none());
    }

    #[test]
    fn test_cylinder_hit() {
        let cyl = Cylinder::default();

        let ray = Ray::new(Point3::new(1.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let t = cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).unwrap();
        assert_eq!(t[0].t, 5.0);
        assert_eq!(t[1].t, 5.0);


        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let t = cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).unwrap();
        assert_eq!(t[0].t, 4.0);
        assert_eq!(t[1].t, 6.0);
    }

    #[test]
    fn test_cylinder_truncation() {
        let cyl = Cylinder::new(Material::default(), 1.0, 2.0, false);

        let ray = Ray::new(Point3::new(0.0, 1.5, 0.0), Vec3::new(0.1, 1.0, 0.0));
        println!("{:?}", cyl.hit(&ray, -f64::INFINITY, f64::INFINITY));
        assert!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).is_none());

        let ray = Ray::new(Point3::new(0.0, 3.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).is_none());

        let ray = Ray::new(Point3::new(0.0, 1.5, -2.0), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_cylinder_hit_cap() {
        let cyl = Cylinder::new(Material::default(), 1.0, 2.0, true);

        let ray = Ray::new(Point3::new(0.0, 3.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).as_ref().unwrap().len(), 2);

        let ray = Ray::new(Point3::new(0.0, 3.0, -2.0), Vec3::new(0.0, -1.0, 2.0));
        assert_eq!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).as_ref().unwrap().len(), 2);

        let ray = Ray::new(Point3::new(0.0, 4.0, -2.0), Vec3::new(0.0, -1.0, 1.0));
        assert_eq!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).as_ref().unwrap().len(), 2);

        let ray = Ray::new(Point3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 1.0, 2.0));
        assert_eq!(cyl.hit(&ray, -f64::INFINITY, f64::INFINITY).as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_cylinder_obj_normal() {
        let cyl = Cylinder::new(Material::default(), 1.0, 2.0, true);

        let point = Point3::new(0.0, 1.0, 0.0);
        assert_eq!(cyl.normal_at(&point), Vec3::new(0.0, -1.0, 0.0));
        
        let point = Point3::new(0.5, 1.0, 0.0);
        assert_eq!(cyl.normal_at(&point), Vec3::new(0.0, -1.0, 0.0));

        let point = Point3::new(0.0, 2.0, 0.0);
        assert_eq!(cyl.normal_at(&point), Vec3::new(0.0, 1.0, 0.0));
    }
}