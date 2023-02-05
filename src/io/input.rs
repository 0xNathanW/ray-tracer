use serde::Deserialize;
use std::fs::read;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, Context};
use crate::*;
use crate::pattern::*;
use crate::object::{Sphere, Plane, Disk};

#[derive(Deserialize, Debug)]
pub struct Inputs {
    camera:  CameraInputs,
    objects: Vec<ObjectInputs>,
    lights:  Vec<LightInputs>,
}

#[derive(Deserialize, Debug)]
pub struct CameraInputs {
    look_from:  (f64, f64, f64),
    look_at:    (f64, f64, f64),
    vup:        (f64, f64, f64),
    vfov:       f64,
    aperture:   f64,
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
pub struct MaterialInputs {
    #[serde(default = "colour_default")]
    colour: (f64, f64, f64),
    #[serde(default)]
    pattern: Option<PatternInputs>,
    #[serde(default = "ambient_default")]
    ambient: f64,
    #[serde(default = "diffuse_default")]
    diffuse: f64,
    #[serde(default = "specular_default")]
    specular: f64,
    #[serde(default = "shininess_default")]
    shininess: f64,
    #[serde(default = "reflectivity_default")]
    reflectivity: f64,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PatternInputs {
    r#type: PatternType,
    colour_a: (f64, f64, f64),
    colour_b: (f64, f64, f64),
    transform: Option<Vec<TransformationInput>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum PatternType {
    Stripes,
    Gradient,
    Rings,
    Checkers,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, PartialEq, Debug)]
pub enum TransformationInput {
    Translate(f64, f64, f64),
    Scale(f64, f64, f64),
    Scale_uniform(f64),
    Rotate_x(f64),
    Rotate_y(f64),
    Rotate_z(f64),
}

#[derive(Deserialize, Debug)]
struct LightInputs {
    position: (f64, f64, f64),
    colour:   (f64, f64, f64),
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
    );

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    a.objects.into_iter().for_each(|obj| {
        let material = parse_material(obj.material);
        let mut object: Box<dyn Object> = match obj.r#type {
            ObjectType::Sphere => Box::new(Sphere::new(material)),
            ObjectType::Plane  => Box::new(Plane::new(material)),
            ObjectType::Disk   => Box::new(Disk::new(material)),
        };

        if let Some(transformations) = obj.transform {
            apply_object_transformations(&mut *object, transformations);
        }
        objects.push(object);
    });

    let lights = parse_lights(a.lights);

    Ok((Arc::new(Scene::new(objects, lights)), camera))
}

// Should be a better way to do this...
fn parse_material(material: MaterialInputs) -> Arc<Material> {
    
    let pattern: Option<Arc<dyn Pattern>> = match material.pattern {
        Some(pattern) => {
            let pattern_out: Arc<dyn Pattern> = match pattern.r#type {

                PatternType::Stripes => {
                    let mut stripes = Stripes::new(
                        Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                        Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
                    );
                    if let Some(transformations) = pattern.transform {
                        apply_pattern_transformations(&mut stripes, transformations);
                    }
                    Arc::new(stripes)
                },

                PatternType::Gradient => {
                    let mut gradient = Gradient::new(
                        Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                        Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
                    );
                    if let Some(transformations) = pattern.transform {
                        apply_pattern_transformations(&mut gradient, transformations);
                    }
                    Arc::new(gradient)
                },

                PatternType::Rings => {
                    let mut rings = Rings::new(
                        Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                        Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
                    );
                    if let Some(transformations) = pattern.transform {
                        apply_pattern_transformations(&mut rings, transformations);
                    }
                    Arc::new(rings)
                },

                PatternType::Checkers => {
                    let mut checkers = Checkers::new(
                        Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                        Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
                    );
                    if let Some(transformations) = pattern.transform {
                        apply_pattern_transformations(&mut checkers, transformations);
                    }
                    Arc::new(checkers)
                },
            };
            Some(pattern_out)
        },

        None => None,
    };

    Arc::new(Material::new(
        Colour::new(material.colour.0, material.colour.1, material.colour.2),
        pattern,
        material.ambient,
        material.diffuse,
        material.specular,
        material.shininess,
        material.reflectivity,
    ))
}

fn apply_object_transformations(obj: &mut dyn Object, transformations: Vec<TransformationInput>) {
    transformations.into_iter().for_each(|transformation| {
        match transformation {
            TransformationInput::Translate(x, y, z) => {
                obj.translate(x, y, z);
            },
            TransformationInput::Scale(x, y, z) => {
                obj.scale(x, y, z);
            },
            TransformationInput::Scale_uniform(s) => {
                obj.scale_uniform(s);
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

// When trait upcasting is stable, this can be removed, and the function above can be used instead.
fn apply_pattern_transformations(pattern: &mut dyn Pattern, transformations: Vec<TransformationInput>) {
    transformations.into_iter().for_each(|transformation| {
        match transformation {
            TransformationInput::Translate(x, y, z) => {
                pattern.translate(x, y, z);
            },
            TransformationInput::Scale(x, y, z) => {
                pattern.scale(x, y, z);
            },
            TransformationInput::Scale_uniform(s) => {
                pattern.scale_uniform(s);
            },
            TransformationInput::Rotate_x(angle) => {
                pattern.rotate(Axis::X, angle)
            },
            TransformationInput::Rotate_y(angle) => {
                pattern.rotate(Axis::Y, angle)
            },
            TransformationInput::Rotate_z(angle) => {
                pattern.rotate(Axis::Z, angle)
            },
        }
    });
}

fn parse_lights(lights: Vec<LightInputs>) -> Vec<Light> {
    lights.into_iter().map(|light| {
        Light::new(
            Point3::new(light.position.0, light.position.1, light.position.2),
            Colour::new(light.colour.0, light.colour.1, light.colour.2),
        )
    }).collect()
}

fn colour_default() -> (f64, f64, f64) {
    (1.0, 1.0, 1.0)
}

fn ambient_default() -> f64 {
    0.1
}

fn diffuse_default() -> f64 {
    0.9
}

fn specular_default() -> f64 {
    0.9
}

fn shininess_default() -> f64 {
    200.0
}

fn reflectivity_default() -> f64 {
    0.0
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
                  material:
                    colour: [1.0, 0.2, 1.0]
                    pattern:
                        type: !Gradient
                        colour_a: [1.0, 0.0, 0.0]
                        colour_b: [0.0, 0.0, 1.0]
                  transform:
                    - !Translate [0.0, 0.0, -1.0]
                    - !Scale [0.5, 0.5, 0.5]

            lights:
                - position: [0.0, 0.0, -10.0]
                  colour:   [1.0, 1.0, 1.0]
        ";

        let a: Inputs = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(a.camera.look_from, (1.0, 2.0, 3.0));
        assert_eq!(a.camera.look_at, (4.0, 5.0, 6.0));
        assert_eq!(a.camera.vup, (7.0, 8.0, 9.0));
        assert_eq!(a.camera.vfov, 90.0);
        assert_eq!(a.camera.aperture, 0.0);

        assert_eq!(a.objects.len(), 1);
        assert_eq!(a.objects[0].r#type, ObjectType::Sphere);
        assert_eq!(a.objects[0].material, 
            MaterialInputs {
                colour: (1.0, 0.2, 1.0),
                pattern: Some(PatternInputs {
                    r#type: PatternType::Gradient,
                    colour_a: (1.0, 0.0, 0.0),
                    colour_b: (0.0, 0.0, 1.0),
                    transform: None,
                }),
                ambient: ambient_default(),
                diffuse: diffuse_default(),
                specular: specular_default(),
                shininess: shininess_default(),
                reflectivity: reflectivity_default(),
            });
        assert_eq!(a.objects[0].transform, Some(vec![
            TransformationInput::Translate(0.0, 0.0, -1.0),
            TransformationInput::Scale(0.5, 0.5, 0.5),
        ]));

        assert_eq!(a.lights.len(), 1);
        assert_eq!(a.lights[0].position, (0.0, 0.0, -10.0));
        assert_eq!(a.lights[0].colour, (1.0, 1.0, 1.0));
    }

    #[test]
    fn test_input_from_file() {

        let a: Inputs = serde_yaml::from_slice(&read("scenes/sphere.yaml").unwrap()).unwrap();
        assert_eq!(a.camera.look_from, (0.0, 0.0, 2.0));
        assert_eq!(a.camera.look_at, (2.0, 2.0, 2.0));
        assert_eq!(a.objects[0].material, 
            MaterialInputs {
                colour: (1.0, 0.2, 1.0),
                pattern: None,
                ambient: ambient_default(),
                diffuse: diffuse_default(),
                specular: specular_default(),
                shininess: shininess_default(),
                reflectivity: reflectivity_default(),
        });
        assert_eq!(a.lights[0].position, (-10.0, 30.0, 20.0));
    }
}