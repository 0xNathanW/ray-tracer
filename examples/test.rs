use ray_tracer::*;

fn main() {
    let dimensions = (1920, 1080);
    let (scene, camera) = parse_scene("scenes/cylinder.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 10, 50);
    write_to_file("test", image, OutputFormat::PNG, dimensions).unwrap();
}