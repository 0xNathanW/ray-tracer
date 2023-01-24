use std::sync::Arc;
use crate::object::{Object, Intersection, Sphere};
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::colour::Colour;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        Self { 
            objects
        }
    }
}

impl Object for Scene {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            // If ray hits object before closest_so_far, update hit_record.
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit = Some(rec);
            }
        }
        hit
    }
}

impl Scene {

    pub fn randomised_scene() -> Scene {
        let mut objects: Vec<Box<dyn Object>> = Vec::new();
        let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = rand::random::<f64>();
                let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                        let sphere_material: Arc<dyn Material> = if choose_material < 0.8 {
                            // Diffuse
                            let albedo = Colour::random() * Colour::random();
                            Arc::new(Lambertian::new(albedo))
                        } else if choose_material < 0.95 {
                            // Metal
                            let albedo = Colour::random_range(0.5, 1.0);
                            let fuzz = rand::random::<f64>() * 0.5;
                            Arc::new(Metal::new(albedo, fuzz))
                        } else {
                            // Glass
                            Arc::new(Dielectric::new(1.5))
                        };

                        objects.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    
                }
            }

            let material1 = Arc::new(Dielectric::new(1.5));
            objects.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

            let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
            objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

            let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
            objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

            Scene::new(objects)
        }
    
}