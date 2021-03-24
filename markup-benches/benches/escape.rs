use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_escape, bench_escape_noop);
criterion_main!(benches);

#[path = "../../markup/src/escape.rs"]
mod escape;

fn bench_escape(c: &mut Criterion) {
    let str = include_str!("./escape.html");
    let mut group = c.benchmark_group("escape");
    group.throughput(Throughput::Bytes(escape(str).len() as u64));
    group.bench_function("escape", |b| b.iter(|| escape(str)));
    group.finish();
}

fn bench_escape_noop(c: &mut Criterion) {
    let string = (0..128)
        .filter(|byte| !b"<>&\"".contains(byte))
        .map(|byte| byte as char)
        .collect::<String>()
        .repeat(100);

    assert_eq!(string.len(), escape(&string).len());

    let mut group = c.benchmark_group("escape_noop");
    group.throughput(Throughput::Bytes(string.len() as u64));
    group.bench_function("escape_noop", |b| b.iter(|| escape(&string)));
    group.finish();
}

fn escape(str: &str) -> String {
    let mut string = String::new();
    escape::escape(str, &mut string).unwrap();
    string
}
