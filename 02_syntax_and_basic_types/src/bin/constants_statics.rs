// ============================================================
// CONCEPT: Constants and Statics
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C#:
//   const double PI = 3.14;       // compile-time constant, inlined
//   static readonly int MAX = 100; // runtime constant per instance/type
//
// Rust:
//   const PI: f64 = 3.14;         // compile-time constant, always inlined
//   static MAX: i32 = 100;        // single memory location for the program
//   static mut COUNTER: i32 = 0;  // mutable global (requires unsafe)
//
// RUN: cargo run --bin constants_statics
// ============================================================

// Constants: evaluated at compile time, inlined at every use site.
// Type annotation is REQUIRED (no inference for const/static).
const MAX_POINTS: u32 = 100_000;
const PI: f64 = std::f64::consts::PI;

// Constants can appear in any scope including inside functions.
// They can use const-evaluable expressions:
const KILOBYTE: usize = 1024;
const MEGABYTE: usize = KILOBYTE * 1024;
const GIGABYTE: usize = MEGABYTE * 1024;

// Static: a single fixed memory address for the program's lifetime.
// Unlike const, statics are NOT inlined — they live at a fixed address.
static GREETING: &str = "Hello from a static!";

// ⚠️ Mutable statics are unsafe — only modifiable in `unsafe` blocks.
// Prefer atomic types (AtomicI32, etc.) for safe global mutation.
static mut UNSAFE_COUNTER: i32 = 0;

// Safe alternative: use atomics
use std::sync::atomic::{AtomicI32, Ordering};
static SAFE_COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    const_demo();
    static_demo();
    atomic_demo();
    const_fn_demo();
    const_generics_demo();
}

fn const_demo() {
    println!("=== Constants ===");
    println!("MAX_POINTS = {MAX_POINTS}");
    println!("PI = {PI:.10}");
    println!("1 KB = {KILOBYTE}");
    println!("1 MB = {MEGABYTE}");
    println!("1 GB = {GIGABYTE}");

    // const inside a function — same rules, scoped to function
    const LOCAL_MAX: i32 = 42;
    println!("LOCAL_MAX = {LOCAL_MAX}");

    // Use in match arms (constants are valid patterns):
    let x = 100_000_u32;
    match x {
        0..=MAX_POINTS => println!("{x} is within limits"),
        _              => println!("{x} exceeds MAX_POINTS"),
    }
}

fn static_demo() {
    println!("\n=== Statics ===");
    println!("{GREETING}");

    // Static has a fixed address — useful for global configuration/tables.
    println!("address of GREETING: {:p}", GREETING as *const str);

    // Mutable statics require unsafe — avoid them; use atomics instead.
    // Edition 2024: creating a shared reference to static mut is a hard error.
    // Use ptr::addr_of! to read without forming a reference.
    unsafe {
        UNSAFE_COUNTER += 1;
        let val = std::ptr::addr_of!(UNSAFE_COUNTER).read();
        println!("UNSAFE_COUNTER = {val}");
    }
}

fn atomic_demo() {
    println!("\n=== Atomic Statics (safe alternative to static mut) ===");

    // Atomics are safe to read/write across threads without data races.
    SAFE_COUNTER.fetch_add(1, Ordering::Relaxed);
    SAFE_COUNTER.fetch_add(1, Ordering::Relaxed);
    println!("SAFE_COUNTER = {}", SAFE_COUNTER.load(Ordering::Relaxed));

    // C# equivalent: Interlocked.Increment(ref counter)
}

// const fn — functions callable at compile time (like C++ constexpr)
// C# has limited equivalent with `const` methods in structs (Span-based).
const fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn const_fn_demo() {
    println!("\n=== const fn (compile-time functions) ===");

    // Evaluated at compile time — zero runtime cost.
    const FIB_10: u64 = fibonacci(10);
    println!("fibonacci(10) = {FIB_10}");

    // Can be called at runtime too:
    let n = 15_u64;
    println!("fibonacci({n}) = {}", fibonacci(n));
}

fn const_generics_demo() {
    println!("\n=== Const Generics ===");

    // Const generics allow types parameterised by constant values.
    // Useful for fixed-size buffers, matrices, etc.
    // (C# doesn't have this — closest is value type generic constraints.)

    fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
        arr.iter().sum()
    }

    println!("sum [1,2,3]   = {}", sum_array([1, 2, 3]));
    println!("sum [1..=5]   = {}", sum_array([1, 2, 3, 4, 5]));

    // Stack-allocated matrix using const generics:
    struct Matrix<const R: usize, const C: usize> {
        data: [[f64; C]; R],
    }

    impl<const R: usize, const C: usize> Matrix<R, C> {
        fn new() -> Self {
            Self { data: [[0.0; C]; R] }
        }
        fn rows(&self) -> usize { R }
        fn cols(&self) -> usize { C }
    }

    let m: Matrix<3, 4> = Matrix::new();
    println!("Matrix: {}×{}", m.rows(), m.cols());
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. `const` requires type annotation — no inference.
// 2. `static` has a fixed address; `const` is inlined.
// 3. Mutable statics require `unsafe` — prefer `AtomicXxx` types.
// 4. `const fn` enables compile-time computation (more powerful than C# const).
// 5. Const generics enable types parameterised by values, not just types.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_const() {
        const FIB: u64 = fibonacci(10);
        assert_eq!(FIB, 55);
    }

    #[test]
    fn megabyte_is_correct() {
        assert_eq!(MEGABYTE, 1_048_576);
    }

    #[test]
    fn atomic_counter() {
        let c = AtomicI32::new(0);
        c.fetch_add(5, Ordering::Relaxed);
        assert_eq!(c.load(Ordering::Relaxed), 5);
    }
}
