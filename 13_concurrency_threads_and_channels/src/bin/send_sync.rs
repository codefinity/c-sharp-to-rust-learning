// ============================================================
// CONCEPT: Send and Sync — Rust's Thread-Safety Type System
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# thread safety is a runtime concern — you get data races, locks,
// and the ThreadStaticAttribute, but the compiler doesn't stop you
// from sharing non-thread-safe types across threads.
//
// Rust encodes thread safety INTO THE TYPE SYSTEM via two marker traits:
//
//   Send  — a value of type T can be MOVED to another thread safely.
//   Sync  — a value of &T can be SHARED across threads safely.
//           Equivalently: T is Sync iff &T is Send.
//
// These traits are automatically implemented (auto traits) for most types.
// The compiler proves thread safety at compile time — data races are
// impossible in safe Rust. This is Rust's "fearless concurrency" guarantee.
//
// C# equivalent mental model:
//   Send  ≈ "safe to pass to Task.Run / new Thread()"
//   Sync  ≈ "safe to access from multiple threads without locking"
//
// RUN: cargo run --bin send_sync
// ============================================================

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    println!("=== Send and Sync — Thread-Safety in the Type System ===\n");

    demo_send();
    demo_sync();
    demo_arc_mutex_pattern();
    demo_not_send_not_sync();
    demo_making_types_send_sync();
    demo_reference_guide();
}

// ─── 1. SEND ────────────────────────────────────────────────────────────────
//
// Send: a type T can be moved (transferred ownership) to another thread.
//
// C# analogy: think of it as "can I safely pass this object to a new Thread()?".
// C# doesn't check this — Rust does, at compile time.
//
// Almost all types are Send automatically. Exceptions:
//   Rc<T>         — reference-counted pointer WITHOUT atomic ref count
//                   (use Arc<T> instead for cross-thread sharing)
//   *mut T        — raw mutable pointer (no safety guarantee)
//   Cell<T>       — interior mutability without synchronisation
//   MutexGuard<T> — lock guard must be released on the same thread that acquired it

fn demo_send() {
    println!("--- 1. Send: Moving Values to Other Threads ---\n");

    // String, Vec, i32, Box<T> are all Send — can move to another thread:
    let data = vec![1_i32, 2, 3, 4, 5];
    let message = String::from("hello from main");

    let handle = thread::spawn(move || {
        // `data` and `message` are moved here — this is safe because Vec and String are Send.
        println!("  Thread received: {message}");
        let sum: i32 = data.iter().sum();
        println!("  Sum in thread: {sum}");
        sum
    });

    let result = handle.join().unwrap();
    println!("  Thread returned: {result}\n");

    // WHY Rc<T> is NOT Send:
    // Rc<T> uses a non-atomic reference count.
    // If two threads incremented/decremented it simultaneously → data race.
    // The compiler rejects: thread::spawn(move || { let _ = rc_value; })
    // Solution: use Arc<T> instead (atomic reference count).

    println!(r#"  Send means: ownership can safely cross a thread boundary.

  Type          Send?   Why
  ──────────────────────────────────────────────────────────────────────
  i32, f64, bool  Yes   Primitive — no shared state
  String, Vec<T>  Yes   Owned heap data, moved in full
  Box<T>          Yes   (when T: Send) — single owner, moves fully
  Arc<T>          Yes   (when T: Send + Sync) — atomic ref count
  Rc<T>           NO    Non-atomic ref count — race condition possible
  *mut T          NO    Raw pointer — compiler can't verify safety
  MutexGuard<T>   NO    Must be released on same thread that locked

  C# analogy: passing a non-[ThreadSafe] object to Task.Run()
  C# allows it and crashes at runtime. Rust rejects it at compile time.
"#);
}

// ─── 2. SYNC ────────────────────────────────────────────────────────────────
//
// Sync: shared references &T can safely be accessed from multiple threads.
// Formally: T is Sync iff &T is Send.
//
// C# analogy: "is it safe for multiple threads to READ this object simultaneously
// without explicit locking?" A C# class marked [ThreadSafe] roughly implies Sync.

fn demo_sync() {
    println!("--- 2. Sync: Sharing References Across Threads ---\n");

    // Arc<T> lets multiple threads SHARE ownership (via clone = atomic ref count bump).
    // T must be Sync for Arc<T> to be Sync — otherwise sharing &T would be unsafe.

    let shared_data = Arc::new(vec![10_i32, 20, 30]);

    let mut handles = vec![];
    for thread_id in 0..3 {
        let data = Arc::clone(&shared_data);  // clone the Arc, not the Vec
        handles.push(thread::spawn(move || {
            // Each thread gets a clone of the Arc — they all point to the SAME Vec.
            // Vec<i32> is Sync (read-only references are safe concurrently).
            let sum: i32 = data.iter().sum();
            println!("  Thread {thread_id} sees sum = {sum}");
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!();

    println!(r#"  Sync means: &T can be safely held by multiple threads simultaneously.

  Type              Sync?   Why
  ────────────────────────────────────────────────────────────────────────
  i32, f64, bool    Yes     Immutable read — always safe to share
  String, Vec<T>    Yes     &String / &Vec are read-only — safe to share
  Arc<T>            Yes     (when T: Sync) — just a shared pointer
  Mutex<T>          Yes     Enforces exclusive access at runtime
  RwLock<T>         Yes     Multiple readers OR one writer
  Rc<T>             NO      Non-atomic ref count — unsafe to clone across threads
  Cell<T>           NO      Allows mutation through shared ref without locking
  RefCell<T>        NO      Runtime borrow checking — not thread-safe

  Rule of thumb:
    Shared read-only data → Arc<T> where T: Sync
    Shared mutable data   → Arc<Mutex<T>> or Arc<RwLock<T>>
"#);
}

// ─── 3. Arc<Mutex<T>> — THE SHARED MUTABLE STATE PATTERN ───────────────────
//
// The idiomatic way to share mutable state across threads:
//   Arc  — shared ownership (reference counted, atomic)
//   Mutex — exclusive write access (one thread at a time)
//
// C# equivalent:
//   private static readonly object _lock = new();
//   private static int _counter = 0;
//   lock (_lock) { _counter++; }
//
// Or with ConcurrentDictionary, Interlocked, etc.

fn demo_arc_mutex_pattern() {
    println!("--- 3. Arc<Mutex<T>> — Shared Mutable State ---\n");

    // Shared counter — multiple threads increment it:
    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut guard = c.lock().unwrap();  // blocks until lock acquired
            *guard += 1;
            // guard is dropped here → lock released automatically
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("  Counter after 5 threads: {}", *counter.lock().unwrap());

    // RwLock: multiple readers, one writer — like C# ReaderWriterLockSlim:
    let config = Arc::new(RwLock::new(String::from("initial")));

    {
        let mut write = config.write().unwrap();  // exclusive write lock
        *write = "updated".to_string();
    } // write lock released

    let r1 = Arc::clone(&config);
    let r2 = Arc::clone(&config);
    let h1 = thread::spawn(move || { println!("  Reader 1: {}", r1.read().unwrap()); });
    let h2 = thread::spawn(move || { println!("  Reader 2: {}", r2.read().unwrap()); });
    h1.join().unwrap();
    h2.join().unwrap();
    println!();

    println!(r#"  C#                                    Rust
  ─────────────────────────────────────────────────────────────────
  lock (_lock) {{ _val++; }}            let mut g = mutex.lock().unwrap(); *g += 1;
  new ReaderWriterLockSlim()            Arc::new(RwLock::new(data))
  rwl.EnterReadLock()                   rwlock.read().unwrap()
  rwl.EnterWriteLock()                  rwlock.write().unwrap()
  Interlocked.Increment(ref count)      Arc::new(Mutex::new(0))  (or std::sync::atomic)
  Interlocked.Add / CompareExchange     std::sync::atomic::AtomicI32
"#);
}

// ─── 4. TYPES THAT ARE NOT SEND OR NOT SYNC ─────────────────────────────────
//
// These are the types where the compiler WILL reject cross-thread usage.
// Understanding WHY each one is excluded builds intuition for the system.

fn demo_not_send_not_sync() {
    println!("--- 4. Types That Are Not Send / Not Sync ---\n");

    println!(r#"  // The following code does NOT compile — shown for educational purposes:

  // ── Rc<T> is NOT Send ───────────────────────────────────────────────
  // let rc = Rc::new(5);
  // thread::spawn(move || println!("{{}}", rc_value)); // ERROR: Rc<i32> cannot be sent between threads safely
  //
  // WHY: Rc uses a plain usize for ref count. Two threads incrementing
  //      it simultaneously is a data race → undefined behaviour.
  // FIX: Use Arc<T> instead — it uses atomic operations.

  // ── RefCell<T> is NOT Sync ──────────────────────────────────────────
  // let cell = Arc::new(RefCell::new(0));
  // let c = Arc::clone(&cell);
  // thread::spawn(move || {{ *c.borrow_mut() += 1; }}); // ERROR: RefCell<i32> cannot be shared between threads
  //
  // WHY: RefCell's borrow tracking uses non-atomic counters. Two threads
  //      calling borrow_mut() simultaneously → data race on the counter.
  // FIX: Use Arc<Mutex<T>> instead of Arc<RefCell<T>>.

  // ── MutexGuard<T> is NOT Send ───────────────────────────────────────
  // let mutex = Mutex::new(0);
  // let guard = mutex.lock().unwrap();
  // thread::spawn(move || {{ drop(guard); }}); // ERROR: MutexGuard cannot be sent between threads
  //
  // WHY: On some platforms (POSIX) a mutex must be unlocked by the same
  //      thread that locked it. Sending the guard crosses this boundary.

  Summary of unsafe patterns (all caught at compile time in Rust):
  ────────────────────────────────────────────────────────────────────
  Pattern                  C# outcome          Rust outcome
  ─────────────────────────────────────────────────────────────────
  Share Rc across threads  Runtime data race   Compile error
  Share RefCell across th. Runtime data race   Compile error
  Move MutexGuard to th.   Runtime deadlock    Compile error
  Share *mut T across th.  Undefined behavior  Compile error (requires unsafe)
"#);
}

// ─── 5. MAKING YOUR OWN TYPES SEND + SYNC ───────────────────────────────────
//
// Auto-trait rules: your struct is Send if ALL its fields are Send.
// Ditto for Sync. You never need to write impl Send/Sync manually
// unless you're wrapping a raw pointer (unsafe).

// This struct is automatically Send + Sync because all fields are:
struct SafeWrapper {
    data: Arc<Mutex<Vec<i32>>>,
    name: String,
}

// A wrapper around a raw pointer — NOT automatically Send:
struct RawPtr(*mut i32);

// To send it across threads you must assert safety manually with `unsafe`:
// SAFETY: We guarantee single-owner access — RawPtr is used in a context
//         where no other thread holds a reference to the pointed-to memory.
unsafe impl Send for RawPtr {}

fn demo_making_types_send_sync() {
    println!("--- 5. Auto-Impl vs Manual Send/Sync ---\n");

    // SafeWrapper is Send + Sync automatically — the compiler derives it:
    let wrapper = SafeWrapper {
        data: Arc::new(Mutex::new(vec![1, 2, 3])),
        name: "example".to_string(),
    };

    let handle = thread::spawn(move || {
        let mut guard = wrapper.data.lock().unwrap();
        guard.push(4);
        println!("  SafeWrapper name: {}", wrapper.name);
        println!("  Data: {:?}", *guard);
    });
    handle.join().unwrap();
    println!();

    println!(r#"  Auto-impl rules:
    struct Foo {{ field1: A, field2: B }}
    Foo: Send  ← if A: Send AND B: Send  (both fields must be Send)
    Foo: Sync  ← if A: Sync AND B: Sync  (both fields must be Sync)

  Manual impl (requires unsafe — you're promising the compiler):
    unsafe impl Send for MyType {{ }}
    unsafe impl Sync for MyType {{ }}

  Use PhantomData to opt OUT of Send/Sync:
    use std::marker::PhantomData;
    struct NotSend {{ _marker: PhantomData<*mut ()> }}
    // *mut () is !Send, so NotSend becomes !Send automatically
"#);
}

// ─── 6. QUICK REFERENCE ─────────────────────────────────────────────────────

fn demo_reference_guide() {
    println!("--- 6. Complete Reference ---\n");

    println!(r#"
  ┌─────────────────────┬──────────┬──────────┬────────────────────────────────────┐
  │ Type                │ Send     │ Sync     │ Notes                              │
  ├─────────────────────┼──────────┼──────────┼────────────────────────────────────┤
  │ i32, f64, bool, ... │ Yes      │ Yes      │ All primitives                     │
  │ String, Vec<T>      │ Yes      │ Yes      │ Owned data                         │
  │ Box<T>              │ if T:Send│ if T:Sync│ Single owner                       │
  │ Arc<T>              │ if T:Send│ if T:Sync│ Shared owner, atomic refcount      │
  │ Mutex<T>            │ if T:Send│ Yes      │ Lock guards exclusive access        │
  │ RwLock<T>           │ if T:Send│ Yes      │ Multi-reader / single-writer        │
  │ AtomicI32, etc.     │ Yes      │ Yes      │ Lock-free atomic ops                │
  │ Rc<T>               │ NO       │ NO       │ Non-atomic refcount                │
  │ Cell<T>             │ if T:Send│ NO       │ Shared mutation, not thread-safe   │
  │ RefCell<T>          │ if T:Send│ NO       │ Runtime borrow, not thread-safe    │
  │ MutexGuard<T>       │ NO       │ if T:Sync│ Must unlock on same thread         │
  │ *const T, *mut T    │ NO       │ NO       │ Raw pointers — unsafe to send      │
  └─────────────────────┴──────────┴──────────┴────────────────────────────────────┘

  Decision guide for shared data:
    Read-only, shared across threads?        →  Arc<T>
    Mutable, one writer at a time?           →  Arc<Mutex<T>>
    Mutable, many readers / one writer?      →  Arc<RwLock<T>>
    Mutable, single thread only?             →  Rc<RefCell<T>>
    Lock-free integer counter?               →  Arc<AtomicI32>
    Per-thread state (no sharing needed)?    →  thread_local!{{ }}

  C# comparison:
    Arc<Mutex<T>>  ≈  lock() + shared field
    Arc<T>         ≈  ImmutableObject shared across tasks
    Rc<T>          ≈  object passed within a single-threaded context
    AtomicI32      ≈  Interlocked.Increment
    thread_local!  ≈  [ThreadStatic]
"#);
}

// ─── TESTS ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_mutex_counter_is_correct() {
        let counter = Arc::new(Mutex::new(0_i32));
        let mut handles = vec![];
        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                *c.lock().unwrap() += 1;
            }));
        }
        for h in handles { h.join().unwrap(); }
        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn arc_rwlock_multiple_readers() {
        let data = Arc::new(RwLock::new(42_i32));
        let mut handles = vec![];
        for _ in 0..5 {
            let d = Arc::clone(&data);
            handles.push(thread::spawn(move || {
                *d.read().unwrap()  // multiple readers at once — no blocking
            }));
        }
        for h in handles {
            assert_eq!(h.join().unwrap(), 42);
        }
    }

    // Compile-time test: SafeWrapper must satisfy Send + Sync bounds:
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn safe_wrapper_is_send_and_sync() {
        assert_send_sync::<SafeWrapper>();
    }
}
