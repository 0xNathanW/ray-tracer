use criterion::{Criterion, criterion_main, criterion_group};
use ray_tracer::{parse_scene, render, default_dims};

fn bench_sphere(c: &mut Criterion) {
    let dimensions = default_dims();
    let samples = 10;
    let max_depth = 10;
    let scene_path = "scenes/tests/bench.yaml";
    let (scene, camera) = parse_scene(scene_path, dimensions).unwrap();

    c.bench_function("spheres", |b| b.iter(|| 
        render(scene.clone(), camera.clone(), dimensions, samples, max_depth)
    ));
}

criterion_group!(benches, bench_sphere);
criterion_main!(benches);