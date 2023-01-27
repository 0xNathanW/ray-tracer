use criterion::{Criterion, criterion_main, criterion_group};
use ray_tracer::{parse_scene, render, default_dims};

fn bench_spheres(c: &mut Criterion) {
    let dimensions = default_dims();
    let samples = 300;
    let max_depth = 100;
    let scene_path = "scenes/bench_spheres.yaml";
    let (scene, camera) = parse_scene(scene_path, dimensions).unwrap();

    let mut group = c.benchmark_group("spheres");
    group.sample_size(10);
    group.bench_function("spheres", |b| b.iter(
        || render(
            scene.clone(), camera.clone(), dimensions, samples, max_depth
        )));
    group.finish();
}

criterion_group!(benches, bench_spheres);
criterion_main!(benches);