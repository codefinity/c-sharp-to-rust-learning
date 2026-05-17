// ============================================================
// BENCHMARK: Iterator Chain vs Manual Loop
// ============================================================
// RUN: cargo bench --bench iter_bench
// ============================================================

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_iterator_vs_loop(c: &mut Criterion) {
    let data: Vec<i32> = (0..10_000).collect();

    let mut group = c.benchmark_group("filter_map_sum");

    group.bench_function("iterator_chain", |b| {
        b.iter(|| {
            let s: i32 = data.iter()
                .filter(|&&x| x % 2 == 0)
                .map(|&x| x * x)
                .sum();
            criterion::black_box(s)
        })
    });

    group.bench_function("manual_loop", |b| {
        b.iter(|| {
            let mut s: i32 = 0;
            for &x in &data {
                if x % 2 == 0 {
                    s += x * x;
                }
            }
            criterion::black_box(s)
        })
    });

    group.finish();
}

fn bench_collect_vs_extend(c: &mut Criterion) {
    let source: Vec<i32> = (0..1000).collect();

    let mut group = c.benchmark_group("collect_vs_extend");

    group.bench_function("collect", |b| {
        b.iter(|| {
            let _v: Vec<i32> = source.iter().map(|&x| x * 2).collect();
        })
    });

    group.bench_function("extend_with_capacity", |b| {
        b.iter(|| {
            let mut v: Vec<i32> = Vec::with_capacity(source.len());
            v.extend(source.iter().map(|&x| x * 2));
            v
        })
    });

    group.finish();
}

criterion_group!(benches, bench_iterator_vs_loop, bench_collect_vs_extend);
criterion_main!(benches);
