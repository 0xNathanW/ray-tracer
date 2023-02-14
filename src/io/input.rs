use serde::Deserialize;
use std::{fs::read, path::Path, sync::Arc};
use anyhow::{Result, Context};
use crate::*;
use crate::pattern::*;
use crate::object::{Sphere, Plane, Disk, AxisAlignedBoundingBox, Cone, Cylinder};

#[derive(Deserialize, Debug)]
pub struct Inputs {
    
    #[serde(default = "camera_default")]
    camera:  CameraInputs,
    
    objects: Vec<ObjectInputs>,
    
    #[serde(default = "lights_default")]
    lights:  Vec<LightInputs>,

    #[serde(default = "background_default")]
    background: (f64, f64, f64),
}

#[derive(Deserialize, Debug)]
pub struct CameraInputs {

    #[serde(default = "from_default")]
    look_from:  (f64, f64, f64),
    
    #[serde(default = "at_default")]
    look_at:    (f64, f64, f64),
    
    #[serde(default = "up_default")]
    vup:        (f64, f64, f64),
    
    #[serde(default = "vfov_default")]
    vfov:       f64,
    
    #[serde(default)]
    aperture:   f64,
}

#[derive(Deserialize, Debug)]
pub struct ObjectInputs {
    r#type:    ObjectType,
    #[serde(default = "material_default")]
    material:  MaterialInputs,
    transform: Option<Vec<TransformationInput>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum ObjectType {
    Sphere,
    Plane,
    Disk,
    Box,
    Cylinder {
        #[serde(default = "min_default")]
        min: f64,
        #[serde(default = "max_default")]
        max: f64,
        #[serde(default)]
        closed: bool,
    },
    Cone {
        #[serde(default = "min_default")]
        min: f64,
        #[serde(default = "max_default")]
        max: f64,
        #[serde(default)]
        closed: bool,
    },
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum MaterialInputs {
    Glass,
    Metal {
        colour: (f64, f64, f64),
        pattern: Option<PatternInputs>,
    },
    Plastic {
        colour: (f64, f64, f64),
        pattern: Option<PatternInputs>,
    },
    Custom(CustomInputs),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CustomInputs {
    
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
    
    #[serde(default)]
    reflective: f64,

    #[serde(default)]
    transparency: f64,

    #[serde(default = "refractive_default")]
    refractive_index: f64,
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

#[derive(Deserialize, Debug, PartialEq)]
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
            ObjectType::Box    => Box::new(AxisAlignedBoundingBox::new(material)),

            ObjectType::Cylinder { min, max, closed } => Box::new(Cylinder::new(material, min, max, closed)),
            ObjectType::Cone { min, max, closed }     => Box::new(Cone::new(material, min, max, closed)),
        };

        if let Some(transformations) = obj.transform {
            apply_object_transformations(&mut *object, transformations);
        }
        objects.push(object);
    });

    let lights = parse_lights(a.lights);
    let background = Colour::new(a.background.0, a.background.1, a.background.2);
    Ok((Arc::new(Scene::new(objects, lights, background)), camera))
}

fn parse_material(material: MaterialInputs) -> Material {
    match material {
        MaterialInputs::Glass => Material::glass(),
        MaterialInputs::Metal { colour, pattern } => {
            Material::metal(Colour::new(colour.0, colour.1, colour.1), pattern.map(parse_pattern))
        }
        MaterialInputs::Plastic { colour, pattern } => {
            Material::plastic(Colour::new(colour.0, colour.1, colour.1), pattern.map(parse_pattern))
        }
        MaterialInputs::Custom(custom) => parse_custom(custom),
    }
}

// Should be a better way to do this...
fn parse_custom(material: CustomInputs) -> Material {
    Material::new(
        Colour::new(material.colour.0, material.colour.1, material.colour.2),
        material.pattern.map(parse_pattern),
        material.ambient,
        material.diffuse,
        material.specular,
        material.shininess,
        material.reflective,
        material.transparency,
        material.refractive_index,
    )
}

fn parse_pattern(pattern: PatternInputs) -> Arc<dyn Pattern> {

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
        }
        PatternType::Gradient => {
            let mut gradient = Gradient::new(
                Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
            );
            if let Some(transformations) = pattern.transform {
                apply_pattern_transformations(&mut gradient, transformations);
            }
            Arc::new(gradient)
        }
        PatternType::Rings => {
            let mut rings = Rings::new(
                Colour::new(pattern.colour_a.0, pattern.colour_a.1, pattern.colour_a.2),
                Colour::new(pattern.colour_b.0, pattern.colour_b.1, pattern.colour_b.2),
            );
            if let Some(transformations) = pattern.transform {
                apply_pattern_transformations(&mut rings, transformations);
            }
            Arc::new(rings)
        }
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
    pattern_out
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

// When trait upcasting is stable, this can be removed, and the function above can be us`ed instead.
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

fn background_default() -> (f64, f64, f64) {
    (0.0, 0.0, 0.0)
}

fn camera_default() -> CameraInputs {
    CameraInputs {
        look_from: (0.0, 5.0, 0.0),
        look_at: (0.0, 0.0, 10.0),
        vup: (0.0, 1.0, 0.0),
        vfov: 90.0,
        aperture: 0.0,
    }
}

fn lights_default() -> Vec<LightInputs> {
    vec![
        LightInputs {
            position: (-10.0, 10.0, -10.0),
            colour: (1.0, 1.0, 1.0),
        }
    ]
}

fn min_default() -> f64 {
    -f64::INFINITY
}

fn max_default() -> f64 {
    f64::INFINITY
}

fn material_default() -> MaterialInputs {
    MaterialInputs::Custom(CustomInputs {
        colour: colour_default(),
        pattern: None,
        ambient: ambient_default(),
        diffuse: diffuse_default(),
        specular: specular_default(),
        shininess: shininess_default(),
        reflective: 0.0,
        transparency: 0.0,
        refractive_index: refractive_default(),
    })
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

fn refractive_default() -> f64 {
    1.0
}

fn from_default() -> (f64, f64, f64) {
    (0.0, 0.0, 0.0)
}

fn at_default() -> (f64, f64, f64) {
    (0.0, 0.0, -1.0)
}

fn up_default() -> (f64, f64, f64) {
    (0.0, 1.0, 0.0)
}

fn vfov_default() -> f64 {
    90.0
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
                  material: !Custom
                    colour: [1.0, 0.0, 0.0]
                    ambient: 0.1
                    diffuse: 0.9
                    specular: 0.9
                    shininess: 200.0
                    reflective: 0.0
                    transparency: 0.0
                    refractive_index: 1.0

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
            MaterialInputs::Custom(CustomInputs {
                colour: (1.0, 0.0, 0.0),
                pattern: None,
                ambient: ambient_default(),
                diffuse: diffuse_default(),
                specular: specular_default(),
                shininess: shininess_default(),
                reflective: 0.0,
                transparency: 0.0,
                refractive_index: refractive_default(),
            }));
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
        let a: Inputs = serde_yaml::from_slice(&read("scenes/tests/test_input.yaml").unwrap()).unwrap();
        
        assert_eq!(a.camera.look_from, (0.0, 0.0, 2.0));
        assert_eq!(a.camera.vfov, 15.0);

        let sphere = &a.objects[0];
        assert_eq!(sphere.r#type, ObjectType::Sphere);
        assert_eq!(sphere.material, MaterialInputs::Plastic {
            colour: (1.0, 0.0, 1.0),
            pattern: Some(
                PatternInputs {
                    r#type: PatternType::Stripes,
                    colour_a: (1.0, 0.0, 1.0),
                    colour_b: (0.0, 0.0, 1.0),
                    transform: Some(vec![
                        TransformationInput::Scale_uniform(0.1),
                        TransformationInput::Rotate_z(90.0)
                    ]),
                }
            )
        });
        assert_eq!(sphere.transform, Some(vec![
            TransformationInput::Translate(30.0, 30.0, 2.0),
            TransformationInput::Scale_uniform(4.0),
        ]));

        let cone = &a.objects[1];
        assert_eq!(cone.r#type, ObjectType::Cone {
            min: -f64::INFINITY,
            max: f64::INFINITY,
            closed: false,
        });
        assert_eq!(cone.material, MaterialInputs::Glass);
        assert_eq!(cone.transform, Some(vec![TransformationInput::Rotate_x(45.0)]));

        let boxx = &a.objects[2];
        assert_eq!(boxx.r#type, ObjectType::Box);
        assert_eq!(boxx.material, MaterialInputs::Metal {
            colour: (1.0, 0.5, 1.0),
            pattern: None,
        });

        let lights = &a.lights;
        assert_eq!(lights[0], LightInputs {
            position: (-10.0, 30.0, 20.0),
            colour: (1.0, 1.0, 1.0),
        });
    }
}