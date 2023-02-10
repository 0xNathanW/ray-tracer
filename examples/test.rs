use ray_tracer::*;

fn main() {
    let dimensions = (1920, 1080);
    let (scene, camera) = parse_scene("cone/box.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 100, 50);
    write_to_file("box", image, OutputFormat::PNG, dimensions).unwrap();
}