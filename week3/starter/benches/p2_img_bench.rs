use criterion::{criterion_group, criterion_main, Criterion};
use std::{env, time::Duration};
use week3::p2_img::Image;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("images");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(10));
    group.significance_level(0.01);

    let paths = ["castle.jpg", "vaporwave.jpeg", "landscape.jpeg"];
    for path in paths {
        if let Ok(target) = env::var("BENCH") {
            if !path.contains(&target) {
                continue;
            }
        }

        let img = Image::load(path).unwrap();
        group.bench_function(path, |b| b.iter(|| img.clone().carve()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
