// ============================================================
// CONCEPT: async/await and Futures
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# async/await uses Tasks backed by the .NET thread pool.
// Rust async/await uses Futures — they are LAZY (don't execute until polled).
// You need an EXECUTOR (like Tokio) to actually run them.
//
// C#:
//   async Task<int> FetchDataAsync() {
//       await Task.Delay(100);
//       return 42;
//   }
//
// Rust:
//   async fn fetch_data() -> i32 {
//       tokio::time::sleep(Duration::from_millis(100)).await;
//       42
//   }
//
// KEY DIFFERENCE: Rust futures are lazy — creating a future does nothing.
// C# Tasks start executing immediately when created.
//
// RUN: cargo run --bin async_basics
// ============================================================

use std::time::Duration;
use tokio::time::sleep;

// Mark main as async and use tokio's runtime:
#[tokio::main]
async fn main() {
    println!("=== Async/Await Basics ===\n");

    basics().await;
    futures_are_lazy().await;
    error_propagation().await;
    async_closures_demo().await;
}

// Simple async function — returns a Future<Output = i32>
async fn compute(x: i32) -> i32 {
    sleep(Duration::from_millis(10)).await; // yield control to executor
    x * x
}

// Multiple awaits in sequence:
async fn pipeline(input: &str) -> Result<i32, String> {
    let n: i32 = input.parse().map_err(|_| format!("not a number: {input}"))?;
    let squared = compute(n).await;
    Ok(squared)
}

async fn basics() {
    println!("--- Basics ---");

    // Await a single future:
    let result = compute(5).await;
    println!("compute(5) = {result}");

    // Sequential await (like C# await Task1; await Task2):
    let a = compute(3).await;
    let b = compute(4).await;
    println!("3²={a} 4²={b}");

    // Concurrent await (like C# Task.WhenAll):
    let (a, b, c) = tokio::join!(
        compute(3),
        compute(4),
        compute(5),
    );
    println!("join: {}+{}+{}={}", a, b, c, a+b+c);

    // Pipeline with error propagation:
    println!("pipeline('7') = {:?}", pipeline("7").await);
    println!("pipeline('x') = {:?}", pipeline("x").await);
}

async fn futures_are_lazy() {
    println!("\n--- Futures are Lazy ---");

    // Creating a future does NOT run it:
    let future = compute(10); // nothing happens here!
    println!("future created but not yet run");

    // Only when .await is called does it execute:
    let result = future.await;
    println!("future ran, result = {result}");

    // C# contrast: Task.Run(() => Compute(10)) starts immediately.
    // Rust: let f = compute(10); — f is just a struct, no code runs.

    // select! — race futures, take the first to complete
    // (like C# Task.WhenAny):
    let result = tokio::select! {
        v = slow_op("op1", 50)  => format!("op1 won: {v}"),
        v = slow_op("op2", 20)  => format!("op2 won: {v}"),
        v = slow_op("op3", 80)  => format!("op3 won: {v}"),
    };
    println!("select winner: {result}");
}

async fn slow_op(name: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    format!("{name} took {ms}ms")
}

async fn error_propagation() {
    println!("\n--- Error Propagation in Async ===");

    // ? works in async fns the same as sync fns:
    async fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n: i32 = s.trim().parse()?; // ? propagates error
        Ok(n * 2)
    }

    println!("'5' → {:?}", parse_and_double("5").await);
    println!("'x' → {:?}", parse_and_double("x").await.map_err(|e| e.to_string()));
}

async fn async_closures_demo() {
    println!("\n--- Async Closures (Rust 1.85+) ---");

    // Async closures are now stable in Rust 1.85 (edition 2024):
    let double_async = async |x: i32| {
        sleep(Duration::from_millis(1)).await;
        x * 2
    };

    let result = double_async(21).await;
    println!("async closure result: {result}");

    // Higher-order async functions — use stdlib AsyncFn (stabilised Rust 1.85):
    async fn apply_async<F>(f: F, x: i32) -> i32
    where
        F: std::ops::AsyncFn(i32) -> i32,
    {
        f(x).await
    }

    let r = apply_async(async |x| x + 10, 5).await;
    println!("apply_async: {r}");
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn async_test() {
        let result = super::compute(5).await;
        assert_eq!(result, 25);
    }

    #[tokio::test]
    async fn pipeline_ok() {
        assert_eq!(super::pipeline("4").await, Ok(16));
    }

    #[tokio::test]
    async fn pipeline_err() {
        assert!(super::pipeline("nope").await.is_err());
    }
}
