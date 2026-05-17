// ============================================================
// CONCEPT: Deref and Drop Traits
// ============================================================
// RUN: cargo run --bin deref_drop
// ============================================================

use std::ops::Deref;

fn main() {
    deref_trait();
    deref_coercion_chains();
    drop_trait();
    drop_order_and_raii();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

fn deref_trait() {
    println!("=== Deref Trait ===");

    let x = 5_i32;
    let y = MyBox::new(x);

    // Without Deref, we'd need: *y.deref()
    // With Deref, the compiler inserts deref calls automatically:
    assert_eq!(x, *y);         // *y == *(y.deref()) == 5
    println!("*y = {}", *y);

    // Regular references:
    let r = &x;
    println!("*r = {}", *r);

    // Deref coercion: &MyBox<String> → &String → &str
    let s = MyBox::new(String::from("hello"));
    println!("length via deref chain: {}", s.len()); // calls Deref repeatedly

    // How deref coercion works:
    // &MyBox<String> → &String (our Deref impl)
    // &String → &str (String's Deref impl)
    fn takes_str(s: &str) { println!("got: {s}"); }
    takes_str(&s); // &MyBox<String> coerces all the way to &str
}

fn deref_coercion_chains() {
    println!("\n=== Deref Coercion Chains ===");
    println!(
        r#"
Automatic coercions:
  &String     → &str       (String: Deref<Target=str>)
  &Vec<T>     → &[T]       (Vec: Deref<Target=[T]>)
  &Box<T>     → &T         (Box: Deref<Target=T>)
  &Arc<T>     → &T         (Arc: Deref<Target=T>)
  &Rc<T>      → &T         (Rc: Deref<Target=T>)
  &PathBuf    → &Path
  &OsString   → &OsStr

Mutable deref coercion (via DerefMut):
  &mut String → &mut str
  &mut Vec<T> → &mut [T]
  &mut Box<T> → &mut T
"#
    );

    // Vec<T> → &[T]:
    fn sum_slice(s: &[i32]) -> i32 { s.iter().sum() }
    let v = vec![1, 2, 3, 4, 5];
    println!("sum via deref coercion: {}", sum_slice(&v)); // &Vec → &[T]

    // Box<T> → &T:
    let boxed = Box::new(42_i32);
    println!("boxed + 1 = {}", *boxed + 1); // *Box<i32> → i32
}

struct Resource {
    name: String,
}

impl Resource {
    fn new(name: &str) -> Self {
        println!("  [Resource] created: {name}");
        Resource { name: name.to_string() }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  [Resource] dropped: {}", self.name);
    }
}

fn drop_trait() {
    println!("=== Drop Trait (Deterministic Destruction) ===");
    println!("C# uses finalizers (GC-called, non-deterministic) and IDisposable.");
    println!("Rust uses Drop — called deterministically when variable goes out of scope.");
    println!("This is guaranteed — no GC, no finalizer thread.\n");

    {
        let _a = Resource::new("a");
        let _b = Resource::new("b");
        let _c = Resource::new("c");
        println!("  (leaving scope...)");
    } // drops in reverse order: c, b, a

    println!("  (after scope)");

    // Manual early drop — transfer cleanup control:
    let early = Resource::new("early");
    println!("  (calling drop explicitly)");
    drop(early); // calls Drop::drop, then forgets the variable
    println!("  (early is gone now)");
}

fn drop_order_and_raii() {
    println!("\n=== RAII (Resource Acquisition Is Initialisation) ===");

    println!(
        r#"
RAII means: constructor acquires resource, destructor releases it.
Rust enforces this perfectly because Drop is always called exactly once.

Examples of RAII in Rust std:
  File         — closes the OS file handle on drop
  MutexGuard   — unlocks the mutex on drop (no explicit unlock!)
  TcpStream    — closes the TCP connection on drop
  BufWriter    — flushes the buffer on drop
  thread::JoinHandle — if not joined, detaches the thread on drop

C# comparison:
  IDisposable.Dispose()   — manual, requires `using` or explicit call
  Finalizer (~ClassName)  — GC-called, non-deterministic

Rust guarantees:
  ✓ Drop is always called (no memory leaks in safe Rust)
  ✓ Drop order is deterministic (reverse declaration order)
  ✓ No double-drop possible (ownership ensures one owner)
"#
    );

    // File as RAII demo:
    {
        use std::io::Write;
        let mut f = std::fs::File::create("raii_demo.txt").unwrap();
        writeln!(f, "written via RAII").unwrap();
        println!("file created");
    } // f.drop() called here — OS file handle closed
    println!("file closed (RAII)");

    // Clean up:
    let _ = std::fs::remove_file("raii_demo.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mybox_deref() {
        let b = MyBox::new(42_i32);
        assert_eq!(*b, 42);
    }

    #[test]
    fn drop_order() {
        use std::cell::RefCell;
        let order: RefCell<Vec<i32>> = RefCell::new(Vec::new());

        struct Tracked<'a> { id: i32, order: &'a RefCell<Vec<i32>> }
        impl<'a> Drop for Tracked<'a> {
            fn drop(&mut self) { self.order.borrow_mut().push(self.id); }
        }

        {
            let _a = Tracked { id: 1, order: &order };
            let _b = Tracked { id: 2, order: &order };
            let _c = Tracked { id: 3, order: &order };
        }

        assert_eq!(*order.borrow(), vec![3, 2, 1]); // reverse order
    }
}
