// ============================================================
// CONCEPT: Async Streams — tokio-stream / futures::Stream
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has IAsyncEnumerable<T> + await foreach.
// Rust has the Stream trait (futures crate) — the async analogue of Iterator.
//
// C#:
//   async IAsyncEnumerable<int> Range(int n) {
//       for (int i = 0; i < n; i++) { yield return i; }
//   }
//   await foreach (var x in Range(5)) { ... }
//
// Rust:
//   use futures::stream::{self, StreamExt};
//   let mut s = stream::iter(0..5);
//   while let Some(x) = s.next().await { ... }
//
// RUN: cargo run --bin streams
// ============================================================

use std::time::Duration;
use futures::stream::{self, Stream, StreamExt};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Async Streams ===\n");

    stream_basics().await;
    stream_adapters().await;
    custom_stream().await;
    stream_combinators().await;
    bounded_channel_stream().await;
}

// ---- 1. Stream basics -----------------------------------------------

async fn stream_basics() {
    println!("--- Stream Basics ---");

    // stream::iter wraps any IntoIterator as a ready-made Stream.
    // C# analogy: AsyncEnumerable.ToAsyncEnumerable(IEnumerable)
    let mut s = stream::iter(vec![1, 2, 3, 4, 5]);

    // while let is the idiomatic way to consume a stream — like await foreach:
    while let Some(val) = s.next().await {
        print!("{val} ");
    }
    println!();

    // collect works just like on iterators:
    let doubled: Vec<i32> = stream::iter(1..=5)
        .map(|x| x * 2)
        .collect()
        .await;
    println!("doubled: {doubled:?}");
}

// ---- 2. Stream adapters (mirror of Iterator adapters) ---------------

async fn stream_adapters() {
    println!("\n--- Stream Adapters ---");

    // map, filter, take, skip, enumerate — same names as Iterator
    // filter closures receive &Item so we copy to move into async block:
    let evens: Vec<u32> = stream::iter(1..=10_u32)
        .filter(|&x| async move { x % 2 == 0 })
        .collect()
        .await;
    println!("evens: {evens:?}");

    // flat_map — like SelectMany in C# LINQ:
    let flattened: Vec<i32> = stream::iter(vec![1, 2, 3])
        .flat_map(|x| stream::iter(vec![x, x * 10]))
        .collect()
        .await;
    println!("flat_map: {flattened:?}");

    // take / skip:
    let taken: Vec<i32> = stream::iter(0..)
        .take(5)
        .collect()
        .await;
    println!("take(5): {taken:?}");

    // fold — like Aggregate in LINQ:
    let sum = stream::iter(1..=10)
        .fold(0, |acc, x| async move { acc + x })
        .await;
    println!("fold sum 1..10 = {sum}");

    // any / all:
    let has_even = stream::iter(1..=10)
        .any(|x| async move { x % 2 == 0 })
        .await;
    println!("any even: {has_even}");
}

// ---- 3. Custom Stream via async_stream or manual poll ---------------

// The simplest way to produce an async stream with per-item async work.
// We use tokio::sync::mpsc as a producer-consumer pattern:
fn interval_stream(ticks: u32, delay_ms: u64) -> impl Stream<Item = u32> {
    let (tx, rx) = tokio::sync::mpsc::channel(16);

    tokio::spawn(async move {
        for i in 0..ticks {
            sleep(Duration::from_millis(delay_ms)).await;
            if tx.send(i).await.is_err() {
                break; // receiver dropped
            }
        }
    });

    tokio_stream::wrappers::ReceiverStream::new(rx)
}

async fn custom_stream() {
    println!("\n--- Custom Stream (via mpsc channel) ---");

    // C# analogy: Channel<T> + IAsyncEnumerable via yield return
    let mut ticks = interval_stream(5, 5);
    while let Some(tick) = ticks.next().await {
        print!("tick {tick} ");
    }
    println!();
}

// ---- 4. Stream combinators -----------------------------------------

async fn stream_combinators() {
    println!("\n--- Stream Combinators ---");

    // chain — concatenate two streams (like Concat in LINQ):
    let s1 = stream::iter(0..3_i32);
    let s2 = stream::iter(10..13_i32);
    let chained: Vec<i32> = s1.chain(s2).collect().await;
    println!("chain: {chained:?}");

    // zip — pair up two streams (stops at shorter):
    let names = stream::iter(vec!["Alice", "Bob", "Carol"]);
    let scores = stream::iter(vec![95, 87, 92]);
    let pairs: Vec<(&str, i32)> = names.zip(scores).collect().await;
    println!("zip: {pairs:?}");

    // enumerate:
    let mut s = stream::iter(["a", "b", "c"]).enumerate();
    while let Some((i, v)) = s.next().await {
        print!("({i},{v}) ");
    }
    println!();

    // peekable — look at next element without consuming:
    // Peekable streams require pinning to use peek().
    let mut peekable = std::pin::pin!(stream::iter(vec![1_i32, 2, 3]).peekable());
    if let Some(&first) = peekable.as_mut().peek().await {
        println!("peek: {first}");
    }
    let rest: Vec<i32> = peekable.collect().await;
    println!("rest: {rest:?}");
}

// ---- 5. tokio::sync::mpsc as bounded async stream ------------------

async fn bounded_channel_stream() {
    println!("\n--- Bounded Channel as Stream ---");

    // tokio's bounded channel is the equivalent of C# Channel.CreateBounded<T>()
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(4);

    // Producer task:
    tokio::spawn(async move {
        for i in 0..6 {
            let msg = format!("msg-{i}");
            println!("  sending: {msg}");
            tx.send(msg).await.unwrap();
            sleep(Duration::from_millis(5)).await;
        }
        // tx dropped here — stream ends
    });

    // Consumer via ReceiverStream adapter:
    use tokio_stream::wrappers::ReceiverStream;
    let mut stream = ReceiverStream::new(rx);

    while let Some(msg) = stream.next().await {
        println!("  received: {msg}");
    }
    println!("stream complete");
}

// ---- Key comparison table ------------------------------------------
//
// C#                              | Rust
// --------------------------------|---------------------------------
// IAsyncEnumerable<T>             | Stream<Item = T>
// await foreach (var x in ...)    | while let Some(x) = s.next().await
// yield return x                  | mpsc channel or async_stream crate
// .Select(x => f(x))              | .map(|x| async move { f(x) })
// .Where(pred)                    | .filter(|x| async move { pred(x) })
// .Take(n)                        | .take(n)
// .Concat(other)                  | .chain(other)
// .Zip(other, (a,b) => ...)       | .zip(other)  (no combiner fn)
// Enumerable.Range(0, n)          | stream::iter(0..n)
// Channel.CreateBounded<T>(n)     | tokio::sync::mpsc::channel(n)
