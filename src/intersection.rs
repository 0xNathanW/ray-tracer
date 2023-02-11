use std::{sync::Arc, collections::HashMap};
use crate::{Point3, Vec3, Material, Colour};

#[derive(Debug, Default)]
pub struct Intersection {
    // Intersection ID.
    pub id: usize,
    // The ID of the object that was hit.
    pub obj_id: usize,
    // The point at which the ray hit the object.
    pub point: Point3,
    // The normal of the object at the point of incidence.
    pub normal: Vec3,
    // Material will be shared between threads.
    pub material: Arc<Material>,
    // Hit only if t is t_min < t < t_max.
    pub t: f64,
    // True if the ray hit the front of the object.
    pub front_face: bool,
    // Direction to camera.
    pub eye: Vec3,
    // Reflection vector.
    pub reflect: Vec3,
    // Colour of material/pattern.
    pub colour: Colour,
    // Point slightly above the surface.
    pub over_point: Point3,
    // Point slightly below the surface.
    pub under_point: Point3,
    // Exit index of refraction.
    pub exit_idx: f64,
    // Enter index of refraction.
    pub enter_idx: f64,
}

impl Intersection {
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eye.dot(&self.normal);
        if self.exit_idx > self.enter_idx {
            let n = self.exit_idx / self.enter_idx;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((self.enter_idx - self.exit_idx) / (self.enter_idx + self.exit_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

pub fn compute_intersections(hits: &mut Vec<Intersection>) {
    
    hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    hits.iter_mut().enumerate()
        .for_each(|(i, hit)| hit.id = i);

    let r_map: HashMap<usize, f64> = hits.iter()
        .map(|i| (i.obj_id, i.material.refractive_index))
        .collect();
    
    // TODO: Make container resusable 2 avoid allocation.
    let refractions = hits.iter()
        .map(|hit| {
            
            let mut exit_idx = 1.0;
            let mut enter_idx = 1.0;
            let mut containers: Vec<usize> = Vec::new();

            for i in 0..hits.len() {
                let other = &hits[i];

                if other.id == hit.id {
                    if !containers.is_empty() {
                        exit_idx = r_map[&containers[containers.len() - 1]];
                    } else {
                        exit_idx = 1.0;
                    }
                }

                if containers.contains(&other.obj_id) {
                    containers.retain(|&x| x != other.obj_id);
                } else {
                    containers.push(other.obj_id);
                }

                if other.id == hit.id {
                    if !containers.is_empty() {
                        enter_idx = r_map[&containers[containers.len() - 1]];
                    } else {
                        enter_idx = 1.0;
                    }
                    break;
                }
            }

            (exit_idx, enter_idx)
        })
        .collect::<Vec<(f64, f64)>>();
    
    // Apply refraction indices to hits.
    hits.iter_mut()
        .zip(refractions.iter())
        .for_each(|(hit, (exit_idx, enter_idx))| {
            hit.exit_idx = *exit_idx;
            hit.enter_idx = *enter_idx;
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::*;
    use crate::{Material, ray::Ray, scene::Scene, object::Sphere, transform::Transformable};

    #[test]
    fn test_refraction() {
        let mut scene = Scene::default();
        // 2 glass spheres inside a larger glass sphere.
        let mut outer_sphere = Sphere::new(Material {
            colour: Colour::default(),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.0,
            specular: 0.0,
            shininess: 0.0,
            reflect: 0.0,
            refractive_index: 1.5,
            transparency: 1.0,
        });
        outer_sphere.scale_uniform(2.0);
        let mut inner_sphere1 = Sphere::new(Material::new(
            Colour::default(), 
            None, 
            0.1,
            0.0, 
            0.0, 
            0.0, 
            0.0, 
            1.0, 
            2.0,
        ));
        inner_sphere1.translate(0.0, 0.0, -0.25);
        let mut inner_sphere2 = Sphere::new(Material::new(
            Colour::default(), 
            None, 
            0.1,
            0.0, 
            0.0, 
            0.0, 
            0.0, 
            1.0, 
            2.5,
        ));
        inner_sphere2.translate(0.0, 0.0, 0.25);

        scene.push(Box::new(outer_sphere));
        scene.push(Box::new(inner_sphere1));
        scene.push(Box::new(inner_sphere2));

        let ray = Ray::new(Point3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 0.0, 1.0));
        let mut hits = scene.hit(&ray, 0.001, f64::INFINITY);
        assert_eq!(hits.len(), 6);
        compute_intersections(&mut hits);
        
        assert_eq!(hits[0].exit_idx, 1.0);
        assert_eq!(hits[0].enter_idx, 1.5);
        
        assert_eq!(hits[1].exit_idx, 1.5);
        assert_eq!(hits[1].enter_idx, 2.0);

        assert_eq!(hits[2].exit_idx, 2.0);
        assert_eq!(hits[2].enter_idx, 2.5);

        assert_eq!(hits[3].exit_idx, 2.5);
        assert_eq!(hits[3].enter_idx, 2.5);

        assert_eq!(hits[4].exit_idx, 2.5);
        assert_eq!(hits[4].enter_idx, 1.5);

        assert_eq!(hits[5].exit_idx, 1.5);
        assert_eq!(hits[5].enter_idx, 1.0);
    }

    #[test]
    fn test_reflectance_perp() {
        let mut scene = Scene::default();
        let sphere = Sphere::new(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Material::default()
        });
        scene.push(Box::new(sphere));

        let ray = Ray::new(Point3::origin(), Vec3::new(0.0, 1.0, 0.0));
        let mut hits = scene.hit(&ray, -f64::INFINITY, f64::INFINITY);
        compute_intersections(&mut hits);
        hits.iter().for_each(|hit| {
            println!("reflectance: {:?}", hit);
        });
    }

    #[test]
    fn test_reflectance_small_angle() {
        let mut scene = Scene::default();
        let sphere = Sphere::new(Material {
            transparency: 1.0,
            refractive_index: 1.5,
            ..Material::default()
        });
        scene.push(Box::new(sphere));

        let ray = Ray::new(Point3::origin(), Vec3::new(0.0, 0.99, -0.1));
        let mut hits = scene.hit(&ray, -f64::INFINITY, f64::INFINITY);
        compute_intersections(&mut hits);
        assert!(fuzzy_eq_f64(hits[1].schlick(), 0.04));

        // n2 > n1
        let ray = Ray::new(Point3::new(0.0, 0.99, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let mut hits = scene.hit(&ray, -f64::INFINITY, f64::INFINITY);
        compute_intersections(&mut hits);
        assert!(fuzzy_eq_f64(hits[0].schlick(), 0.48873));
    }

}