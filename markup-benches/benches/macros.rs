use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_dynamic);
criterion_main!(benches);

#[path = "../../markup/tests/macros.rs"]
mod imp;

fn bench_dynamic(c: &mut Criterion) {
    let data = imp::FORTUNES;

    let mut group = c.benchmark_group("macros");

    group.throughput(Throughput::Bytes(imp::to_string(&data).len() as u64));

    group.bench_function("define", |b| {
        b.iter(|| imp::Define { fortunes: &data }.to_string())
    });

    group.bench_function("dynamic", |b| b.iter(|| imp::dynamic(&data).to_string()));

    group.bench_function("to_string", |b| b.iter(|| imp::to_string(&data)));

    group.bench_function("to_writer", |b| {
        b.iter(|| {
            let mut string = String::new();
            let _ = imp::to_writer(&data, &mut string);
            string
        })
    });

    group.finish();
}
