use criterion::{criterion_group, criterion_main, Criterion, Throughput};

criterion_group!(benches, bench_dynamic);
criterion_main!(benches);

markup::define! {
    Struct<'a>(data: &'a [usize]) {
        foo {
            @for d in *data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

fn dynamic<'a>(data: &'a [usize]) -> impl markup::Render + std::fmt::Display + 'a {
    markup::dynamic! {
        foo {
            @for d in data {
                bar { "Hello" @d "World" }
            }
        }
    }
}

fn bench_dynamic(c: &mut Criterion) {
    let data = (1..1000).into_iter().collect::<Vec<_>>();
    let mut group = c.benchmark_group("dynamic");
    let len1 = Struct { data: &data }.to_string().len();
    assert_eq!(len1, 23879);
    let len2 = dynamic(&data).to_string().len();
    assert_eq!(len1, len2);
    group.throughput(Throughput::Bytes(len1 as u64));
    group.bench_function("struct", |b| b.iter(|| Struct { data: &data }.to_string()));
    group.bench_function("dynamic", |b| b.iter(|| dynamic(&data).to_string()));
    group.finish();
}
