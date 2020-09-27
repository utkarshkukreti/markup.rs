use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_escape);
criterion_main!(benches);

#[path = "../src/escape.rs"]
mod escape;

fn bench_escape(c: &mut Criterion) {
    let mut group = c.benchmark_group("escape");
    group.throughput(Throughput::Bytes(run().len() as u64));
    group.bench_function("escape", |b| b.iter(run));
    group.finish();

    fn run() -> String {
        let mut string = String::new();
        escape::escape(include_str!("./escape.html"), &mut string).unwrap();
        string
    }
}
