use anyhow::Context;
use rand::prelude::*;
use crate::transform::Transformable;
use crate::{Point3, Vec3, Matrix4, Translation};
use crate::ray::Ray;

#[derive(Default, Debug, Clone, Copy)]
pub struct Camera {
    transform:          Matrix4,
    inverse:            Matrix4,
    half_width:         f64,
    half_height:        f64,
    pixel_size:         f64,
    lens_radius:        f64,
}

impl Camera {

    pub fn new(
        look_from:      Point3,
        look_at:        Point3,
        view_up:        Vec3,
        vert_fov:       f64, // Vertical field of view in degrees.
        dimensions:     (u32, u32),
        aperture:       f64,
    ) -> Self {
        
        let transform = Camera::view_matrix(look_from, look_at, view_up);
        let inverse = transform.try_inverse().context("Camera matrix is not invertible").unwrap();

        // Cut vfov in half creating a right-angle triangle.
        let half_view = (vert_fov.to_radians() / 2.0).tan();
        let aspect_ratio = dimensions.0 as f64 / dimensions.1 as f64;

        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };

        Self {
            transform,
            inverse,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / dimensions.0 as f64,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        
        let offset_x = (x as f64 + 0.5) * self.pixel_size;
        let offset_y = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let pixel = self.inverse.transform_point(&Point3::new(world_x, world_y, -1.0));
        let origin = self.inverse.transform_point(&Point3::origin());
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn view_matrix(from: Point3, to: Point3, up: Vec3) -> Matrix4 {
        let f = (to - from).normalize();
        let s = f.cross(&(up.normalize()));
        let u = s.cross(&f);

        Matrix4::new(
            s.x,   s.y,  s.z, 0.0,
            u.x,   u.y,  u.z, 0.0,
            -f.x, -f.y, -f.z, 0.0,
            0.0,   0.0,  0.0, 1.0,
        ) * Translation::new(-from.x, -from.y, -from.z).to_homogeneous()
    }
}

impl Transformable for Camera {

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
    use crate::{Scale, math::{fuzzy_eq_f64, fuzzy_eq_vec}};

    #[test]
    fn test_view_matrix() {

        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = Camera::view_matrix(from, to, up);
        assert_eq!(view_matrix, Matrix4::identity());

        let from = Point3::new(0.0, 0.0, 0.0);
        let to = Point3::new(0.0, 0.0, 1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let view_matrix = Camera::view_matrix(from, to, up);
        assert_eq!(view_matrix, Scale::new(-1.0, 1.0, -1.0).to_homogeneous());
    }

    #[test]
    fn test_new_camera() {
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0, 
            (200, 125), 
            0.0);
        assert!(fuzzy_eq_f64(camera.pixel_size, 0.01));
    
        let camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0, 
            (125, 200), 
            0.0);
        assert!(fuzzy_eq_f64(camera.pixel_size, 0.01));
    }

    #[test]
    fn test_get_ray() {

        let mut camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0, 
            (201, 101), 
            0.0
        );

        // Center of canvas.
        let ray1 = camera.get_ray(100, 50);
        assert_eq!(ray1.origin, Point3::new(0.0, 0.0, 0.0));
        assert!(fuzzy_eq_vec(&ray1.direction, &Vec3::new(0.0, 0.0, -1.0)));

        // Corner of canvas.
        let ray2 = camera.get_ray(0, 0);
        assert_eq!(ray2.origin, Point3::origin());
        assert!(fuzzy_eq_vec(&ray2.direction, &Vec3::new(0.66519, 0.33259, -0.66851)));

        // Transformed camera.
        camera.rotate(crate::Axis::Y, f64::to_radians(45.0));
        camera.translate(0.0, -2.0, 5.0);
        let ray3 = camera.get_ray(100, 50);
        assert_eq!(ray3.origin, Point3::new(0.0, 2.0, -5.0));
        assert!(fuzzy_eq_vec(&ray3.direction, &Vec3::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)));
    }
}