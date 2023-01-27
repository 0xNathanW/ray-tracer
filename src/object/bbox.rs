use crate::{ray::Ray, Point3}; 

pub trait BoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool;
}

pub struct AxisAlignedBoundingBox {
    min: Point3,
    max: Point3,
}

impl AxisAlignedBoundingBox {
    pub fn new(min: Point3, max: Point3) -> AxisAlignedBoundingBox {
        assert!(min < max);
        AxisAlignedBoundingBox { min, max }
    }

    pub fn surrounding_box(box0: &AxisAlignedBoundingBox, box1: &AxisAlignedBoundingBox) -> AxisAlignedBoundingBox {
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AxisAlignedBoundingBox::new(small, big)
    }

    pub fn surrounding_box_list(list: &[AxisAlignedBoundingBox]) -> AxisAlignedBoundingBox {
        let mut small = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut big = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);
        for box_ in list {
            small = Point3::new(
                small.x.min(box_.min.x),
                small.y.min(box_.min.y),
                small.z.min(box_.min.z),
            );
            big = Point3::new(
                big.x.max(box_.max.x),
                big.y.max(box_.max.y),
                big.z.max(box_.max.z),
            );
        }
        AxisAlignedBoundingBox::new(small, big)
    }

    // Resize the bounding box to include the given point.
    pub fn resize(&mut self, point: &Point3) {
        self.min = Point3::new(
            self.min.x.min(point.x),
            self.min.y.min(point.y),
            self.min.z.min(point.z),
        );
        self.max = Point3::new(
            self.max.x.max(point.x),
            self.max.y.max(point.y),
            self.max.z.max(point.z),
        );
    }

    fn single_axis_hit(min: f64, max: f64, origin: f64, direction: f64) -> (f64, f64) {
        let inv_direction = 1.0 / direction;
    
        let t0 = (min - origin) * inv_direction;
        let t1 = (max - origin) * inv_direction;
    
        if inv_direction < 0.0 {
            (t1, t0)
        } else {
            (t0, t1)
        }
    }
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        AxisAlignedBoundingBox {
            min: Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            max: Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        }
    }
}

impl BoundingBox for AxisAlignedBoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {

        let (t0, t1) = Self::single_axis_hit(self.min.x, self.max.x, ray.origin.x, ray.direction.x);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        let (t0, t1) = Self::single_axis_hit(self.min.y, self.max.y, ray.origin.y, ray.direction.y);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        let (t0, t1) = Self::single_axis_hit(self.min.z, self.max.z, ray.origin.z, ray.direction.z);
        let t_min = t_min.max(t0);
        let t_max = t_max.min(t1);
        if t_max <= t_min {
            return false;
        }

        true    
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vec3;

    #[test]
    fn test_bbox_hit() {

        let bbox = AxisAlignedBoundingBox::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
        let mut ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        
        assert!(bbox.hit(&ray, 0.0, f64::INFINITY));
        
        ray.direction = Vec3::new(-1.0, -1.0, -1.0);
        assert!(!bbox.hit(&ray, 0.0, f64::INFINITY));

        ray.direction = Vec3::new(1.0, 0.0, 0.0);
        assert!(!bbox.hit(&ray, 0.0, f64::INFINITY));

        ray.direction = Vec3::new(0.0, 1.0, 0.0);
        assert!(!bbox.hit(&ray, 0.0, f64::INFINITY));

        ray.direction = Vec3::new(0.5, 0.5, 0.5);
        assert!(bbox.hit(&ray, 0.0, f64::INFINITY));
    }

    #[test]
    fn test_bbox_resize() {
        let mut bbox = AxisAlignedBoundingBox::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
        bbox.resize(&Point3::new(0.0, 0.0, 0.0));
        assert_eq!(bbox.min, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(bbox.max, Point3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_bbox_surrounding() {
        let bbox0 = AxisAlignedBoundingBox::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
        let bbox1 = AxisAlignedBoundingBox::new(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
        let bbox2 = AxisAlignedBoundingBox::surrounding_box(&bbox0, &bbox1);
        assert_eq!(bbox2.min, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(bbox2.max, Point3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_bbox_surrounding_list() {
        let bbox0 = AxisAlignedBoundingBox::new(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
        let bbox1 = AxisAlignedBoundingBox::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(3.0, 3.0, 3.0));
        let bbox2 = AxisAlignedBoundingBox::new(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0));
        let bbox3 = AxisAlignedBoundingBox::surrounding_box_list(&vec![bbox0, bbox1, bbox2]);
        assert_eq!(bbox3.min, Point3::new(-2.0, -2.0, -2.0));
        assert_eq!(bbox3.max, Point3::new(4.0, 4.0, 4.0));  
    }

}