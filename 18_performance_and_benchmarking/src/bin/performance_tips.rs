// ============================================================
// CONCEPT: Performance Patterns and Profiling Guidance
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# performance: heap allocations, GC pressure, boxing, virtual dispatch.
// Rust performance: zero-cost abstractions, stack vs heap control,
//                   monomorphisation (no boxing), no GC pauses.
//
// This file documents the major Rust performance patterns with
// C# comparisons. Microbenchmarks are in benches/.
//
// RUN: cargo run --bin performance_tips
// RUN BENCHMARKS: cargo bench
// ============================================================

use std::hint::black_box;
use std::time::Instant;

fn main() {
    println!("=== Performance Patterns ===\n");

    allocation_patterns();
    iterator_zero_cost();
    string_performance();
    capacity_pre_allocation();
    avoid_cloning();
    compile_hints();
    profiling_guidance();
}

// ---- 1. Allocation patterns ----------------------------------------

fn allocation_patterns() {
    println!("--- Allocation Patterns ---");

    // Stack vs heap — prefer stack when size is known at compile time:
    let stack_array: [i32; 1024] = [0; 1024];  // 4KB on stack, zero alloc
    let heap_vec: Vec<i32> = vec![0; 1024];      // heap allocation

    // Measure allocation speed:
    let t = Instant::now();
    let sum: i32 = (0..10_000).map(|_| {
        let a: [i32; 16] = [1; 16];  // stack — no alloc
        a.iter().sum::<i32>()
    }).sum();
    println!("stack arrays: {}ns (sum={sum})", t.elapsed().as_nanos() / 10_000);

    let t = Instant::now();
    let sum2: i32 = (0..10_000).map(|_| {
        let v: Vec<i32> = vec![1; 16];  // heap alloc per iteration
        v.iter().sum::<i32>()
    }).sum();
    println!("heap vecs:    {}ns (sum={sum2})", t.elapsed().as_nanos() / 10_000);

    drop(stack_array);
    drop(heap_vec);

    println!(r#"
Tips:
  • Prefer [T; N] over Vec<T> for fixed-size data
  • Use &str over String when ownership isn't needed
  • Box<T> — one allocation; avoid unnecessary wrapping
  • Avoid Vec<Box<dyn Trait>> when Vec<T> (concrete) is possible
"#);
}

// ---- 2. Iterator zero-cost abstractions ----------------------------

fn iterator_zero_cost() {
    println!("--- Iterator Zero-Cost Abstractions ---");

    let data: Vec<i32> = (0..1_000_000).collect();

    // Chained iterators compile down to the same code as a hand-written loop:
    let t = Instant::now();
    let result_iter: i32 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .take(1000)
        .sum();
    let iter_time = t.elapsed();

    let t = Instant::now();
    let mut result_loop: i32 = 0;
    let mut count = 0;
    for &x in &data {
        if x % 2 == 0 {
            result_loop += x * x;
            count += 1;
            if count == 1000 { break; }
        }
    }
    let loop_time = t.elapsed();

    assert_eq!(result_iter, result_loop);
    println!("iterator chain: {:?}", iter_time);
    println!("manual loop:    {:?}", loop_time);
    println!("(difference is measurement noise — both compile identically)");
}

// ---- 3. String performance -----------------------------------------

fn string_performance() {
    println!("\n--- String Performance ---");

    // Use &str slices instead of String when possible (no allocation):
    fn count_words(s: &str) -> usize {  // &str — zero copy
        s.split_whitespace().count()
    }

    let text = "the quick brown fox jumps over the lazy dog";
    println!("word count: {}", count_words(text));  // no heap allocation for &str

    // String building — use with_capacity to avoid reallocations:
    let words = ["Hello", " ", "World", "!"];

    let t = Instant::now();
    let s1 = words.concat();  // single allocation via concat
    let _ = black_box(s1);
    println!("concat: {:?}", t.elapsed());

    let t = Instant::now();
    let mut s2 = String::with_capacity(16);  // pre-allocated
    for w in &words { s2.push_str(w); }
    let _ = black_box(s2);
    println!("push_str with capacity: {:?}", t.elapsed());

    // format! for complex strings — but it always allocates:
    let s3 = format!("{}{}{}{}", words[0], words[1], words[2], words[3]);
    println!("format!: '{s3}'");
}

// ---- 4. Pre-allocating capacity ------------------------------------

fn capacity_pre_allocation() {
    println!("\n--- Pre-Allocating Capacity ---");

    let n: usize = 100_000;

    // Without pre-allocation: multiple reallocations as Vec grows:
    let t = Instant::now();
    let mut v1: Vec<i32> = Vec::new();
    for i in 0..n { v1.push(i as i32); }
    let t1 = t.elapsed();

    // With pre-allocation: single allocation:
    let t = Instant::now();
    let mut v2: Vec<i32> = Vec::with_capacity(n);
    for i in 0..n { v2.push(i as i32); }
    let t2 = t.elapsed();

    println!("Vec::new() + push*n: {:?}", t1);
    println!("Vec::with_capacity:  {:?}", t2);
    println!("capacity trick saves reallocations (O(log n) → O(1) allocs)");

    // Same for HashMap:
    use std::collections::HashMap;
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(n);
    for i in 0..n as i32 { map.insert(i, i * 2); }
    println!("HashMap::with_capacity({n}): {} entries", map.len());
}

// ---- 5. Avoid unnecessary cloning ----------------------------------

fn avoid_cloning() {
    println!("\n--- Avoid Unnecessary Clones ---");

    #[derive(Debug, Clone)]
    struct BigData(Vec<u8>);

    let data = BigData(vec![0u8; 10_000]);

    // BAD — clones the entire Vec:
    fn process_clone(d: BigData) -> usize { d.0.len() }

    // GOOD — borrow, zero copy:
    fn process_ref(d: &BigData) -> usize { d.0.len() }

    let result = process_ref(&data);     // no clone
    println!("process_ref: {result}");

    let result2 = process_clone(data.clone()); // explicit clone where needed
    println!("process_clone: {result2}");

    // Cow<str> — clone-on-write, stays borrowed when no modification needed:
    use std::borrow::Cow;

    fn to_uppercase_if_needed(s: &str) -> Cow<str> {
        if s.chars().any(|c| c.is_lowercase()) {
            Cow::Owned(s.to_uppercase())   // allocates only when needed
        } else {
            Cow::Borrowed(s)               // zero-copy when already uppercase
        }
    }

    println!("Cow: {}", to_uppercase_if_needed("hello"));  // allocates
    println!("Cow: {}", to_uppercase_if_needed("HELLO"));  // no alloc
}

// ---- 6. Compiler optimization hints --------------------------------

fn compile_hints() {
    println!("\n--- Compiler Hints ---");

    // #[inline] — suggest inlining (eliminate function call overhead):
    #[inline]
    fn add(a: i32, b: i32) -> i32 { a + b }

    // #[inline(always)] — force inlining:
    #[inline(always)]
    fn mul(a: i32, b: i32) -> i32 { a * b }

    // #[cold] — mark a function as rarely called (error paths):
    #[cold]
    fn unlikely_path() { println!("  cold path"); }

    println!("add(3,4) = {}", add(3, 4));
    println!("mul(3,4) = {}", mul(3, 4));
    unlikely_path();

    // black_box — prevent the optimizer from eliminating a computation:
    let x = black_box(42_i32);
    let _ = black_box(x * x);

    // likely/unlikely are not stable in Rust yet — use std::hint::cold_path()
    // (stabilised in 1.95) for branch prediction hints:
    let flag = false;
    if flag {
        std::hint::cold_path();
        println!("unlikely branch");
    } else {
        println!("likely branch");
    }

    println!(r#"
Optimization attributes:
  #[inline]         — suggest inlining
  #[inline(always)] — force inlining
  #[inline(never)]  — prevent inlining (keep as separate function)
  #[cold]           — rarely executed (optimizer reduces priority)
  std::hint::black_box(x)   — prevent dead-code elimination in benchmarks
  std::hint::cold_path()    — mark current code path as unlikely (Rust 1.95)
"#);
}

// ---- 7. Profiling guidance -----------------------------------------

fn profiling_guidance() {
    println!("--- Profiling Guidance ---");

    println!(r#"
Performance workflow:
  1. PROFILE first — don't guess
     Windows:   cargo-flamegraph, VS Performance Profiler, PerfView
     Linux/Mac: perf, flamegraph, cargo-flamegraph

  2. Always benchmark in RELEASE mode:
     cargo bench                    (criterion)
     cargo run --release

  3. Use criterion for microbenchmarks:
     cargo bench --bench string_bench

  4. Common hotspots to check:
     • Allocation inside hot loops → pre-allocate or use stack
     • .clone() on large data → switch to &T or Arc<T>
     • HashMap with bad hash → try FxHashMap (rustc-hash crate)
     • String formatting → write! to a buffer instead of format!
     • Sync Mutex contention → RwLock, or per-thread state + combine
     • Async task spawning overhead → use a bounded task pool

  5. cargo-llvm-lines — see how much LLVM IR each function generates
     (high IR = more monomorphisation = longer compile time)

  6. C# → Rust performance mindset:
     C# GC pause   → Rust has no GC; alloc cost is upfront
     C# boxing     → Rust generics are monomorphised (no boxing)
     C# virtual    → Rust dyn Trait (vtable) vs impl Trait (static dispatch)
     C# LINQ lazy  → Rust iterators are always lazy (same model)
"#);
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec_capacity() {
        let mut v: Vec<i32> = Vec::with_capacity(10);
        assert!(v.capacity() >= 10);
        for i in 0..10 { v.push(i); }
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn cow_no_alloc_for_uppercase() {
        use std::borrow::Cow;
        let s = "ALREADY_UPPER";
        let result: Cow<str> = if s.chars().any(|c| c.is_lowercase()) {
            Cow::Owned(s.to_uppercase())
        } else {
            Cow::Borrowed(s)
        };
        assert!(matches!(result, Cow::Borrowed(_)));
    }
}
