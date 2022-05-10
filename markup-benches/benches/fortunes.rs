use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench);
criterion_main!(benches);

#[path = "../../markup/examples/fortunes.rs"]
mod fortunes;

fn bench(c: &mut Criterion) {
    let data = fortunes::FORTUNES;

    let mut group = c.benchmark_group("fortunes");

    group.throughput(Throughput::Bytes(
        fortunes::new(data).to_string().len() as u64
    ));

    group.bench_function("define", |b| {
        b.iter(|| fortunes::Define { fortunes: data }.to_string())
    });

    group.bench_function("new", |b| b.iter(|| fortunes::new(data).to_string()));

    group.finish();
}
