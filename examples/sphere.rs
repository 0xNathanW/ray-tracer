use ray_tracer::*;

fn main() {
    let dimensions = default_dims();
    let (scene, camera) = parse_scene("scenes/sphere.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 100, 100);
    write_to_file("sphere", image, OutputFormat::PNG, dimensions).unwrap();
}