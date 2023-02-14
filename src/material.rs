use std::sync::Arc;
use crate::colour::{Colour, BLACK};
use crate::{Matrix4, Point3};
use crate::intersection::Intersection;
use crate::light::Light;
use crate::math::reflect;
use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Material {
    pub colour:         Colour,
    pub pattern:        Option<Arc<dyn Pattern>>,
    // Ambient reflection is background lighting, or light reflected from other
    // objects in the environment. The Phong model treats this as a constant,
    // coloring all points on the surface equally.
    pub ambient:        f64,
    // Diffuse reflection is light reflected from a matte surface.
    pub diffuse:        f64,
    // Reflection of light source, results in specular highlights.
    pub specular:       f64,
    // The higher the shininess, the smaller and tighter the specular highlights.
    pub shininess:      f64,
    // The amount of light reflected from a surface.
    pub reflect: f64,
    // The amount of light refracted through a surface.
    pub transparency:   f64,
    // The index of refraction of a surface.
    pub refractive_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            colour:           Colour::new(1.0, 1.0, 1.0),
            pattern:          None,
            ambient:          0.1,
            diffuse:          0.9,
            specular:         0.9,
            shininess:        200.0,
            reflect:          0.0,
            transparency:     0.0,
            refractive_index: 1.0,
        }
    }
}

impl Material {
    pub fn new(
        colour:           Colour, 
        pattern:          Option<Arc<dyn Pattern>>,
        ambient:          f64, 
        diffuse:          f64, 
        specular:         f64, 
        shininess:        f64, 
        reflect:          f64,
        transparency:     f64,
        refractive_index: f64,
        
    ) -> Self {
        Self {
            colour,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
            reflect,
            transparency,
            refractive_index,
        }
    }

    pub fn glass() -> Material {
        Material {
            colour:           Colour::new(1.0, 1.0, 1.0),
            pattern:          None,
            ambient:          0.0,
            diffuse:          0.0,
            specular:         0.0,
            shininess:        00.0,
            reflect:          0.0,
            transparency:     1.0,
            refractive_index: 1.52,
        }
    }

    pub fn metal(colour: Colour, pattern: Option<Arc<dyn Pattern>>) -> Material {
        Material {
            colour,
            pattern,
            ambient:          0.0,
            diffuse:          0.0,
            specular:         1.0,
            shininess:        200.0,
            reflect:          1.0,
            transparency:     0.0,
            refractive_index: 1.0,
        }
    }

    pub fn plastic(colour: Colour, pattern: Option<Arc<dyn Pattern>>) -> Material{
        Material {
            colour,
            pattern,
            ambient:          0.0,
            diffuse:          0.5,
            specular:         0.5,
            shininess:        100.0,
            reflect:          0.0,
            transparency:     0.0,
            refractive_index: 1.0,
        }
    }

    pub fn light(&self, light: &Light, hit: &Intersection, in_shadow: bool) -> Colour {
        let effective_colour = hit.colour * light.intensity;
        let ambient = effective_colour * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_direction = (light.position - hit.point).normalize();
        let light_dot_normal = light_direction.dot(&hit.normal);    // THIS IS ALWAYS NEGATIVE
        let (diffuse, specular) = if light_dot_normal < 0.0 {
            // Light is on the other side of the surface.
            (BLACK, BLACK)
        } else { 
              
            let diffuse = effective_colour * self.diffuse * light_dot_normal;

            let reflect_direction = reflect(&(-light_direction), &hit.normal);
            let reflect_dot_eye = reflect_direction.dot(&hit.eye);

            let specular = if reflect_dot_eye <= 0.0 {
                BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular
    }

    pub fn colour_at(&self, point: &Point3, inverse: &Matrix4) -> Colour {
        if let Some(pattern) = &self.pattern {
            pattern.colour_at(point, inverse)
        } else {
            self.colour
        }
    }
}
