use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(pixel_colour: Colour, num_samples: usize) {
    
    // Divide colour by number of samples.
    let scale = 1.0 / (num_samples as f64);
   
    let r = (pixel_colour.x * scale).sqrt();
    let g = (pixel_colour.y * scale).sqrt();
    let b = (pixel_colour.z * scale).sqrt();
    
    println!("{} {} {}",
    (256.0 * r.clamp(0.0, 0.999)) as u32, 
    (256.0 * g.clamp(0.0, 0.999)) as u32, 
    (256.0 * b.clamp(0.0, 0.999)) as u32,
    );
}