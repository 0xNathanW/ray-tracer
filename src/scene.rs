use crate::colour::BLACK;
use crate::{Colour, Point3, Material};
use crate::object::Object;
use crate::intersection::{Intersection, compute_intersections};
use crate::ray::Ray;
use crate::light::Light;

#[derive(Default, Debug)]
pub struct Scene {
    pub objects:    Vec<Box<dyn Object>>,
    pub lights:     Vec<Light>,
    pub background: Colour,
    pub id_counter: usize,
}

impl Scene {

    pub fn new(mut objects: Vec<Box<dyn Object>>, lights: Vec<Light>, bg: Colour) -> Self {
        let mut id_counter = 0;
        for obj in &mut objects {
            obj.set_id(id_counter);
            id_counter += 1;
        }
        Self { objects, lights, id_counter, background: bg }
    }

    pub fn push(&mut self, mut object: Box<dyn Object>) {
        object.set_id(self.id_counter);
        self.id_counter += 1;
        self.objects.push(object);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Object>> {
        self.objects.pop()
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Vec<Intersection> {
        self.objects.iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .flatten()
            .collect()
    }

    pub fn colour_at(&self, ray: &Ray, depth: usize) -> Colour {

        let mut hits = self.hit(&ray, -0.0001, f64::INFINITY);
        if hits.is_empty() { return self.background; }

        compute_intersections(&mut hits);
        // TODO: doesnt need to be an iterator.
        if let Some(hit) = hits.first() {
            return hit.colour;
            let in_shadow = self.is_shadowed(&hit.over_point);

            let surface_colour = hit.material.light(&self.lights[0], &hit, in_shadow);
            let reflected_colour = self.reflected_colour_at(&hit.material, &hit, depth);
            let refracted_colour = self.refracted_colour_at(&hit.material, &hit, depth);
            // println!("{:?} {:?} {:?}", surface_colour, reflected_colour, refracted_colour);
            if hit.material.reflect > 0.0 && hit.material.transparency > 0.0 {
                let reflectance = hit.schlick();
                return surface_colour + reflected_colour * reflectance + refracted_colour * (1.0 - reflectance);
            } else {
                return surface_colour + reflected_colour + refracted_colour;
            }
        }
        
        self.background
    }

    fn reflected_colour_at(&self, material: &Material, hit: &Intersection, depth: usize) -> Colour {
        if depth == 0 || material.reflect == 0.0 {
            return BLACK;
        }
        let reflected = Ray::new(hit.over_point, hit.reflect);
        self.colour_at(&reflected, depth - 1) * material.reflect      
    }

    fn refracted_colour_at(&self, material: &Material, hit: &Intersection, depth: usize) -> Colour {
        // Material is opaque/max depth.
        if material.transparency == 0.0 || depth == 0 {
            return BLACK;
        }

        // n1 = exited, n2 = entered.
        let idx_ratio = hit.exit_idx / hit.enter_idx;
        let cos_i = hit.eye.dot(&hit.normal);
        let sin2_t = idx_ratio.powi(2) * (1.0 - cos_i.powi(2));

        // Total internal reflection.
        if sin2_t > 1.0 {
            return BLACK;
        }
        
        let cost_t = (1.0 - sin2_t).sqrt();
        let direction = hit.normal * (idx_ratio * cos_i - cost_t) - hit.eye * idx_ratio;
        let refracted = Ray::new(hit.under_point, direction);

        self.colour_at(&refracted, depth - 1) * material.transparency
    }

    fn is_shadowed(&self, point: &Point3) -> bool {
        let shadow_vec = self.lights[0].position - point;
        
        let distance = shadow_vec.magnitude();
        let direction = shadow_vec.normalize();

        let shadow_ray = Ray::new(*point, direction);
        let hits = self.hit(&shadow_ray, 0.0001, f64::INFINITY);
        
        if let Some(hit) = hits.first() {
            hit.t < distance
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::pattern::MockPattern;
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
        let hit_rec = &scene.hit(&ray, 0.0001, f64::INFINITY)[0];
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
        let hit_rec = &scene.hit(&ray, 0.0001, f64::INFINITY)[0];
        let colour = scene.reflected_colour_at(scene.objects[1].material(), hit_rec, 1);
        // 0.5 reflectiveness so should be half the colour of the light.
        assert!(fuzzy_eq_colour(colour, Colour::new(0.19032, 0.2379, 0.14274)));
    }

    #[test]
    fn test_refraction_opaque() {
        let mut scene = Scene::default();
        scene.push(Box::new(default_sphere()));

        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let mut intersections = scene.hit(&ray, 0.0001, f64::INFINITY);
        compute_intersections(&mut intersections);
        let hit = &intersections[0];
        let colour = scene.refracted_colour_at(&hit.material, hit, 5);
        assert_eq!(colour, BLACK);
    }

    #[test]
    fn test_refraction_max_depth() {
        let mut scene = Scene::default();
        scene.push(Box::new(default_sphere()));

        let ray = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let mut intersections = scene.hit(&ray, 0.0001, f64::INFINITY);
        compute_intersections(&mut intersections);
        let hit = &intersections[0];
        let colour = scene.refracted_colour_at(&hit.material, hit, 0);
        assert_eq!(colour, BLACK);
    }

    #[test]
    fn test_refracted() {
        let mut scene = Scene::default();
        let sphere = Sphere::new(Material {
            colour: Colour::new(0.8, 1.0, 0.6),
            ambient: 1.0,
            diffuse: 0.7,
            specular: 0.2,
            pattern: Some(Arc::new(MockPattern::new())),
            ..Default::default()
        });
        scene.push(Box::new(sphere));

        let mut sphere2 = Sphere::new(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Default::default()
        });
        sphere2.scale_uniform(0.5);
        scene.push(Box::new(sphere2));

        scene.lights.push(default_light());

        let ray = Ray::new(Point3::new(0.0, 0.0, 0.1), Vec3::new(0.0, 1.0, 0.0));
        let mut intersections = scene.hit(&ray, -f64::INFINITY, f64::INFINITY);    
        compute_intersections(&mut intersections);
        let hit = &intersections[2];
        let colour = scene.refracted_colour_at(&hit.material, hit, 5);
        assert!(fuzzy_eq_colour(colour, Colour::new(0.0, 0.99888, 0.04725)));
    }

    #[test]
    fn test_colour_at() {
        let mut scene = Scene::default();
        let sphere1 = default_sphere();
        let mut sphere2 = Sphere::new(Material::default());
        sphere2.scale_uniform(0.5);
        scene.push(Box::new(sphere1));
        scene.push(Box::new(sphere2));

        let mut plane = Plane::new(Material {
            transparency: 0.5,
            refractive_index: 1.5,
            ..Default::default()
        });
        plane.translate(0.0, -1.0, 0.0);
        scene.push(Box::new(plane));

        let mut sphere3 = Sphere::new(Material {
            colour: Colour::new(1.0, 0.0, 0.0),
            ambient: 0.5,
            ..Default::default()
        });
        sphere3.translate(0.0, -3.5, -0.5);
        scene.push(Box::new(sphere3));

        scene.lights.push(default_light());

        let ray = Ray::new(Point3::new(0.0, 0.0, -3.0), Vec3::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0));
        let colour = scene.colour_at(&ray, 5);
        assert!(fuzzy_eq_colour(colour, Colour::new(0.93642, 0.68642, 0.68642)))
    }
}
