// ============================================================
// CONCEPT: Rc<T> and Arc<T> — Reference Counting
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# GC tracks all references automatically. Rust needs explicit
// shared ownership via reference counting.
//
//   Rc<T>  — Reference Counted (single-threaded only)
//             ≈ C# object shared between multiple local holders
//   Arc<T> — Atomic Reference Counted (thread-safe)
//             ≈ C# object shared across threads
//
// Think of Rc/Arc as "a GC that you control explicitly for specific objects."
//
// WHEN TO USE:
//   Rc<T>  — multiple ownership within a single thread (graphs, trees with
//             parent pointers)
//   Arc<T> — sharing data across threads without copying
//
// RUN: cargo run --bin rc_arc
// ============================================================

use std::rc::Rc;
use std::sync::Arc;

fn main() {
    rc_basics();
    rc_shared_state();
    weak_references();
    arc_threading();
    rc_vs_arc_comparison();
}

fn rc_basics() {
    println!("=== Rc<T> (Single-Threaded Shared Ownership) ===");

    let a = Rc::new(String::from("shared data"));
    let b = Rc::clone(&a); // clone the Rc (increments ref count), NOT the data
    let c = Rc::clone(&a);

    println!("a = {a}  b = {b}  c = {c}");
    println!("strong count = {}", Rc::strong_count(&a)); // 3

    drop(b);
    println!("after drop(b): count = {}", Rc::strong_count(&a)); // 2

    // When all Rc handles are dropped, the data is freed:
    drop(c);
    println!("after drop(c): count = {}", Rc::strong_count(&a)); // 1
    // a dropped at end of scope → count reaches 0 → memory freed

    // Rc is NOT Clone in the deep-copy sense — it clones the POINTER:
    println!("size of Rc<String>: {} bytes", std::mem::size_of::<Rc<String>>());
}

fn rc_shared_state() {
    println!("\n=== Rc for Shared Graph Nodes ===");

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<Self> {
            Rc::new(Self { value, children: Vec::new() })
        }
    }

    // Shared child — referenced from two parents:
    let shared_child = Node::new(100);
    println!("shared_child count before sharing: {}", Rc::strong_count(&shared_child));

    let parent1 = Rc::new(Node {
        value: 1,
        children: vec![Rc::clone(&shared_child)],
    });
    let parent2 = Rc::new(Node {
        value: 2,
        children: vec![Rc::clone(&shared_child)],
    });

    println!("shared_child count with two parents: {}", Rc::strong_count(&shared_child)); // 3

    println!("parent1.children[0].value = {}", parent1.children[0].value);
    println!("parent2.children[0].value = {}", parent2.children[0].value);
    // Both parents share the SAME child node — no duplication
}

fn weak_references() {
    println!("\n=== Weak<T> — Breaking Reference Cycles ===");

    // Rc cycle: A → B → A would prevent either from being freed!
    // C# GC handles cycles; Rc does NOT.
    // Solution: Weak<T> — a non-owning reference (doesn't bump ref count)

    use std::rc::Weak;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,    // weak reference to parent (no cycle)
        children: RefCell<Vec<Rc<Node>>>, // strong references to children
    }

    impl Node {
        fn new(val: i32) -> Rc<Self> {
            Rc::new(Node {
                value: val,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(Vec::new()),
            })
        }
    }

    let root = Node::new(1);
    let child = Node::new(2);

    // Child knows its parent (weak — doesn't prevent root from being freed):
    *child.parent.borrow_mut() = Rc::downgrade(&root);

    // Root knows its child (strong):
    root.children.borrow_mut().push(Rc::clone(&child));

    println!("root strong count: {}", Rc::strong_count(&root)); // 1 (child weak → root doesn't count)
    println!("child strong count: {}", Rc::strong_count(&child)); // 2 (root + child var)

    // Upgrade weak to strong (returns Option<Rc<T>>):
    let maybe_parent = child.parent.borrow().upgrade();
    match maybe_parent {
        Some(parent) => println!("parent value: {}", parent.value),
        None         => println!("parent was freed"),
    }
}

fn arc_threading() {
    println!("\n=== Arc<T> (Thread-Safe Shared Ownership) ===");

    // Arc = Atomically Reference Counted — uses atomic operations for ref count
    // Safe to clone and send across threads.
    // Immutable by default — combine with Mutex for mutation.

    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);

    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&shared_data); // cheap clone — just pointer copy
        let handle = std::thread::spawn(move || {
            let sum: i32 = data.iter().sum();
            println!("  thread {i}: sum = {sum}");
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }

    println!("main: count = {}", Arc::strong_count(&shared_data));

    // Arc<Mutex<T>> for shared mutable state:
    use std::sync::Mutex;
    let counter = Arc::new(Mutex::new(0_i32));

    let mut handles2 = vec![];
    for _ in 0..5 {
        let c = Arc::clone(&counter);
        let h = std::thread::spawn(move || {
            let mut guard = c.lock().unwrap();
            *guard += 1;
        });
        handles2.push(h);
    }
    for h in handles2 { h.join().unwrap(); }
    println!("counter: {}", *counter.lock().unwrap()); // 5
}

fn rc_vs_arc_comparison() {
    println!("\n=== Rc vs Arc ===");
    println!(
        r#"
Feature         | Rc<T>              | Arc<T>
----------------+--------------------+----------------------
Thread safety   | Single-threaded    | Multi-threaded (Send+Sync)
Performance     | Faster (no atomics)| Slower (atomic ref count)
Mutation        | + RefCell<T>       | + Mutex<T> or RwLock<T>
Weak pointer    | Weak<T>            | Weak<T> (arc::Weak)
Use case        | Graphs, trees, UI  | Shared data across threads

C# GC analogy:
  Rc<T>  ≈ a mini GC for a specific object on one thread
  Arc<T> ≈ a mini GC for a specific object across threads
  Both are freed when the last reference is dropped — deterministic!
"#
    );
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn rc_count() {
        let a = Rc::new(42);
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn arc_thread_sharing() {
        let data = Arc::new(vec![1, 2, 3]);
        let d2 = Arc::clone(&data);
        let handle = std::thread::spawn(move || {
            d2.iter().sum::<i32>()
        });
        let result = handle.join().unwrap();
        assert_eq!(result, 6);
    }
}
