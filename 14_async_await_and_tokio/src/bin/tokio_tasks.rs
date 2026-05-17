// ============================================================
// CONCEPT: Tokio Tasks — Async Thread Pool
// ============================================================
// RUN: cargo run --bin tokio_tasks
// ============================================================

use std::time::Duration;
use tokio::{time::sleep, task, sync::Semaphore};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    spawn_tasks().await;
    task_concurrency().await;
    rate_limiting().await;
    timeout_demo().await;
}

async fn spawn_tasks() {
    println!("=== tokio::spawn (async green threads) ===");

    // tokio::spawn runs a future on Tokio's thread pool.
    // C# analogy: Task.Run(() => ...) or Fire-and-forget tasks
    let h1 = task::spawn(async {
        sleep(Duration::from_millis(20)).await;
        println!("  task 1 complete");
        1_i32
    });

    let h2 = task::spawn(async {
        sleep(Duration::from_millis(10)).await;
        println!("  task 2 complete");
        2_i32
    });

    // Like Task.WhenAll:
    let (r1, r2) = tokio::join!(h1, h2);
    println!("results: {} {}", r1.unwrap(), r2.unwrap());
}

async fn task_concurrency() {
    println!("\n=== Concurrent Task Processing ===");

    // Process a batch concurrently (like C# Parallel.ForEachAsync):
    let inputs: Vec<i32> = (1..=10).collect();

    let tasks: Vec<_> = inputs.iter()
        .map(|&n| task::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            n * n
        }))
        .collect();

    let results: Vec<i32> = futures::future::try_join_all(tasks)
        .await
        .unwrap();
    println!("squared concurrently: {results:?}");

    // FuturesUnordered — process as they complete (not in submission order):
    use futures::stream::{FuturesUnordered, StreamExt};

    let mut futs = FuturesUnordered::new();
    for i in [30_u64, 10, 20, 5, 25] {
        futs.push(async move {
            sleep(Duration::from_millis(i)).await;
            i
        });
    }

    print!("completed in order: ");
    while let Some(ms) = futs.next().await {
        print!("{ms}ms ");
    }
    println!();
}

async fn rate_limiting() {
    println!("\n=== Semaphore (rate limiting) ===");

    // Semaphore limits concurrent access — like C# SemaphoreSlim.
    let sem = Arc::new(Semaphore::new(3)); // max 3 concurrent operations

    let mut handles = vec![];
    for i in 0..10 {
        let permit = sem.clone().acquire_owned().await.unwrap();
        let h = task::spawn(async move {
            println!("  task {i} running");
            sleep(Duration::from_millis(5)).await;
            drop(permit); // release on done
        });
        handles.push(h);
    }

    for h in handles { h.await.unwrap(); }
    println!("all tasks done");
}

async fn timeout_demo() {
    println!("\n=== Timeout (tokio::time::timeout) ===");

    // C# analogy: CancellationToken with timeout, or Task.WhenAny(task, Task.Delay)
    use tokio::time::timeout;

    async fn slow_operation() -> String {
        sleep(Duration::from_millis(100)).await;
        "done".to_string()
    }

    // Should succeed:
    match timeout(Duration::from_millis(200), slow_operation()).await {
        Ok(result) => println!("succeeded: {result}"),
        Err(_)     => println!("timed out"),
    }

    // Should time out:
    match timeout(Duration::from_millis(50), slow_operation()).await {
        Ok(result) => println!("succeeded: {result}"),
        Err(_)     => println!("timed out (expected)"),
    }
}
