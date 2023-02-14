use ray_tracer::*;

fn main() {
    let dimensions = (1920, 1080);
    let (scene, camera) = parse_scene("scenes/examples/3spheres.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 100, 100);
    write_to_file("renders/3spheres", image, OutputFormat::PNG, dimensions).unwrap();
}