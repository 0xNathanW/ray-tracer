use std::sync::Arc;
use std::ops::Deref;
use rand::rngs::ThreadRng;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::Camera;
use crate::Scene;
use crate::ray::Ray;
use crate::colour::Colour;
use crate::Object;

pub type Image = Vec<Vec<u8>>;

pub fn render(
    scene: Scene,
    camera: Camera,
    dimensions: (u32, u32),
    samples_per_pixel: u32,
    max_depth: u32,
) -> Image {

    let scene = Arc::new(scene);

    println!();
    let progress_bar = ProgressBar::new(dimensions.1 as u64)
        .with_message("Progress");

    progress_bar
        .set_style(ProgressStyle::with_template("{spinner:.green} {msg} [{elapsed_precise}] [{bar:100.cyan/blue}] {pos}/{len} Lines rendered (ETA: {eta})")
        .unwrap()
        .progress_chars("#>-")
    );

    let pixels = (0..dimensions.1)
    .into_par_iter()
    .rev()
    .map(|j| {

        let mut rng = rand::thread_rng();
        let scene = Arc::clone(&scene);
        let mut row = vec![0; 3 * dimensions.0 as usize];
        for i in 0..dimensions.0 {
        
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                // Randomise the sample point within the pixel.
                let u = (i as f64 + rand::random::<f64>()) / (dimensions.0 - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (dimensions.1 - 1) as f64;
                let ray = camera.get_ray(u, v, &mut rng);
                pixel_colour += ray_colour(&ray, scene.deref(), max_depth as usize, &mut rng);
            }
            
            pixel_colour.gamma_correct(samples_per_pixel);

            let rgb: Vec<u8> = pixel_colour.into();
            row[i as usize * 3..i as usize * 3 + 3].copy_from_slice(&rgb);
        }

        progress_bar.inc(1);
        row
    }).collect::<Image>();
    
    let time_taken = progress_bar.elapsed();
    progress_bar.finish_with_message("Done");
    println!("Finished rendering in {} seconds.", time_taken.as_secs_f64());
    pixels
}

pub fn ray_colour(ray: &Ray, obj: &dyn Object, depth: usize, rng: &mut ThreadRng) -> Colour {
        
    if depth == 0 {
        return Colour::default();
    }

    if let Some(hit_rec) = obj.hit(ray, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Colour::default();
    
        if hit_rec.material.scatter(ray, &hit_rec, &mut attenuation, &mut scattered, rng) {
            attenuation * ray_colour(&scattered, obj, depth - 1, rng)
        } else {
            Colour::default()
        }
    
    } else {
        // Background colour.
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}