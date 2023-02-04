use std::sync::Arc;
use crate::colour::{Colour, BLACK};
use crate::{Intersection, Matrix4, Point3};
use crate::light::Light;
use crate::math::reflect;
use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Material {
    colour:         Colour,
    pattern:        Option<Arc<dyn Pattern>>,
    ambient:        f64,
    diffuse:        f64,
    specular:       f64,
    shininess:      f64,
    reflectiveness: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            colour:         Colour::new(1.0, 1.0, 1.0),
            pattern:        None,
            ambient:        0.1,
            diffuse:        0.9,
            specular:       0.9,
            shininess:      200.0,
            reflectiveness: 0.0,
        }
    }
}

impl Material {
    pub fn new(
        colour:         Colour, 
        pattern:        Option<Arc<dyn Pattern>>,
        ambient:        f64, 
        diffuse:        f64, 
        specular:       f64, 
        shininess:      f64, 
        reflectiveness: f64
    ) -> Self {
        Self {
            colour,
            pattern,
            ambient,
            diffuse,
            specular,
            shininess,
            reflectiveness,
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

    pub fn reflectiveness(&self) -> f64 {
        self.reflectiveness
    }

    pub fn colour_at(&self, point: &Point3, inverse: &Matrix4) -> Colour {
        if let Some(pattern) = &self.pattern {
            pattern.colour_at(point, inverse)
        } else {
            self.colour
        }
    }
}
