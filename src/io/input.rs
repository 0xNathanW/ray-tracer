use serde::Deserialize;
use std::fs::read;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, Context};
use crate::{
    Point3,
    Scene,
    Object,
    Material,
    Camera,
    Vec3, 
    Colour,
};
use crate::object::{Sphere, Plane, Disk};
use crate::material::{Lambertian, Metal, Dielectric};

#[derive(Deserialize)]
pub struct Inputs {
    camera: CameraInputs,
    objects: Vec<ObjectInputs>,
}

#[derive(Deserialize)]
pub struct CameraInputs {
    look_from: (f64, f64, f64),
    look_at: (f64, f64, f64),
    vup: (f64, f64, f64),
    vfov: f64,
    aperture: f64,
    focus_dist: f64,
}

#[derive(Deserialize)]
pub enum ObjectInputs {
    Sphere {
        center: (f64, f64, f64),
        radius: f64,
        material: MaterialInputs,
    },
    Plane {
        point: (f64, f64, f64),
        normal: (f64, f64, f64),
        material: MaterialInputs,
    },
    Disk {
        center: (f64, f64, f64),
        normal: (f64, f64, f64),
        radius: f64,
        material: MaterialInputs,
    },
}

#[derive(Deserialize)]
pub enum MaterialInputs {
    Lambertian {
        colour: (f64, f64, f64),
    },
    Metal {
        colour: (f64, f64, f64),
        fuzz: f64,
    },
    Dielectric {
        refraction_index: f64,
    },
}

pub fn parse_scene<P: AsRef<Path>>(path: P, dimensions: (u32, u32)) -> Result<(Scene, Camera)> {
    
    let content = read(path).context("Failed to read scene file")?;
    let a: Inputs = serde_yaml::from_slice(&content).context("Failed to parse scene file")?;
    
    let camera = Camera::new(
        Point3::new(a.camera.look_from.0, a.camera.look_from.1, a.camera.look_from.2),
        Point3::new(a.camera.look_at.0, a.camera.look_at.1, a.camera.look_at.2),
        Vec3::new(a.camera.vup.0, a.camera.vup.1, a.camera.vup.2),
        a.camera.vfov,
        dimensions,
        a.camera.aperture,
        a.camera.focus_dist,
    );

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    a.objects.into_iter().for_each(|obj| {
        match obj {

            ObjectInputs::Sphere { center, radius, material } => {
                let center = Point3::new(center.0, center.1, center.2);
                objects.push(Box::new(Sphere::new(center, radius, parse_material(material))));
            },

            ObjectInputs::Plane { point, normal, material } => {
                let point = Point3::new(point.0, point.1, point.2);
                let normal = Vec3::new(normal.0, normal.1, normal.2);
                objects.push(Box::new(Plane::new(point, normal, parse_material(material))));
            },
            ObjectInputs::Disk { center, normal, radius, material } => {
                let center = Point3::new(center.0, center.1, center.2);
                let normal = Vec3::new(normal.0, normal.1, normal.2);
                objects.push(Box::new(Disk::new(center, normal, radius, parse_material(material))));
            },
        }
    });

    Ok((Scene::new(objects), camera))
}


pub fn parse_material(material: MaterialInputs) -> Arc<dyn Material> {
    match material {
        MaterialInputs::Lambertian { colour } => {
            Arc::new(Lambertian::new(Colour::new(colour.0, colour.1, colour.2)))
        },
        MaterialInputs::Metal { colour, fuzz } => {
            Arc::new(Metal::new(Colour::new(colour.0, colour.1, colour.2), fuzz))
        },
        MaterialInputs::Dielectric { refraction_index } => {
            Arc::new(Dielectric::new(refraction_index))
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    // Make sure the test scene file parses without error.
    #[test]
    fn test_input() {
        let path = Path::new("scenes/test_scene.yaml");
        let (scene, _camera) = parse_scene(path, (200, 100)).unwrap();
        
        assert_eq!(scene.objects.len(), 3);
    }
}