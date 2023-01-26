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
pub use object::Object;
pub use scene::Scene;
pub use camera::Camera;
pub use io::{OutputFormat, write_to_file, parse_scene};
pub use render::{render, Image};

pub type Point3 = nalgebra::Point3<f64>;
pub type Vec3 = nalgebra::Vector3<f64>;
