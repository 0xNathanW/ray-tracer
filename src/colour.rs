use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::object::Object;

pub type Colour = Vec3;

const RED: Colour    = Colour { x: 1.0, y: 0.0, z: 0.0 };
const GREEN: Colour  = Colour { x: 0.0, y: 1.0, z: 0.0 };
const BLUE: Colour   = Colour { x: 0.0, y: 0.0, z: 1.0 };
const WHITE: Colour  = Colour { x: 1.0, y: 1.0, z: 1.0 };
const BLACK: Colour  = Colour { x: 0.0, y: 0.0, z: 0.0 };
const PINK: Colour   = Colour { x: 1.0, y: 0.0, z: 1.0 };
const YELLOW: Colour = Colour { x: 1.0, y: 1.0, z: 0.0 };
const CYAN: Colour   = Colour { x: 0.0, y: 1.0, z: 1.0 };
const ORANGE: Colour = Colour { x: 1.0, y: 0.5, z: 0.0 };

impl Colour {
    pub fn gamma_correct(&mut self, samples: u32) {
        let scale = 1.0 / (samples as f64);
        self.x = (self.x * scale).sqrt();
        self.y = (self.y * scale).sqrt();
        self.z = (self.z * scale).sqrt();
    }
}

impl Into<Vec<u8>> for Colour {
    fn into(self) -> Vec<u8> {
        vec![
            (256.0 * self.x.clamp(0.0, 0.999)) as u8,
            (256.0 * self.y.clamp(0.0, 0.999)) as u8,
            (256.0 * self.z.clamp(0.0, 0.999)) as u8,
        ]
    }
}

pub fn ray_colour(ray: &Ray, obj: &dyn Object, depth: usize) -> Colour {
        
    if depth == 0 {
        return Colour::default();
    }

    if let Some(hit_rec) = obj.hit(ray, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Colour::default();
    
        if hit_rec.material.scatter(ray, &hit_rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_colour(&scattered, obj, depth - 1);
        } else {
            return Colour::default();
        }
    
    } else {
        // Background colour.
        let unit_direction = ray.direction.normalise();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

