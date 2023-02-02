use crate::colour::{Colour, BLACK};
use crate::Intersection;
use crate::light::Light;
use crate::math::reflect;

pub struct Material {
    colour:         Colour,
    ambient:        f64,
    diffuse:        f64,
    specular:       f64,
    shininess:      f64,
    reflectiveness: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            colour: Colour::new(1.0, 1.0, 1.0),
            ambient:        0.1,
            diffuse:        0.9,
            specular:       0.9,
            shininess:      200.0,
            reflectiveness: 0.0,
        }
    }
}

impl Material {
    pub fn new(colour: Colour, ambient: f64, diffuse: f64, specular: f64, shininess: f64, reflectiveness: f64) -> Self {
        Self {
            colour,
            ambient,
            diffuse,
            specular,
            shininess,
            reflectiveness,
        }
    }

    pub fn light(&self, light: &Light, hit: &Intersection, in_shadow: bool) -> Colour {

        let effective_colour = self.colour * light.intensity;
        let ambient = effective_colour * self.ambient;

        if in_shadow {
            return ambient;
        }

        let light_direction = (light.position - hit.point).normalize();
        let light_dot_normal = light_direction.dot(&hit.normal);
        
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

    pub fn set_colour(mut self, colour: Colour) -> Self {
        self.colour = colour;
        self
    }

    pub fn set_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn set_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn set_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn set_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn set_reflectiveness(mut self, reflectiveness: f64) -> Self {
        self.reflectiveness = reflectiveness;
        self
    }
}
