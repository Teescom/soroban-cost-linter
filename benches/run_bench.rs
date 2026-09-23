use criterion::{criterion_group, criterion_main, Criterion};

fn lint_benchmark(c: &mut Criterion) {
    c.bench_function("run linter", |b| b.iter(|| {
        let _ = 1 + 1;
    }));
}

criterion_group!(benches, lint_benchmark);
criterion_main!(benches);
