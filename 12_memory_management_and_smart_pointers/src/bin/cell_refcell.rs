// ============================================================
// CONCEPT: Cell<T> and RefCell<T> — Interior Mutability
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust's borrow checker normally prevents mutation through shared references.
// Interior mutability types let you mutate data even when you only have &T.
//
//   Cell<T>    — for Copy types; get/set without references
//   RefCell<T> — for any T; runtime borrow checking (dynamic borrow)
//
// C# analogy: these are like `mutable` fields in a readonly struct,
// or fields that can be modified through a read-only interface.
// The borrow checking moves from COMPILE TIME to RUNTIME.
//
// Use SPARINGLY — prefer normal ownership when possible.
// Common use cases: internal mutable caches, lazy init, callback states.
//
// RUN: cargo run --bin cell_refcell
// ============================================================

use std::cell::{Cell, RefCell};

fn main() {
    cell_basics();
    refcell_basics();
    interior_mutability_pattern();
    refcell_with_rc();
    common_panics();
}

fn cell_basics() {
    println!("=== Cell<T> (Copy types only) ===");

    // Cell<T> allows mutation through a shared reference.
    // Works only for Copy types (i32, bool, f64, etc.)
    let cell = Cell::new(5_i32);

    println!("cell.get() = {}", cell.get());  // reads
    cell.set(10);                              // writes
    println!("after set(10): {}", cell.get());

    // Useful for shared mutable counters in closures:
    let counter = Cell::new(0_u32);
    let inc = || counter.set(counter.get() + 1);
    inc(); inc(); inc();
    println!("counter = {}", counter.get()); // 3

    // Cell in a struct — allows mutation through &self (not &mut self):
    struct Stats {
        hits:   Cell<u32>,
        misses: Cell<u32>,
    }

    impl Stats {
        fn new() -> Self { Stats { hits: Cell::new(0), misses: Cell::new(0) } }
        fn hit(&self)  { self.hits.set(self.hits.get() + 1);   } // &self, not &mut self
        fn miss(&self) { self.misses.set(self.misses.get() + 1); }
        fn ratio(&self) -> f64 {
            let h = self.hits.get() as f64;
            let t = (h + self.misses.get() as f64).max(1.0);
            h / t
        }
    }

    let stats = Stats::new();
    stats.hit(); stats.hit(); stats.miss();
    println!("hit ratio: {:.2}", stats.ratio());
}

fn refcell_basics() {
    println!("\n=== RefCell<T> (dynamic borrow checking) ===");

    let rf = RefCell::new(vec![1, 2, 3]);

    // borrow() — like &T (panics if mutable borrow exists)
    let r1 = rf.borrow();        // immutable borrow
    let r2 = rf.borrow();        // second immutable borrow — OK
    println!("r1 = {r1:?}  r2 = {r2:?}");
    drop(r1); drop(r2);          // release borrows

    // borrow_mut() — like &mut T (panics if ANY borrow exists)
    {
        let mut rm = rf.borrow_mut(); // mutable borrow
        rm.push(4);
        rm.push(5);
    } // mutable borrow released
    println!("after push: {:?}", rf.borrow());

    // try_borrow() / try_borrow_mut() — non-panicking versions:
    let maybe = rf.try_borrow_mut();
    println!("try_borrow_mut ok: {}", maybe.is_ok());
}

fn interior_mutability_pattern() {
    println!("\n=== Interior Mutability Pattern ===");

    // Classic use case: lazy computation / caching
    struct LazyCache {
        input:  String,
        cached: RefCell<Option<String>>,
    }

    impl LazyCache {
        fn new(input: &str) -> Self {
            LazyCache {
                input: input.to_string(),
                cached: RefCell::new(None),
            }
        }

        fn get_processed(&self) -> std::cell::Ref<'_, Option<String>> {
            // Compute only once, cache the result
            if self.cached.borrow().is_none() {
                let result = self.input.to_uppercase();
                *self.cached.borrow_mut() = Some(result);
            }
            self.cached.borrow()
        }
    }

    let cache = LazyCache::new("hello world");
    println!("first: {:?}", *cache.get_processed());
    println!("second (cached): {:?}", *cache.get_processed());
}

fn refcell_with_rc() {
    println!("\n=== Rc<RefCell<T>> — Shared Mutable State ===");

    // The classic Rust pattern for shared mutable data on a single thread.
    // C# equivalent: multiple references to a mutable object.
    // Combined: Rc (shared ownership) + RefCell (interior mutability).

    use std::rc::Rc;

    let shared = Rc::new(RefCell::new(Vec::<i32>::new()));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    // Both `a` and `b` can mutate the shared Vec:
    a.borrow_mut().push(1);
    b.borrow_mut().push(2);
    a.borrow_mut().push(3);

    println!("shared: {:?}", shared.borrow());

    // For multi-threaded equivalent: Arc<Mutex<T>> (see mutex_rwlock.rs)
}

fn common_panics() {
    println!("\n=== RefCell Panics (Runtime Borrow Errors) ===");

    // This would PANIC at runtime — not a compile error:
    // let rf = RefCell::new(5);
    // let _r = rf.borrow();    // immutable borrow alive
    // let _w = rf.borrow_mut(); // PANIC: already borrowed!

    // Using try_borrow_mut to avoid panic:
    let rf = RefCell::new(5_i32);
    let _r = rf.borrow(); // immutable borrow active
    match rf.try_borrow_mut() {
        Ok(_) => println!("got mutable borrow"),
        Err(e) => println!("borrow failed (expected): {e}"),
    }
    // _r dropped here — now mutable borrow would succeed
    drop(_r);
    let mut w = rf.borrow_mut();
    *w = 42;
    println!("after mutation: {}", *w);
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[test]
    fn cell_get_set() {
        let c = Cell::new(1_i32);
        c.set(99);
        assert_eq!(c.get(), 99);
    }

    #[test]
    fn refcell_borrow_mut() {
        let rf = RefCell::new(vec![1, 2, 3]);
        rf.borrow_mut().push(4);
        assert_eq!(*rf.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn rc_refcell_shared_mutation() {
        let shared = Rc::new(RefCell::new(0_i32));
        let a = Rc::clone(&shared);
        *a.borrow_mut() += 10;
        *shared.borrow_mut() += 5;
        assert_eq!(*shared.borrow(), 15);
    }
}
