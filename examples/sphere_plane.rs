use ray_tracer::*;

fn main() {
    let dimensions = default_dims();
    let (scene, camera) = parse_scene("scenes/sphere_plane.yaml", dimensions).unwrap();
    println!("Scene: {:#?}", scene);
    let image = render(scene, camera, dimensions, 100, 0);
    write_to_file("sphere_plane", image, OutputFormat::PNG, dimensions).unwrap();
}