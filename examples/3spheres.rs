use ray_tracer::*;

fn main() {
    let dimensions = (1920, 1080);
    let (scene, camera) = parse_scene("scenes/3spheres.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 10, 30);
    write_to_file("3spheres", image, OutputFormat::PNG, dimensions).unwrap();
}