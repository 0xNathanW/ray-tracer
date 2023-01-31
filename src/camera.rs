use rand::prelude::*;
use crate::{Point3, Vec3};
use crate::ray::Ray;
use crate::math::rand_in_unit_disk;

#[derive(Default, Debug, Clone, Copy)]
pub struct Camera {
    origin:             Point3,
    lower_left_corner:  Point3,
    horizontal:         Vec3,
    vertical:           Vec3,
    u:                  Vec3,
    v:                  Vec3,
    #[allow(dead_code)]
    w:                  Vec3,
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
        focus_dist:     f64,
    ) -> Self {
        
        let h = (vert_fov.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = (dimensions.0 as f64 / dimensions.1 as f64) * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = view_up.cross(&w).normalize();
        let v = w.cross(&u);
        
        let origin = look_from;
        let horizontal = viewport_width * u * focus_dist;
        let vertical = viewport_height * v * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, a: f64, b: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * rand_in_unit_disk(rng);
        let offset = self.u * rd.x + self.v * rd.y;
        
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + a * self.horizontal + b * self.vertical - self.origin - offset,
        )
    }
}
