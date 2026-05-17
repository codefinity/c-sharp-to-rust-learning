// ============================================================
// BENCHMARK: String Building Strategies
// ============================================================
// RUN: cargo bench --bench string_bench
// ============================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_string_concat(c: &mut Criterion) {
    let words: Vec<String> = (0..10).map(|i| format!("word{i}")).collect();

    let mut group = c.benchmark_group("string_building");

    group.bench_function("format_macro", |b| {
        b.iter(|| {
            let _s: String = words.iter()
                .map(|w| w.as_str())
                .collect::<Vec<_>>()
                .join(", ");
        })
    });

    group.bench_function("push_str_with_capacity", |b| {
        b.iter(|| {
            let total_len: usize = words.iter().map(|w| w.len() + 2).sum();
            let mut s = String::with_capacity(total_len);
            for (i, w) in words.iter().enumerate() {
                if i > 0 { s.push_str(", "); }
                s.push_str(w);
            }
            s
        })
    });

    group.bench_function("collect_join", |b| {
        b.iter(|| words.join(", "))
    });

    group.finish();
}

fn bench_string_search(c: &mut Criterion) {
    let haystack = "the quick brown fox jumps over the lazy dog".repeat(100);
    let needle = "lazy";

    let mut group = c.benchmark_group("string_search");

    group.bench_function("contains", |b| {
        b.iter(|| criterion::black_box(haystack.contains(needle)))
    });

    group.bench_function("find", |b| {
        b.iter(|| criterion::black_box(haystack.find(needle)))
    });

    group.bench_function("match_count", |b| {
        b.iter(|| criterion::black_box(haystack.matches(needle).count()))
    });

    group.finish();
}

fn bench_vec_vs_capacity(c: &mut Criterion) {
    let n = 1_000_usize;

    let mut group = c.benchmark_group("vec_allocation");

    for size in [100, 1000, 10_000] {
        group.bench_with_input(BenchmarkId::new("no_capacity", size), &size, |b, &s| {
            b.iter(|| {
                let mut v: Vec<i32> = Vec::new();
                for i in 0..s as i32 { v.push(i); }
                v
            })
        });

        group.bench_with_input(BenchmarkId::new("with_capacity", size), &size, |b, &s| {
            b.iter(|| {
                let mut v: Vec<i32> = Vec::with_capacity(s);
                for i in 0..s as i32 { v.push(i); }
                v
            })
        });
    }

    drop(n);
    group.finish();
}

criterion_group!(benches, bench_string_concat, bench_string_search, bench_vec_vs_capacity);
criterion_main!(benches);
