// ============================================================
// CONCEPT: Mutex<T>, RwLock<T>, and Thread-Safe Shared State
// ============================================================
// RUN: cargo run --bin mutex_rwlock
// ============================================================

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    mutex_basics();
    rwlock_demo();
    poison_handling();
    arc_mutex_pattern();
    send_sync_traits();
}

fn mutex_basics() {
    println!("=== Mutex<T> ===");

    // Mutex provides mutual exclusion — only one thread can hold the lock.
    // C# analogy: lock(obj) { ... }  or SemaphoreSlim(1,1)
    // Key difference: in Rust, Mutex OWNS the data. Data can only be
    // accessed through the lock — impossible to forget to lock.

    let m = Mutex::new(5_i32);

    {
        let mut num = m.lock().unwrap(); // acquire lock, returns MutexGuard
        *num = 6;
        println!("num = {num}");
    } // MutexGuard dropped here → lock released automatically (like C# `using`)

    println!("after unlock: {}", m.lock().unwrap());

    // Arc<Mutex<T>> for multi-threaded shared mutation:
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut val = c.lock().unwrap();
            *val += 1;
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }
    println!("final counter: {}", counter.lock().unwrap()); // 10
}

fn rwlock_demo() {
    println!("\n=== RwLock<T> (readers-writer lock) ===");

    // RwLock allows multiple readers OR one writer — never both.
    // C# analogy: ReaderWriterLockSlim
    // Use when reads are more frequent than writes.

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    // Spawn 3 reader threads:
    for i in 0..3 {
        let d = Arc::clone(&data);
        let h = thread::spawn(move || {
            let guard = d.read().unwrap(); // multiple readers OK simultaneously
            println!("  reader {i}: {:?}", *guard);
        });
        handles.push(h);
    }

    // Spawn one writer thread:
    {
        let d = Arc::clone(&data);
        let h = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1)); // let readers start
            let mut guard = d.write().unwrap(); // waits for all readers
            guard.push(4);
            println!("  writer: pushed 4");
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }
    println!("final data: {:?}", data.read().unwrap());
}

fn poison_handling() {
    println!("\n=== Mutex Poisoning ===");

    // If a thread panics while holding a lock, the mutex becomes "poisoned".
    // Subsequent .lock() calls return Err(PoisonError<...>).
    // This prevents access to potentially inconsistent data.

    let mutex = Arc::new(Mutex::new(0_i32));
    let m = Arc::clone(&mutex);

    let _ = thread::spawn(move || {
        let _guard = m.lock().unwrap();
        panic!("thread panicked while holding lock!"); // poisons the mutex
    }).join(); // we expect this to fail

    // Now the mutex is poisoned:
    match mutex.lock() {
        Ok(val)  => println!("lock acquired: {val}"),
        Err(e)   => {
            println!("mutex poisoned: {e}");
            // You CAN recover with into_inner():
            let val = e.into_inner();
            println!("  recovered value: {val}");
        }
    }
}

fn arc_mutex_pattern() {
    println!("\n=== Arc<Mutex<T>> Pattern ===");

    // The definitive pattern for shared mutable state across threads.
    // This is Rust's answer to C#'s thread-safe mutable fields.

    #[derive(Debug)]
    struct SharedState {
        count:    u64,
        messages: Vec<String>,
    }

    let state = Arc::new(Mutex::new(SharedState { count: 0, messages: Vec::new() }));

    let mut handles = vec![];
    for i in 0..5 {
        let s = Arc::clone(&state);
        let h = thread::spawn(move || {
            let mut guard = s.lock().unwrap();
            guard.count += 1;
            guard.messages.push(format!("message from thread {i}"));
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    let final_state = state.lock().unwrap();
    println!("count: {}", final_state.count);
    println!("messages ({}):", final_state.messages.len());
    for msg in &final_state.messages {
        println!("  {msg}");
    }
}

fn send_sync_traits() {
    println!("\n=== Send and Sync Marker Traits ===");
    println!(
        r#"
Send: a type can be MOVED to another thread
  - All primitive types: Send
  - Rc<T>: NOT Send (non-atomic ref count)
  - Arc<T>: Send (atomic ref count)
  - MutexGuard<T>: NOT Send (must unlock on same thread)

Sync: a type can be SHARED between threads via &T
  - All primitive types: Sync
  - RefCell<T>: NOT Sync (non-atomic borrow count)
  - Mutex<T>: Sync (serialises access)
  - Cell<T>: NOT Sync

The borrow checker uses Send+Sync to prevent data races
at COMPILE TIME — this is Rust's "fearless concurrency."

C# has no compile-time check for thread safety.
ThreadLocal, [ThreadStatic], and concurrent collections
are runtime-guarded only.
"#
    );

    // The compiler enforces Send+Sync — these won't compile:
    // let rc = Rc::new(5);
    // thread::spawn(move || { let _ = rc; }); // Rc is not Send!

    // But Arc works fine:
    let arc = Arc::new(5_i32);
    let h = thread::spawn(move || arc); // Arc is Send
    let _ = h.join().unwrap();
    println!("Arc successfully moved to thread");
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn mutex_counter() {
        let counter = Arc::new(Mutex::new(0_i32));
        let mut handles = vec![];
        for _ in 0..5 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || { *c.lock().unwrap() += 1; }));
        }
        for h in handles { h.join().unwrap(); }
        assert_eq!(*counter.lock().unwrap(), 5);
    }
}
