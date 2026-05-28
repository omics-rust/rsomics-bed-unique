use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rsomics_bed_unique::unique;
use std::io::Cursor;

fn make_fixture(n: usize, dup_rate: usize) -> String {
    (0..n)
        .map(|i| {
            let start = (i / dup_rate) * 100;
            format!("chr1\t{start}\t{}\n", start + 100)
        })
        .collect()
}

fn bench_unique(c: &mut Criterion) {
    let fixture = make_fixture(100_000, 2); // 50% duplicates
    let mut group = c.benchmark_group("bed-unique");
    group.throughput(Throughput::Elements(100_000));
    group.bench_function("unique_100k_50pct_dup", |b| {
        b.iter(|| {
            let mut out = Vec::with_capacity(2 * 1024 * 1024);
            unique(Cursor::new(fixture.as_str()), &mut out).unwrap();
        });
    });
    group.finish();
}

criterion_group!(benches, bench_unique);
criterion_main!(benches);
