use std::sync::Arc;
use crate::{Material, Matrix4, Object, ray::Ray, transform::Transformable, Vec3, Point3};

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    id:         usize,
    transform:  Matrix4,
    inverse:    Matrix4,
    material:   Arc<Material>,
}

impl AxisAlignedBoundingBox {
    pub fn new(material: Material) -> Self {
        Self {
            id: 0,
            transform: Matrix4::identity(),
            inverse: Matrix4::identity(),
            material: Arc::new(material),
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (mut close, mut far) = if direction.abs() >= 0.0001 {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * f64::INFINITY, tmax_numerator * f64::INFINITY)
        };

        if close > far {
            std::mem::swap(&mut close, &mut far);
        }

        (close, far)
    }
}

impl Object for AxisAlignedBoundingBox {
    
    fn hit_obj(&self, obj_ray: &Ray, t_min: f64, t_max: f64) -> Option<Vec<f64>> {
        
        let (tmin_x, tmax_x) = self.check_axis(obj_ray.origin.x, obj_ray.direction.x);
        let (tmin_y, tmax_y) = self.check_axis(obj_ray.origin.y, obj_ray.direction.y);
        let (tmin_z, tmax_z) = self.check_axis(obj_ray.origin.z, obj_ray.direction.z);

        let close = tmin_x.max(tmin_y).max(tmin_z);
        let far = tmax_x.min(tmax_y).min(tmax_z);

        if close > far {
            return None;
        }

        let mut hits = vec![];
        if close > t_min && close < t_max {
            hits.push(close);
        }
        if far > t_min && far < t_max {
            hits.push(far);
        }
        if hits.is_empty() { None } else { Some(hits) }
    }

    fn normal_obj(&self, point: &Point3) -> Vec3 {
        let max_c = point.x.abs().max(point.y.abs()).max(point.z.abs());

        // Direction of normal is the direction of the largest component.
        if max_c == point.x.abs() {
            Vec3::new(point.x, 0.0, 0.0)
        } else if max_c == point.y.abs() {
            Vec3::new(0.0, point.y, 0.0)
        } else {
            Vec3::new(0.0, 0.0, point.z)
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

impl Transformable for AxisAlignedBoundingBox {
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
    fn test_bbox_hit() {
        let bbox = AxisAlignedBoundingBox::new(Material::default());
       
        // Check axis.
        // x + 
        let ray = Ray::new(Point3::new(5.0, 0.5, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // x -
        let ray = Ray::new(Point3::new(-5.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // y +
        let ray = Ray::new(Point3::new(0.5, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // y -
        let ray = Ray::new(Point3::new(0.5, -5.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // z +
        let ray = Ray::new(Point3::new(0.5, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // z -
        let ray = Ray::new(Point3::new(0.5, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![4.0, 6.0]));

        // Inside.
        let ray = Ray::new(Point3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, Some(vec![-1.0, 1.0]));
    }

    #[test]
    fn test_bbox_miss() {
        let bbox = AxisAlignedBoundingBox::new(Material::default());

        let ray = Ray::new(Point3::new(-2.0, 0.0, 0.0), Vec3::new(0.2673, 0.5345, 0.8018));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, None);

        let ray = Ray::new(Point3::new(0.0, -2.0, 0.0), Vec3::new(0.8018, 0.2673, 0.5345));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, None);

        let ray = Ray::new(Point3::new(2.0, 2.0, 0.0), Vec3::new(-1.0, 0.0, 0.0));
        let hit = bbox.hit_obj(&ray, -f64::INFINITY, f64::INFINITY);
        assert_eq!(hit, None);
    }

    #[test]
    fn test_bbox_normal_obj() {
        let bbox = AxisAlignedBoundingBox::new(Material::default());

        let obj_norm = bbox.normal_obj(&Point3::new(1.0, 0.5, -0.8));
        assert_eq!(obj_norm, Vec3::new(1.0, 0.0, 0.0));

        let obj_norm = bbox.normal_obj(&Point3::new(-1.0, -0.2, 0.9));
        assert_eq!(obj_norm, Vec3::new(-1.0, 0.0, 0.0));

        let obj_norm = bbox.normal_obj(&Point3::new(0.4, 0.4, -1.0));
        assert_eq!(obj_norm, Vec3::new(0.0, 0.0, -1.0));
    }
}