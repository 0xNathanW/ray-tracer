use std::sync::Arc;
use std::ops::Deref;
use rayon::prelude::*;
use indicatif::ProgressBar;
use crate::camera::Camera;
use crate::object::Scene;
use crate::colour::{Colour, ray_colour};

pub type Image = Vec<Vec<u8>>;

pub fn render(
    scene: Arc<Scene>,
    camera: Camera,
    width: u32,
    height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
    progress_bar: Option<ProgressBar>,
) -> Image {

    let pixels = (0..height)
    .into_par_iter()
    .rev()
    .map(|j| {

        let scene = Arc::clone(&scene);
        let mut row = vec![0; 3 * width as usize];
        for i in 0..width {
        
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                // Randomise the sample point within the pixel.
                let u = (i as f64 + rand::random::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, scene.deref(), max_depth as usize);
            }
            
            pixel_colour.gamma_correct(samples_per_pixel);

            let rgb: Vec<u8> = pixel_colour.into();
            row[i as usize * 3..i as usize * 3 + 3].copy_from_slice(&rgb);
        }

        progress_bar.as_ref().map(|pb| pb.inc(1));
        row
    }).collect::<Image>();
    
    progress_bar.as_ref().map(|pb| pb.finish());

    pixels
}