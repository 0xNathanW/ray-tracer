use ray_tracer::*;

fn main() {
    let dimensions = default_dims();
    let (scene, camera) = parse_scene("scenes/sphere_plane.yaml", dimensions).unwrap();
    let image = render(scene, camera, dimensions, 1, 0);
    write_to_file("sphere_plane", image, OutputFormat::PNG, dimensions).unwrap();
}