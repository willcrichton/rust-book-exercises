use criterion::{black_box, criterion_group, criterion_main, Criterion};
use week1::p2_vec::{baseline, vectorized};

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("baseline", |b| {
    b.iter(|| baseline(black_box([1., 2., 3., 4.]), black_box([0., 5., 1., 2.])))
  });
  c.bench_function("vectorized", |b| {
    b.iter(|| vectorized(black_box([1., 2., 3., 4.]), black_box([0., 5., 1., 2.])))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
