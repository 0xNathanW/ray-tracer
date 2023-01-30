use serde::Deserialize;
use std::fs::read;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, Context};
use crate::{
    Axis,
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

#[derive(Deserialize, Debug)]
pub struct Inputs {
    camera:  CameraInputs,
    objects: Vec<ObjectInputs>,
}

#[derive(Deserialize, Debug)]
pub struct CameraInputs {
    look_from:  (f64, f64, f64),
    look_at:    (f64, f64, f64),
    vup:        (f64, f64, f64),
    vfov:       f64,
    aperture:   f64,
    focus_dist: f64,
}

#[derive(Deserialize, Debug)]
pub struct ObjectInputs {
    r#type:    ObjectType,
    material:  MaterialInputs,
    transform: Option<Vec<TransformationInput>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum ObjectType {
    Sphere,
    Plane,
    Disk,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum MaterialInputs {
    Lambertian {
        colour: (f64, f64, f64),
    },
    Metal {
        colour: (f64, f64, f64),
        fuzz:   f64,
    },
    Dielectric {
        refraction_index: f64,
    },
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, PartialEq, Debug)]
pub enum TransformationInput {
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    Rotate_x(f64),
    Rotate_y(f64),
    Rotate_z(f64),
}

pub fn parse_scene<P: AsRef<Path>>(path: P, dimensions: (u32, u32)) -> Result<(Arc<Scene>, Camera)> {
    
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
        let material = parse_material(obj.material);
        let mut object: Box<dyn Object> = match obj.r#type {
            ObjectType::Sphere => Box::new(Sphere::new(material)),
            ObjectType::Plane => Box::new(Plane::new(material)),
            ObjectType::Disk => Box::new(Disk::new(material)),
        };

        if let Some(transformations) = obj.transform {
            parse_and_apply_transformations(&mut *object, transformations);
        }
        objects.push(object);
    });
    Ok((Arc::new(Scene::new(objects)), camera))
}

fn parse_material(material: MaterialInputs) -> Arc<dyn Material> {
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

fn parse_and_apply_transformations(obj: &mut dyn Object, transformations: Vec<TransformationInput>) {
    transformations.into_iter().for_each(|transformation| {
        match transformation {
            TransformationInput::Translate(x, y, z) => {
                obj.translate(x, y, z);
            },
            TransformationInput::Scale(x, y, z) => {
                obj.scale(x, y, z);
            },
            TransformationInput::Rotate_x(angle) => {
                obj.rotate(Axis::X, angle)
            },
            TransformationInput::Rotate_y(angle) => {
                obj.rotate(Axis::Y, angle)
            },
            TransformationInput::Rotate_z(angle) => {
                obj.rotate(Axis::Z, angle)
            },
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    // Make sure the test scene file parses without error.
    #[test]
    fn test_simple_input() {
        
        let yaml = "
            camera:
                look_from:  [1.0, 2.0, 3.0]
                look_at:    [4.0, 5.0, 6.0]
                vup:        [7.0, 8.0, 9.0]
                vfov:       90.0
                aperture:   0.0
                focus_dist: 1.0

            objects:
                - type: !Sphere
                  material: !Lambertian
                    colour: [0.8, 0.3, 0.3]
                  transform:
                    - !Translate [0.0, 0.0, -1.0]
                    - !Scale [0.5, 0.5, 0.5]
        ";

        let a: Inputs = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(a.camera.look_from, (1.0, 2.0, 3.0));
        assert_eq!(a.camera.look_at, (4.0, 5.0, 6.0));
        assert_eq!(a.camera.vup, (7.0, 8.0, 9.0));
        assert_eq!(a.camera.vfov, 90.0);
        assert_eq!(a.camera.aperture, 0.0);
        assert_eq!(a.camera.focus_dist, 1.0);

        assert_eq!(a.objects.len(), 1);
        assert_eq!(a.objects[0].r#type, ObjectType::Sphere);
        assert_eq!(a.objects[0].material, MaterialInputs::Lambertian { colour: (0.8, 0.3, 0.3) });
        assert_eq!(a.objects[0].transform, Some(vec![
            TransformationInput::Translate(0.0, 0.0, -1.0),
            TransformationInput::Scale(0.5, 0.5, 0.5),
        ]));
    }

    #[test]
    fn test_input_from_file() {

        let a: Inputs = serde_yaml::from_slice(&read("scenes/test_scene.yaml").unwrap()).unwrap();
        assert_eq!(a.camera.look_from, (1.0, 2.0, 3.0));
        assert_eq!(a.camera.look_at, (4.0, 5.0, 6.0));
        assert_eq!(a.objects[0].material, MaterialInputs::Lambertian { colour: (0.8, 0.3, 0.3) });
    }
}