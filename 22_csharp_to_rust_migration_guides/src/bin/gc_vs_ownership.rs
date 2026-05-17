// ============================================================
// MIGRATION GUIDE: GC vs Ownership
// ============================================================
//
// C# runtime: the GC tracks references, collects unreachable objects,
// compacts the heap, and runs finalizers asynchronously.
//
// Rust: the compiler tracks ownership at compile time.
// Memory is freed deterministically at the end of the owning scope.
// There is no GC, no pauses, no non-deterministic finalization.
//
// RUN: cargo run --bin gc_vs_ownership
// ============================================================

fn main() {
    println!("=== GC vs Ownership ===\n");

    ownership_rules();
    deterministic_drop();
    sharing_patterns();
    gc_concepts_in_rust();
}

fn ownership_rules() {
    println!("--- Ownership Rules ---");

    println!(r#"
C# Memory Model:
  • Objects live on the managed heap
  • Multiple references can point to the same object
  • GC runs when memory pressure is high (non-deterministic)
  • Finalizers (~MyClass()) run asynchronously, order not guaranteed
  • Heap fragmentation is handled by compaction (can cause pauses)

Rust Memory Model:
  • Every value has exactly ONE owner
  • When the owner goes out of scope, the value is DROPPED (memory freed)
  • Ownership can be MOVED (transferred) or BORROWED (&T, &mut T)
  • The compiler tracks this at compile time — no runtime overhead
  • Memory is freed deterministically: end-of-scope = immediate free
"#);

    // Ownership demo:
    {
        let s = String::from("hello"); // s owns the heap memory
        println!("  owned string: {s}");
        // s goes out of scope here — memory freed immediately
    }
    println!("  s was dropped when the block ended");
}

fn deterministic_drop() {
    println!("\n--- Deterministic Drop (RAII) ---");

    struct Resource(String);
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  [DROP] {} freed", self.0);
        }
    }

    println!(r#"
C# Finalizers vs Rust Drop:
  C# ~Finalizer()  → runs on GC thread, non-deterministic timing
  C# IDisposable   → Dispose() called explicitly or by using statement
  Rust Drop trait  → runs synchronously, exactly when scope ends

C# using (var r = new Resource()) {{ ... }}   // Dispose at end of block
Rust {{ let r = Resource::new(); ... }}        // Drop at end of block (automatic!)
"#);

    {
        let a = Resource("FileHandle-A".to_string());
        let b = Resource("Connection-B".to_string());
        println!("  using a and b...");
        // b dropped first (LIFO), then a
    }
    println!("  both resources freed");
}

fn sharing_patterns() {
    println!("\n--- Sharing Patterns ---");

    println!(r#"
C# sharing: multiple variables can reference the same object freely.
Rust sharing: controlled via borrowing or reference-counted smart pointers.

 C#                         | Rust equivalent
----------------------------|----------------------------------------------
 var a = obj; var b = obj;  | let b = &a;           (shared borrow)
 Multiple owners of heap    | Arc<T>                 (ref-counted)
 Mutable from multiple refs | Arc<Mutex<T>>          (thread-safe interior mut)
 Mutable single-thread      | Rc<RefCell<T>>         (single-thread)
 Clone the whole object     | .clone()               (deep copy)
"#);

    use std::sync::{Arc, Mutex};

    // Arc<Mutex<T>> replaces "two variables pointing to same mutable object":
    let shared: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
    let shared2 = Arc::clone(&shared);

    shared.lock().unwrap().push(4);
    println!("  shared via Arc<Mutex>: {:?}", shared2.lock().unwrap());
}

fn gc_concepts_in_rust() {
    println!("\n--- GC Concepts Mapped to Rust ---");

    println!(r#"
Concept               | C#                  | Rust
----------------------|---------------------|------------------------------
Object lifetime       | GC decides          | Owner's scope
Memory release        | GC collects         | Drop at end of scope
Multiple references   | All allowed         | Borrow rules (& or Arc)
Mutable sharing       | Always allowed      | Only one &mut at a time
Cyclic references     | GC handles          | Weak<T> to break cycles
Finalization          | ~Destructor()       | impl Drop for T
Pinning               | GC.KeepAlive        | Pin<T>
Large Object Heap     | GC LOH (>85KB)      | Box::new allocates on heap
Allocation            | new anywhere        | let x: Box<T> = Box::new(...)
Stack allocation      | struct (value type) | let x: T (any non-Box T)

GC pauses:
  C#  → stop-the-world (Gen2), or background GC with short pauses
  Rust → no GC, no pauses — deterministic alloc/free

Memory fragmentation:
  C#  → compaction can relocate objects (hence pinning for P/Invoke)
  Rust → no compaction; allocator manages fragmentation

Weak references:
  C#  → WeakReference<T> — GC can collect despite weak ref
  Rust → Weak<T> (from Rc/Arc) — does not prevent drop
"#);

    // Weak reference example:
    use std::rc::{Rc, Weak};

    let strong = Rc::new(String::from("hello"));
    let weak: Weak<String> = Rc::downgrade(&strong);

    println!("  strong count: {}", Rc::strong_count(&strong));
    println!("  weak   count: {}", Rc::weak_count(&strong));
    println!("  upgrade: {:?}", weak.upgrade());

    drop(strong); // freed because no more strong refs
    println!("  after drop — upgrade returns: {:?}", weak.upgrade()); // None
}
