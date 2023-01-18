use ray_tracer::camera::Camera;
use ray_tracer::object::{HitRecord, Object, ObjectList};
use ray_tracer::vec3::{Colour, Point3, Vec3};
use ray_tracer::colour::write_colour;
use ray_tracer::ray::Ray;

const ASPECT_RATIO: f64      = 3.0 / 2.0;
const IMAGE_WIDTH: u32       = 1200;
const IMAGE_HEIGHT: u32      = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32         = 50;

fn main() {

    let scene = ObjectList::randomised_scene();

    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0
    );

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            
            let mut pixel_colour = Colour::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                // Randomise the sample point within the pixel.
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_colour += ray_colour(&ray, &scene, MAX_DEPTH as usize);
            }
            write_colour(pixel_colour, SAMPLES_PER_PIXEL as usize);

        }
    }
    eprintln!("Done.")
}

fn ray_colour(ray: &Ray, obj: &dyn Object, depth: usize) -> Colour {
    let mut hit_rec = HitRecord::default();
    // Depth limit.
    if depth == 0 { 
        Colour::default() 
    }

    else if obj.hit(ray, 0.001, f64::INFINITY, &mut hit_rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Colour::default();
        if let Some(material) = &hit_rec.material {
            if material.scatter(ray, &hit_rec, &mut attenuation, &mut scattered) {
                attenuation * ray_colour(&scattered, obj, depth - 1)
            } else {
                Colour::default()
            }
        } else {
            Colour::default()
        }

    } else {
        // Background colour.
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
