use std::sync::Arc;
use crate::{Matrix4, Material, Object, ray::Ray, Point3, Vec3};
use crate::transform::Transformable;

#[derive(Debug)]
pub struct Cone {
    pub id:         usize,
    pub min:        f64,
    pub max:        f64,
    pub capped:     bool,
    pub transform:  Matrix4,
    pub inverse:    Matrix4,
    pub material:   Arc<Material>,
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(Material::default(), -f64::INFINITY, f64::INFINITY, false)
    }
}

impl Cone {
    pub fn new(material: Material, min: f64, max: f64, capped: bool) -> Self {
        Self {
            id: 0,
            min,
            max,
            capped,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
            material: Arc::new(material),
        }
    }

    fn check_caps(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        let y = ray.origin.y + t * ray.direction.y;

        x.powi(2) + z.powi(2) <= y.abs()
    }

    fn hit_caps(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<f64> {
        
        if !self.capped || ray.direction.y.abs() < 1e-8 {
            return vec![];
        };
        
        let mut hits = vec![];
        let close = (self.min - ray.origin.y) / ray.direction.y;
        if close >= t_min && close <= t_max && Self::check_caps(ray, close) {
                hits.push(close);
        }

        let far = (self.max - ray.origin.y) / ray.direction.y;
        if far >= t_min && far <= t_max && Self::check_caps(ray, far) {
                hits.push(far);
        }
        hits
    }
}

impl Object for Cone {

    fn hit_obj(&self, obj_ray: &Ray, t_min: f64, t_max: f64) -> Option<Vec<f64>> {

        let a = obj_ray.direction.x.powi(2) - obj_ray.direction.y.powi(2) + obj_ray.direction.z.powi(2);
        let b = 2.0 * obj_ray.origin.x * obj_ray.direction.x 
        - 2.0 * obj_ray.origin.y * obj_ray.direction.y 
        + 2.0 * obj_ray.origin.z * obj_ray.direction.z;
        let c = obj_ray.origin.x.powi(2) - obj_ray.origin.y.powi(2) + obj_ray.origin.z.powi(2);
        
        if a.abs() < 1e-8 {
            if b.abs() < 1e-8 {
                return None;
            }
            let t = -c / (2.0 * b);
            if t > t_min && t < t_max {
                return Some(vec![t]);
            }
        }
        
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            return None;
        }
        
        let mut hits = vec![];
        let mut close = (-b - disc.sqrt()) / (2.0 * a);
        let mut far = (-b + disc.sqrt()) / (2.0 * a);
        if close > far {
            std::mem::swap(&mut close, &mut far);
        }

        if close > t_min && close < t_max {
            let y0 = obj_ray.origin.y + close * obj_ray.direction.y;
            if self.min < y0 && y0 < self.max {
                hits.push(close);
            }
        }
        if far > t_min && far < t_max {
            let y1 = obj_ray.origin.y + far * obj_ray.direction.y;
            if self.min < y1 && y1 < self.max {
                hits.push(far);
            }
        }

        hits.extend(self.hit_caps(obj_ray, t_min, t_max));
        if hits.is_empty() { None } else { Some(hits) }
    }

    fn normal_obj(&self, point: &Point3) -> Vec3 {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1.0 && point.y >= self.max - 1e-8 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.min + 1e-8 {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::new(point.x, -point.y, point.z)
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn material(&self) -> &Arc<Material> {
        &self.material
    }
}

impl Transformable for Cone {

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
    use crate::math::*;

    #[test]
    fn test_cone_hit() {
        let cone = Cone::default();

        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let t = cone.hit(&ray, 0.0, f64::INFINITY).unwrap();
        assert_eq!(t[0].t, 5.0);
        assert_eq!(t[1].t, 5.0);

        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0));
        let t = cone.hit(&ray, 0.0, f64::INFINITY).unwrap();
        println!("{:?}", t[0].t);
        // assert!(fuzzy_eq_f64(t[0].t, 8.66025));
        // assert!(fuzzy_eq_f64(t[1].t, 8.66025));


    }


}