use ray_tracer::*;

fn main() {
    let dimensions = default_dims();
    let (scene, camera) = parse_scene("scenes/single_sphere.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 100, 0);
    write_to_file("single_sphere", image, OutputFormat::PNG, dimensions).unwrap();
}