use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::Camera;
use crate::Scene;
use crate::colour::Colour;

pub type Image = Vec<Vec<u8>>;

pub fn render(
    scene: Arc<Scene>,
    camera: Camera,
    dimensions: (u32, u32),
    samples_per_pixel: u32,
    max_depth: u32,
) -> Image {

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
    .map(|j| {

        let mut rng = if samples_per_pixel > 1 {
            Some(rand::thread_rng())
        } else {
            None
        };
        let scene = Arc::clone(&scene);
        let mut row = vec![0; 3 * dimensions.0 as usize];
        for i in 0..dimensions.0 {
            let mut pixel_colour = Colour::default();
            for _ in 0..samples_per_pixel {
                let ray = camera.get_ray(i, j, rng.as_mut());
                pixel_colour += scene.colour_at(&ray, max_depth as usize);
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
