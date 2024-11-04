use criterion::{ black_box, criterion_group, criterion_main, Criterion };

fn benchmark_simple(c: &mut Criterion) {
    c.bench_function("DeSer::Ours", |b| {
        b.iter(
            || {
                // black_box(BookUpdate::deserialize_from(&buf, 0).unwrap());
            }
        );
    });
}

criterion_group!(benches, benchmark_simple);

criterion_main!(benches);
