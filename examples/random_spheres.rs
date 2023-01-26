use std::sync::Arc;
use ray_tracer::{
    Camera,
    Colour,
    Point3,
    Vec3,
    Scene,
    Object,
    Material,
    OutputFormat,
    render,
    write_to_file,
};
use ray_tracer::object::Sphere;
use ray_tracer::material::{Lambertian, Metal, Dielectric};

fn main() {
    let dimensions = (1500, 862);

    let scene = randomised_scene();
    let camera = Camera::new(
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        dimensions,
        0.01,
        10.0,
    );

    let image = render(scene, camera, dimensions, 300, 100);

    write_to_file("random_spheres", image, OutputFormat::PNG, dimensions).unwrap();
}

fn randomised_scene() -> Scene {
    let mut rng = rand::thread_rng();
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    let ground_material = Arc::new(Lambertian::new(ray_tracer::colour::GREEN));
    objects.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand::random::<f64>();
            let center = Point3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {

                let sphere_material: Arc<dyn Material> = if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Colour::new_random(&mut rng) * Colour::new_random(&mut rng);
                    Arc::new(Lambertian::new(albedo))
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Colour::new_random_range(0.5, 1.0, &mut rng);
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

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    objects.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    objects.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    Scene::new(objects)
}