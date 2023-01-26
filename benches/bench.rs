use criterion::{Criterion, criterion_main, criterion_group};

fn bench_spheres(_c: &mut Criterion) {
    ()
}

criterion_group!(benches, bench_spheres);
criterion_main!(benches);