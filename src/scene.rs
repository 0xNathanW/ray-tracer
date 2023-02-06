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
        Self { objects, lights }
    }

    pub fn push(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Object>> {
        self.objects.pop()
    }

    pub fn hit(&self, ray: &Ray) -> Vec<Intersection> {
        self.objects.iter()
            .filter_map(|obj| obj.hit(ray, 0.001, f64::INFINITY))
            .flatten()
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
        if depth == 0 || material.reflect() == 0.0 {
            return BLACK;
        }

        let reflected = Ray::new(hit.over_point, hit.reflect);
        self.colour_at(&reflected, depth - 1) * material.reflect()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray::Ray, Vec3, colour::fuzzy_eq_colour};
    use crate::object::{Sphere, Plane};
    use crate::material::Material;
    use crate::transform::Transformable;

    fn default_sphere() -> Sphere {
        Sphere::new(
            Material::new(
                Colour::new(0.8, 1.0, 0.6), 
                None, 
                0.1, 
                0.7, 
                0.2, 
                200.0, 
                0.0,
                0.0,
                1.0,
            )
        )
    }

    fn default_light() -> Light {
        Light::new(
            Point3::new(-10.0, 10.0, -10.0),
            Colour::new(1.0, 1.0, 1.0),
        )
    }

    #[test]
    fn test_nonreflective_colour() {
        let mut scene = Scene::default();
        scene.push(Box::new(default_sphere()));

        let mut sphere2 = Sphere::new(
            Material::new(
                Colour::new(0.8, 1.0, 0.6), 
                None, 
                1.0, 
                0.7, 
                0.2, 
                200.0, 
                0.0,
                0.0,
                1.0,
        ));
        sphere2.scale_uniform(0.5);
        scene.push(Box::new(sphere2));

        scene.lights.push(default_light());

        let ray = Ray::new(Point3::origin(), Vec3::new(0.0, 0.0, 1.0));
        let hit_rec = &scene.hit(&ray)[0];
        let colour = scene.reflected_colour_at(scene.objects[1].material(), hit_rec, 1);
        assert_eq!(colour, Colour::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_reflective_material() {
        let mut scene = Scene::default();
        scene.push(Box::new(default_sphere()));

        let mut plane = Plane::new(Material::new(
            Colour::new(0.8, 1.0, 0.6), 
            None, 
            0.1, 
            0.7, 
            0.2, 
            200.0,
            0.5,
            0.0,
            1.0,
        ));
        plane.translate(0.0, -1.0, 0.0);
        scene.push(Box::new(plane));

        scene.lights.push(default_light());
        
        let ray = Ray::new(Point3::new(0.0, 0.0, -3.0), Vec3::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));
        let hit_rec = &scene.hit(&ray)[0];
        let colour = scene.reflected_colour_at(scene.objects[1].material(), hit_rec, 1);
        // 0.5 reflectiveness so should be half the colour of the light.
        assert!(fuzzy_eq_colour(colour, Colour::new(0.19032, 0.2379, 0.14274)));
    }

    #[test]
    fn test_refraction() {
        let mut scene = Scene::default();
        
        let mut outer_sphere = Sphere::new(Material::glass());
        outer_sphere.scale_uniform(2.0);
        let mut inner_sphere1 = Sphere::new(Material::glass());
        inner_sphere1.translate(0.0, 0.0, -0.25);
        let mut inner_sphere2 = Sphere::new(Material::glass());
        inner_sphere2.translate(0.0, 0.0, 0.25);

        scene.push(Box::new(outer_sphere));
        scene.push(Box::new(inner_sphere1));
        scene.push(Box::new(inner_sphere2));

        let ray = Ray::new(Point3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 0.0, 1.0));
        let mut hits = scene.hit(&ray);
        hits.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        for hit in hits {
            println!("{}", hit.t);

        }
    }
}
