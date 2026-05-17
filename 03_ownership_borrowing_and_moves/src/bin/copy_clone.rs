// ============================================================
// CONCEPT: Copy and Clone Traits
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# distinguishes value types (struct) from reference types (class):
//   • Structs are copied on assignment (unless passed by ref)
//   • Classes share references on assignment
//
// Rust distinguishes Copy types from non-Copy types:
//   • Copy types: IMPLICITLY copied on assignment (bitwise copy)
//   • Non-Copy types: MOVED on assignment (ownership transfer)
//
// `Copy` — a marker trait meaning "safe to bitwise copy".
//   Cannot be implemented if the type contains non-Copy fields (e.g., String).
//
// `Clone` — an explicit deep-copy operation (like .MemberwiseClone() in C#).
//   Must be called explicitly with `.clone()`.
//
// All `Copy` types are also `Clone`, but NOT vice versa.
//
// RUN: cargo run --bin copy_clone
// ============================================================

fn main() {
    copy_types_demo();
    clone_demo();
    implementing_copy();
    implementing_clone();
    copy_vs_clone_performance();
}

fn copy_types_demo() {
    println!("=== Copy Types ===");

    // These types implement Copy — assignment duplicates the value:
    let x: i32  = 42;
    let y = x;       // x is COPIED, not moved
    println!("x={x} y={y}"); // both still valid!

    // All primitive scalars are Copy:
    let a: bool   = true;
    let b: f64    = 3.14;
    let c: char   = 'R';
    let d: usize  = 100;
    let _a2 = a; let _b2 = b; let _c2 = c; let _d2 = d;
    // a, b, c, d all still valid here

    // Tuples of Copy types are Copy:
    let t1 = (1_i32, 2.0_f64, true);
    let t2 = t1; // copied
    println!("t1={t1:?} t2={t2:?}");

    // Arrays of Copy types are Copy:
    let arr1 = [1_i32; 5];
    let arr2 = arr1; // copied
    println!("arr1={arr1:?} arr2={arr2:?}");

    // References (&T) are Copy — the pointer is copied, not the data:
    let s = String::from("hello");
    let r1: &String = &s;
    let r2 = r1; // r1 is copied — both r1 and r2 point to s
    println!("r1={r1} r2={r2}");

    println!("\nNOT Copy: String, Vec, Box, and any type with heap allocation");
}

fn clone_demo() {
    println!("\n=== Clone (explicit deep copy) ===");

    // String does NOT implement Copy (heap allocated) — use .clone():
    let s1 = String::from("hello");
    let s2 = s1.clone(); // explicit deep copy — two separate heap allocations
    println!("s1={s1} s2={s2}");

    let mut s3 = s1.clone();
    s3.push_str(" world");
    println!("s1={s1}  s3={s3}"); // s1 unchanged

    // Vec clone:
    let v1 = vec![1, 2, 3];
    let mut v2 = v1.clone();
    v2.push(4);
    println!("v1={v1:?} v2={v2:?}");

    // Cloning is O(n) for heap types — use references where possible.
}

// Types that implement Copy:
#[derive(Debug, Clone, Copy)] // derive both Clone and Copy
struct Point {
    x: f64,
    y: f64,
}

// Types that implement only Clone (because they contain non-Copy fields):
#[derive(Debug, Clone)]
struct NamedPoint {
    name: String,  // String is not Copy
    x: f64,
    y: f64,
}

fn implementing_copy() {
    println!("\n=== Custom Copy Type ===");

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // COPIED (not moved) because Point: Copy
    println!("p1={p1:?} p2={p2:?}"); // both valid!

    // You can derive Copy only if ALL fields are Copy.
    // Point has two f64 fields — both Copy — so Point can be Copy.
}

fn implementing_clone() {
    println!("\n=== Custom Clone Type ===");

    let np1 = NamedPoint {
        name: String::from("Origin"),
        x: 0.0,
        y: 0.0,
    };
    let np2 = np1.clone(); // explicit deep copy
    // np1 and np2 are separate
    println!("np1={np1:?}");
    println!("np2={np2:?}");

    // np1 is still valid (we cloned, didn't move)
    drop(np1); // explicit drop (usually not needed)
    println!("np2 still valid: {np2:?}");

    // Custom Clone implementation (when derive isn't enough):
    #[derive(Debug)]
    struct Counter {
        value: i32,
        clones: std::sync::Arc<std::sync::atomic::AtomicI32>,
    }

    impl Clone for Counter {
        fn clone(&self) -> Self {
            self.clones.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Counter {
                value: self.value,
                clones: self.clones.clone(),
            }
        }
    }

    let tracker = std::sync::Arc::new(std::sync::atomic::AtomicI32::new(0));
    let c1 = Counter { value: 42, clones: tracker.clone() };
    let _c2 = c1.clone();
    let _c3 = c1.clone();
    println!("Counter cloned {} times", tracker.load(std::sync::atomic::Ordering::Relaxed));
}

fn copy_vs_clone_performance() {
    println!("\n=== Copy vs Clone Performance ===");
    println!(
        r#"
Copy:
  - Zero runtime cost — bitwise duplicate
  - Implicit (no .clone() needed)
  - Only valid for stack-sized, bitwise-safe types
  - Cannot contain Box, Vec, String, Rc, etc.

Clone:
  - Explicit deep copy — O(n) for heap types
  - Must be called with .clone()
  - Can do arbitrary work (custom Clone impl)
  - Always available alongside Copy

Guideline:
  - Primitive types: Copy is fine and automatic
  - Heap-owning types: use &T (borrow) to avoid cloning
  - Only clone when you NEED two independent copies
"#
    );
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. Copy is a trait, not a built-in distinction (like value vs ref type).
// 2. Clone must be called explicitly — no silent heap copies.
// 3. A type with String/Vec fields CANNOT be Copy.
// 4. `#[derive(Clone, Copy)]` works when all fields implement them.
// 5. Rust encourages borrowing (&T) over cloning to share data.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_copy() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1;
        // p1 still usable — it was copied
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }

    #[test]
    fn named_point_clone_is_independent() {
        let np1 = NamedPoint { name: "A".into(), x: 1.0, y: 2.0 };
        let mut np2 = np1.clone();
        np2.name = "B".into();
        assert_eq!(np1.name, "A");
        assert_eq!(np2.name, "B");
    }

    #[test]
    fn i32_is_copy() {
        let a = 10_i32;
        let b = a;
        assert_eq!(a, b); // a still valid
    }
}
