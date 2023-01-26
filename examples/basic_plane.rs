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
use ray_tracer::material::{Lambertian, Dielectric};

fn main() {
    let dimensions = (1500, 862);

    let scene = plane_scene();
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 10.0),
        Point3::new(20.0, 20.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        30.0,
        dimensions,
        0.0,
        10.0,
    );

    let image = render(scene, camera, dimensions, 500, 500);
    write_to_file("plane", image, ray_tracer::OutputFormat::PNG, dimensions).unwrap();
}

fn plane_scene() -> Scene {
    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    
    let plane_material = Arc::new(Lambertian::new(colour::BLUE));
    objects.push(Box::new(Plane::new(
        Point3::new(20.0, 20.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        // 5.0,
        plane_material,
    )));

    let disk_material = Arc::new(Dielectric::new(1.5));
    objects.push(Box::new(Disk::new(
        Point3::new(20.0, 20.0, 4.0),
        Vec3::new(0.0, 0.0, 1.0),
        5.0,
        disk_material,
    )));

    let sphere_material = Arc::new(Lambertian::new(colour::RED));
    objects.push(Box::new(Sphere::new(
        Point3::new(20.0, 20.0, 2.0),
        2.0,
        sphere_material,
    )));

    Scene::new(objects)
}
