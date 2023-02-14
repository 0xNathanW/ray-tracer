use ray_tracer::*;

fn main() {
    let dimensions = (1270, 720);
    let (scene, camera) = parse_scene("scenes/examples/shapes.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 50, 30);
    write_to_file("renders/shapes", image, OutputFormat::PNG, dimensions).unwrap();
}