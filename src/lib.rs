#![allow(dead_code)]

pub mod colour;
pub mod ray;
pub mod object;
pub mod scene;
pub mod camera;
pub mod material;
pub mod light;
pub mod render;
pub mod pattern;
mod intersection;
mod transform;
mod math;
mod io;

pub use colour::Colour;
pub use material::Material;
pub use object::Object;
pub use scene::Scene;
pub use camera::Camera;
pub use io::{OutputFormat, write_to_file, parse_scene};
pub use render::{render, Image};
pub use light::Light;

// Type aliases.
pub type Point3       = nalgebra::Point3<f64>;
pub type Vec3         = nalgebra::Vector3<f64>;
pub type Matrix3      = nalgebra::Matrix3<f64>;
pub type Matrix4      = nalgebra::Matrix4<f64>;
pub type Translation  = nalgebra::geometry::Translation3<f64>;
pub type Rotation     = nalgebra::geometry::Rotation3<f64>;
pub type Transform    = nalgebra::geometry::Transform3<f64>;
pub type Scale        = nalgebra::geometry::Scale3<f64>;

pub enum Axis {
    X,
    Y,
    Z,
}

pub fn default_dims() -> (u32, u32) {
    (1280, 720)
}

#[cfg(test)]
mod random_tests {

    #[test]
    fn it_works() {
    
    }
}