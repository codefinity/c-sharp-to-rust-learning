// ============================================================
// MIGRATION GUIDE: C# Async vs Rust Async
// ============================================================
//
// C# and Rust async/await look similar but have important differences.
// This guide maps every major C# async concept to its Rust equivalent.
//
// RUN: cargo run --bin async_model_comparison
// ============================================================

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== C# Async vs Rust Async ===\n");

    model_overview();
    task_vs_future().await;
    cancellation_pattern().await;
    structured_concurrency().await;
    exception_vs_result().await;
}

fn model_overview() {
    println!("--- Model Overview ---");

    println!(r#"
C# Async Model:
  • Tasks start EAGERLY — await Task.Run(() => ...) begins immediately
  • .NET ThreadPool manages threads
  • ConfigureAwait(false) needed to avoid deadlocks in some contexts
  • CancellationToken for cooperative cancellation
  • SynchronizationContext for UI thread affinity

Rust Async Model:
  • Futures are LAZY — they do nothing until .awaited or spawned
  • The runtime (Tokio) manages the executor
  • No SynchronizationContext — async code is Send+Sync by default
  • Cancellation via dropping the future (structured) or channels
  • No ambient context — state must be passed explicitly

Both models:
  • async fn returns a future/task that can be awaited
  • .await suspends the current task, yielding to the executor
  • Multiple tasks run concurrently on a thread pool
"#);
}

// ---- Task vs Future -----------------------------------------------

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(10)).await;
    format!("data-{id}")
}

async fn task_vs_future() {
    println!("--- Task<T> vs Future ---");

    // C#: var t = FetchDataAsync(1);  // starts immediately
    // Rust:
    let future = fetch_data(1); // created but NOT yet running
    println!("  future created (not running yet)");

    let result = future.await;  // NOW it runs
    println!("  future result: {result}");

    // C#: await Task.WhenAll(t1, t2, t3)
    // Rust: tokio::join!
    let (r1, r2, r3) = tokio::join!(fetch_data(1), fetch_data(2), fetch_data(3));
    println!("  join results: {r1}, {r2}, {r3}");

    // C#: await Task.WhenAny(t1, t2)
    // Rust: tokio::select! (takes the first to complete)
    let winner = tokio::select! {
        r = fetch_data(10) => format!("10 won: {r}"),
        r = fetch_data(20) => format!("20 won: {r}"),
    };
    println!("  select winner: {winner}");

    // C#: Task.Run(() => ...) — fire on threadpool
    // Rust: tokio::spawn(...) — fire on Tokio's thread pool
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(5)).await;
        "spawned task result"
    });
    let result = handle.await.unwrap();
    println!("  spawn result: {result}");

    println!(r#"
Task<T>           ↔ tokio::task::JoinHandle<T>
await t           ↔ handle.await.unwrap()
Task.WhenAll      ↔ tokio::join! or futures::try_join_all
Task.WhenAny      ↔ tokio::select!
Task.Run          ↔ tokio::spawn
Task.Delay(ms)    ↔ tokio::time::sleep(Duration::from_millis(ms))
ValueTask<T>      ↔ impl Future<Output = T> (no heap alloc)
"#);
}

// ---- Cancellation -------------------------------------------------

async fn cancellation_pattern() {
    println!("\n--- Cancellation Patterns ---");

    println!(r#"
C# CancellationToken pattern:
  async Task DoWork(CancellationToken ct) {{
      while (!ct.IsCancellationRequested) {{
          await DoStep(ct);
      }}
  }}

Rust cancellation — three approaches:

1. Drop the future (structured — recommended):
   let task = tokio::spawn(long_running());
   task.abort();  // or just drop(task)

2. tokio::sync::CancellationToken (close to C# CancellationToken):
   let token = CancellationToken::new();
   let child = token.child_token();
   tokio::spawn(async move {{
       tokio::select! {{
           _ = child.cancelled() => {{ /* cancelled */ }}
           _ = do_work() => {{ /* done */ }}
       }}
   }});
   token.cancel();  // trigger cancellation

3. Channel-based signal (like ManualResetEvent):
   let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
   tokio::spawn(async move {{
       tokio::select! {{
           _ = &mut rx => {{ /* shutdown */ }}
           _ = serve()  => {{ }}
       }}
   }});
   let _ = tx.send(());  // signal shutdown
"#);

    // Demo: drop-based cancellation via timeout
    use tokio::time::timeout;

    let result = timeout(
        Duration::from_millis(50),
        async {
            sleep(Duration::from_millis(200)).await;
            "finished"
        },
    ).await;

    match result {
        Ok(v)  => println!("  completed: {v}"),
        Err(_) => println!("  cancelled by timeout (future was dropped)"),
    }
}

// ---- Structured concurrency ---------------------------------------

async fn structured_concurrency() {
    println!("\n--- Structured Concurrency ---");

    println!(r#"
C# .NET 7+ structured concurrency:
  Parallel.ForEachAsync(items, async (item, ct) => {{ ... }});
  await using var cts = new CancellationTokenSource();

Rust tokio structured concurrency:
  JoinSet — like a scoped group of tasks that can be awaited:
    let mut set = JoinSet::new();
    for i in 0..5 {{ set.spawn(work(i)); }}
    while let Some(r) = set.join_next().await {{ ... }}

  tokio::task::spawn_blocking — offload CPU-bound work to a
    dedicated thread pool (like Task.Run in C#):
    tokio::task::spawn_blocking(|| expensive_sync_work())
"#);

    use tokio::task::JoinSet;

    let mut set = JoinSet::new();
    for i in 0..4_u32 {
        set.spawn(async move {
            sleep(Duration::from_millis(10 * i as u64)).await;
            i * i
        });
    }

    let mut results = Vec::new();
    while let Some(r) = set.join_next().await {
        results.push(r.unwrap());
    }
    results.sort();
    println!("  JoinSet results: {results:?}");

    // spawn_blocking for CPU-intensive work:
    let heavy = tokio::task::spawn_blocking(|| {
        (0..100_000_u64).sum::<u64>()
    });
    println!("  spawn_blocking sum: {}", heavy.await.unwrap());
}

// ---- Exceptions vs Result in async code ---------------------------

async fn risky_operation(fail: bool) -> Result<String, String> {
    sleep(Duration::from_millis(1)).await;
    if fail {
        Err("something went wrong".to_string())
    } else {
        Ok("success".to_string())
    }
}

async fn exception_vs_result() {
    println!("\n--- Exceptions vs Result<T,E> in Async ---");

    println!(r#"
C#:
  try {{ await riskyOp(); }}
  catch (Exception e) {{ Console.WriteLine(e.Message); }}

Rust:
  match risky_operation().await {{
      Ok(v)  => println!("{{v}}"),
      Err(e) => println!("{{e}}"),
  }}

  // Or with ? operator in an async fn returning Result:
  async fn do_stuff() -> Result<(), String> {{
      let v = risky_operation(false).await?;
      Ok(())
  }}
"#);

    // Error propagation in async:
    async fn do_stuff() -> Result<String, String> {
        let v = risky_operation(false).await?;
        let v2 = risky_operation(false).await?;
        Ok(format!("{v} + {v2}"))
    }

    println!("  success: {:?}", do_stuff().await);
    println!("  failure: {:?}", risky_operation(true).await);

    // Collecting results from spawned tasks:
    let handles: Vec<_> = (0..3).map(|i| {
        tokio::spawn(async move { risky_operation(i == 2).await })
    }).collect();

    for (i, h) in handles.into_iter().enumerate() {
        match h.await.unwrap() {
            Ok(v)  => println!("  task {i}: {v}"),
            Err(e) => println!("  task {i} error: {e}"),
        }
    }
}
