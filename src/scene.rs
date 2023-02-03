use crate::colour::BLACK;
use crate::{Colour, Point3, Material};
use crate::object::{Object, Intersection};
use crate::ray::Ray;
use crate::light::Light;

#[derive(Default, Debug)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights:  Vec<Light>,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<Light>) -> Self {
        Self {
            objects,
            lights,
        }
    }

    pub fn hit(&self, ray: &Ray) -> Vec<Intersection> {
        self.objects.iter()
            .filter_map(|obj| obj.hit(ray, 0.001, f64::INFINITY))
            .collect()
    }

    pub fn colour_at(&self, ray: &Ray, depth: usize) -> Colour {

        let mut hits = self.hit(&ray);
        hits.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        
        for hit in hits {
            let in_shadow = self.is_shadowed(&hit.over_point);
            let surface_colour = hit.material.light(&self.lights[0], &hit, in_shadow);
            let reflected_colour = self.reflected_colour_at(&hit.material, &hit, depth);
            return surface_colour + reflected_colour;
        }
        
        // Background colour.
        BLACK
        // let unit_direction = ray.direction.normalize();
        // let t = 0.5 * (unit_direction.z + 1.0);
        // (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }

    fn reflected_colour_at(&self, material: &Material, hit: &Intersection, depth: usize) -> Colour {
        if depth == 0 || material.reflectiveness() == 0.0 {
            return BLACK;
        }

        let reflected = Ray::new(hit.over_point, hit.reflect);
        self.colour_at(&reflected, depth - 1) * material.reflectiveness()
    }

    fn is_shadowed(&self, point: &Point3) -> bool {
        let shadow_vec = self.lights[0].position - point;
        
        let distance = shadow_vec.magnitude();
        let direction = shadow_vec.normalize();

        let shadow_ray = Ray::new(*point, direction);
        let hits = self.hit(&shadow_ray);
        if !hits.is_empty() {
            let hit = &hits[0];
            if hit.t < distance {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
