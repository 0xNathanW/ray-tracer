use std::sync::Arc;
use ray_tracer::{
    colour,
    Camera,
    Point3,
    Vec3,
    Scene,
    Object,
    render,
    write_to_file,
};
use ray_tracer::object::{Sphere, Plane, Disk};
use ray_tracer::material::Lambertian;

fn main() {
    let dimensions = (1500, 862);

    let scene = plane_scene();
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 8.0),
        Point3::new(30.0, 30.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        30.0,
        dimensions,
        0.0,
        10.0,
    );

    let image = render(Arc::new(scene), camera, dimensions, 500, 500);
    write_to_file("plane", image, ray_tracer::OutputFormat::PNG, dimensions).unwrap();
}

fn plane_scene() -> Scene {
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    
    let plane = Box::new(Plane::new(Arc::new(Lambertian::new(colour::GREEN))));
    objects.push(plane);

    let mut sphere = Box::new(Sphere::new(Arc::new(Lambertian::new(colour::BLUE))));
    sphere.translate(30.0, 30.0, 2.0);
    sphere.scale_uniform(2.0);
    objects.push(sphere);

    Scene::new(objects)
}
