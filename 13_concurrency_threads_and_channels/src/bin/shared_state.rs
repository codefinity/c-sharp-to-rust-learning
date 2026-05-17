// ============================================================
// CONCEPT: Shared-State Concurrency Patterns
// ============================================================
// RUN: cargo run --bin shared_state
// ============================================================

use std::sync::{Arc, Mutex, RwLock, Barrier, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    barrier_demo();
    condvar_demo();
    once_cell_demo();
    atomic_operations();
}

fn barrier_demo() {
    println!("=== Barrier (synchronise at a point) ===");

    // Barrier waits for N threads to reach it before any continue.
    // C# analogy: Barrier class in System.Threading
    let barrier = Arc::new(Barrier::new(4));

    let handles: Vec<_> = (0..4).map(|i| {
        let b = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("  thread {i}: before barrier");
            thread::sleep(Duration::from_millis(i as u64 * 5));
            b.wait(); // all 4 threads must reach here before any proceed
            println!("  thread {i}: after barrier");
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
}

fn condvar_demo() {
    println!("\n=== Condvar (condition variable) ===");

    // Condvar lets threads wait for a condition to become true.
    // C# analogy: Monitor.Wait / Monitor.PulseAll or AutoResetEvent

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let producer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one(); // wake up one waiting thread
        println!("  producer: signalled ready");
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap(); // atomically releases lock and waits
    }
    println!("  consumer: condition met!");

    producer.join().unwrap();
}

fn once_cell_demo() {
    println!("\n=== Once / OnceLock (lazy static initialisation) ===");

    // OnceLock is a thread-safe cell that initialises exactly once.
    // C# analogy: Lazy<T> (thread-safe by default)
    use std::sync::OnceLock;

    static CONFIG: OnceLock<String> = OnceLock::new();

    fn get_config() -> &'static str {
        CONFIG.get_or_init(|| {
            println!("  initialising config (once only)...");
            "prod_config_value".to_string()
        })
    }

    // All threads see the same initialisation:
    let handles: Vec<_> = (0..3).map(|_| {
        thread::spawn(|| println!("  config: {}", get_config()))
    }).collect();
    for h in handles { h.join().unwrap(); }

    // std::sync::Once for arbitrary one-time setup:
    use std::sync::Once;
    static INIT: Once = Once::new();
    let mut initialised = false;

    for _ in 0..5 {
        INIT.call_once(|| {
            initialised = true;
            println!("  Once: executed exactly once");
        });
    }
    println!("  initialised: {initialised}");
}

fn atomic_operations() {
    println!("\n=== Atomic Operations ===");

    // Atomics allow lock-free operations on shared primitives.
    // C# analogy: Interlocked class, Volatile.Read/Write
    use std::sync::atomic::{AtomicI64, AtomicBool, Ordering};

    let counter = Arc::new(AtomicI64::new(0));
    let done    = Arc::new(AtomicBool::new(false));

    let counter_clone = Arc::clone(&counter);
    let done_clone    = Arc::clone(&done);

    let producer = thread::spawn(move || {
        for _ in 0..1000 {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }
        done_clone.store(true, Ordering::Release);
    });

    // Spin wait for done flag (busy-wait — normally use Condvar or channels):
    while !done.load(Ordering::Acquire) {
        thread::yield_now();
    }

    println!("final count: {}", counter.load(Ordering::SeqCst));

    producer.join().unwrap();

    // Ordering semantics (analogous to C# Volatile / Thread.MemoryBarrier):
    println!(
        r#"
Atomic Ordering:
  Relaxed    — no ordering guarantees, just atomicity (fastest)
  Acquire    — see all writes before this point
  Release    — make all writes visible to Acquire loads
  AcqRel     — both Acquire + Release
  SeqCst     — sequential consistency (strongest, like volatile in Java)

C# Interlocked equivalents:
  Interlocked.Increment  → fetch_add(1, SeqCst)
  Interlocked.Exchange   → swap(val, SeqCst)
  Interlocked.CompareExchange → compare_exchange(...)
"#
    );
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn barrier_synchronises() {
        use std::sync::Barrier;
        let barrier = Arc::new(Barrier::new(3));
        let order: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

        let handles: Vec<_> = (0..3).map(|i| {
            let b = Arc::clone(&barrier);
            let o = Arc::clone(&order);
            thread::spawn(move || {
                b.wait();
                o.lock().unwrap().push(i);
            })
        }).collect();

        for h in handles { h.join().unwrap(); }
        assert_eq!(order.lock().unwrap().len(), 3);
    }
}
