use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_dynamic);
criterion_main!(benches);

#[path = "../../markup/examples/fortunes.rs"]
mod fortunes;

fn bench_dynamic(c: &mut Criterion) {
    let data = fortunes::FORTUNES;

    let mut group = c.benchmark_group("macros");

    group.throughput(Throughput::Bytes(
        fortunes::dynamic(&data).to_string().len() as u64,
    ));

    group.bench_function("define", |b| {
        b.iter(|| fortunes::Define { fortunes: &data }.to_string())
    });

    group.bench_function("dynamic", |b| {
        b.iter(|| fortunes::dynamic(&data).to_string())
    });

    group.finish();
}
