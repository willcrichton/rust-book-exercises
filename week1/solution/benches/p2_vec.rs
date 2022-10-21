use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;
use week1::p2_vec::{baseline, vectorized};

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let inputs = (0..100)
        .map(|_| {
            [
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                rng.gen::<f64>(),
            ]
        })
        .collect::<Vec<_>>();
    c.bench_function("baseline", |b| {
        b.iter(|| {
            for i in 0..99 {
                baseline(inputs[i], inputs[i + 1]);
            }
        })
    });
    c.bench_function("vectorized", |b| {
        b.iter(|| {
            for i in 0..99 {
                vectorized(inputs[i], inputs[i + 1]);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
