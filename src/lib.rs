#![allow(dead_code)]

pub mod colour;
pub mod ray;
pub mod object;
pub mod scene;
pub mod camera;
pub mod material;
pub mod render;
mod math;
mod io;

pub use colour::Colour;
pub use material::Material;
pub use object::{Object, Intersection};
pub use scene::Scene;
pub use camera::Camera;
pub use io::{OutputFormat, write_to_file, parse_scene};
pub use render::{render, Image};

// Type aliases.
pub type Point3       = nalgebra::Point3<f64>;
pub type Vec3         = nalgebra::Vector3<f64>;
pub type Matrix3      = nalgebra::Matrix3<f64>;
pub type Matrix4      = nalgebra::Matrix4<f64>;
pub type Translation3 = nalgebra::geometry::Translation3<f64>;
pub type Rotation3    = nalgebra::geometry::Rotation3<f64>;

pub fn default_dims() -> (u32, u32) {
    (1280, 720)
}

#[cfg(test)]
mod random_tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let t = nalgebra::Rotation::from_euler_angles(0.1, 0.2, 0.3);
        let b = apply_t(&v, t.into());
        println!("{:?}", b);
    }

    fn apply_t(v: &Vec3, t: nalgebra::Matrix4<f64>) -> Vec3 {
        t.transform_vector(v)
    }
}