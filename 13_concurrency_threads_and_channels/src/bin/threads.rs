// ============================================================
// CONCEPT: Threads — std::thread
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# threading: Task, Thread, ThreadPool, async/await.
// Rust threading: std::thread (OS threads), plus channels for communication.
// Async/await with Tokio is in module 14.
//
// KEY GUARANTEE: Rust's borrow checker prevents data races at compile time.
// If it compiles with threads, it won't have data races — this is the
// "fearless concurrency" promise.
//
// RUN: cargo run --bin threads
// ============================================================

use std::thread;
use std::time::Duration;

fn main() {
    basic_threads();
    join_handles();
    thread_with_data();
    thread_pool_pattern();
    scoped_threads();
}

fn basic_threads() {
    println!("=== Basic Thread Spawning ===");

    // C#: Task.Run(() => { ... }) or new Thread(() => { ... }).Start()
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=3 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // .join() waits for the thread to finish (like Task.Wait() / await task)
    handle.join().unwrap();
    println!("spawned thread finished");
}

fn join_handles() {
    println!("\n=== JoinHandle and Thread Results ===");

    // Threads can return values:
    let compute = thread::spawn(|| {
        // Simulate expensive computation
        thread::sleep(Duration::from_millis(10));
        let sum: u64 = (1..=1000).sum();
        sum
    });

    // Do other work while compute runs:
    println!("doing other work while thread computes...");

    // Retrieve the result:
    let result = compute.join().unwrap(); // unwrap: Result<T, Box<dyn Any>>
    println!("sum 1..=1000 = {result}");

    // Multiple threads with results:
    let handles: Vec<_> = (0..4)
        .map(|i| thread::spawn(move || i * i))
        .collect();

    let results: Vec<u64> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("squares: {results:?}");
}

fn thread_with_data() {
    println!("\n=== Passing Data to Threads ===");

    // `move` is required when the closure needs to own data from outer scope:
    let data = vec![1, 2, 3, 4, 5];

    // Without move: `data` would need to live as long as the thread — compiler error
    let handle = thread::spawn(move || {
        // `data` is MOVED into this thread
        let sum: i32 = data.iter().sum();
        println!("  thread sum: {sum}");
        sum
    });
    // data is no longer accessible here — it was moved
    let result = handle.join().unwrap();
    println!("result: {result}");

    // Sharing data with Arc (multiple threads reading the same data):
    use std::sync::Arc;
    let shared = Arc::new(vec![10, 20, 30, 40, 50]);

    let handles: Vec<_> = (0..3).map(|i| {
        let d = Arc::clone(&shared);
        thread::spawn(move || {
            println!("  thread {i} sees: {d:?}");
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
}

fn thread_pool_pattern() {
    println!("\n=== Simulating Thread Pool ===");

    use std::sync::{Arc, Mutex};

    // Simple work queue processed by multiple threads:
    let work_queue: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new((1..=20).collect()));
    let results:    Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];
    for t in 0..4 {
        let queue   = Arc::clone(&work_queue);
        let results = Arc::clone(&results);
        let h = thread::spawn(move || {
            loop {
                let item = queue.lock().unwrap().pop();
                match item {
                    None    => break,
                    Some(n) => {
                        results.lock().unwrap().push(n * n);
                        thread::sleep(Duration::from_millis(1));
                        print!("  t{t}: {n}² ");
                    }
                }
            }
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    let mut final_results = results.lock().unwrap().clone();
    final_results.sort();
    println!("\nsquares: {final_results:?}");
}

fn scoped_threads() {
    println!("\n=== Scoped Threads (thread::scope) ===");

    // thread::scope lets threads BORROW data (no move needed!)
    // All spawned threads are guaranteed to finish before scope exits.
    // C# analogy: Parallel.For / PLINQ — all work completes before returning.

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let chunk_size = data.len() / 4;

    let mut partial_sums = [0_i64; 4];

    thread::scope(|s| {
        for (i, (chunk, sum)) in data.chunks(chunk_size).zip(partial_sums.iter_mut()).enumerate() {
            let chunk = chunk; // reborrow inside scope
            s.spawn(move || {
                *sum = chunk.iter().map(|&x| x as i64).sum();
                println!("  chunk {i}: sum = {sum}");
            });
        }
        // All spawned threads join here when the closure ends
    });

    let total: i64 = partial_sums.iter().sum();
    println!("total: {total}");
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::{Arc, Mutex};

    #[test]
    fn thread_returns_value() {
        let h = thread::spawn(|| 42_i32);
        assert_eq!(h.join().unwrap(), 42);
    }

    #[test]
    fn mutex_counter_correct() {
        let c = Arc::new(Mutex::new(0_i32));
        let handles: Vec<_> = (0..10).map(|_| {
            let c = Arc::clone(&c);
            thread::spawn(move || { *c.lock().unwrap() += 1; })
        }).collect();
        for h in handles { h.join().unwrap(); }
        assert_eq!(*c.lock().unwrap(), 10);
    }
}
