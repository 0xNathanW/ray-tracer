use std::rc::Rc;
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3, Colour};

#[derive(Default)]
pub struct HitRecord {
    // The point at which the ray hit the object.
    pub incidence_point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,

    pub material: Option<Rc<dyn Material>>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

// An object is something that can be hit by a ray.
pub trait Object {
    // Returns true if the ray hits the object.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

#[derive(Default)]
pub struct ObjectList(Vec<Rc<dyn Object>>);

impl ObjectList {
    pub fn add(&mut self, object: Rc<dyn Object>) { self.0.push(object); }

    pub fn clear(&mut self) { self.0.clear(); }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.0 {
            // If ray hits object before closest_so_far, update hit_record.
            if object.hit(ray, t_min, closest_so_far, hit_record) {
                hit_anything = true;
                closest_so_far = hit_record.t;
            }
        }
        hit_anything
    }
}

impl ObjectList {

    pub fn randomised_scene() -> ObjectList {
        let mut scene = ObjectList::default();

        let ground_material = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        scene.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = rand::random::<f64>();
                let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                        let sphere_material: Rc<dyn Material> = if choose_material < 0.8 {
                            // Diffuse
                            let albedo = Colour::random() * Colour::random();
                            Rc::new(Lambertian::new(albedo))
                        } else if choose_material < 0.95 {
                            // Metal
                            let albedo = Colour::random_range(0.5, 1.0);
                            let fuzz = rand::random::<f64>() * 0.5;
                            Rc::new(Metal::new(albedo, fuzz))
                        } else {
                            // Glass
                            Rc::new(Dielectric::new(1.5))
                        };

                        scene.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    
                }
            }

            let material1 = Rc::new(Dielectric::new(1.5));
            scene.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

            let material2 = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
            scene.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

            let material3 = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
            scene.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

            scene
        }
    
}