pub mod vec3;
pub mod point3;
pub mod matrix;
pub mod colour;
pub mod ray;
pub mod object;
pub mod scene;
pub mod camera;
pub mod material;
pub mod render;
pub mod output;

pub use vec3::Vec3;
pub use point3::Point3;
pub use matrix::Matrix;
pub use colour::Colour;
pub use material::{
    Material,
    Lambertian,
    Metal,
    Dielectric,
};
pub use object::{
    Sphere,
    Object,
    Plane,
    Disk,
};
pub use scene::Scene;
pub use camera::Camera;
pub use output::{OutputFormat, write_to_file};
pub use render::{render, Image};