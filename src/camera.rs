use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Default)]
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
            aspect_ratio:   f64,
            aperture:       f64,
            focus_dist:     f64,
        ) -> Self {
        
        let theta = degrees_to_radians(vert_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalise();
        let u = view_up.cross(w).normalise();
        let v = w.cross(u);
        
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

    pub fn get_ray(&self, a: f64, b: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        
        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + a * self.horizontal + b * self.vertical - self.origin - offset
        )
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}