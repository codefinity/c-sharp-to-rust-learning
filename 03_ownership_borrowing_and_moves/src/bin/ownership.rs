// ============================================================
// CONCEPT: Ownership — Rust's Core Memory Safety Mechanism
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses a garbage collector: the runtime tracks all live references and
// periodically reclaims memory for unreachable objects. You never think about
// "who owns this object" — multiple references can coexist freely.
//
// Rust has NO garbage collector. Instead, the BORROW CHECKER enforces three rules:
//
//   RULE 1: Every value has exactly ONE owner (a variable binding).
//   RULE 2: There can only be one owner at a time.
//   RULE 3: When the owner goes out of scope, the value is DROPPED (freed).
//
// These rules are checked at COMPILE TIME — zero runtime overhead.
//
// The mental model: ownership ≈ a non-nullable, single-owner smart pointer
// that automatically destroys its value when it goes out of scope.
// C# analogy: IDisposable + using() { } enforced at compile time for ALL types.
//
// RUN: cargo run --bin ownership
// ============================================================

fn main() {
    scope_and_drop();
    heap_ownership();
    ownership_and_functions();
    returning_ownership();
    drop_order();
}

fn scope_and_drop() {
    println!("=== Scope and Drop ===");

    // Every variable is the owner of its value.
    // When the variable goes out of scope, its Drop is called automatically.
    {
        let s = String::from("hello"); // s comes into scope, heap allocated
        println!("inside scope: {s}");
    } // ← s goes out of scope here; String::drop() is called; memory freed

    // println!("{s}"); // ← compile error: `s` no longer in scope

    // Stack-allocated primitives are owned too, but "drop" just pops the stack:
    {
        let x: i32 = 42;
        println!("x = {x}");
    } // x's memory reclaimed here (trivially — it's the stack frame)

    println!("After scopes — no leaks, no GC pressure");
}

fn heap_ownership() {
    println!("\n=== Heap Ownership ===");

    // C#: var a = new StringBuilder("hello");
    //     var b = a; // b and a BOTH point to the same heap object
    //     a.Append(" world"); // modifies the shared object — b sees it too
    //     // GC collects it when both a and b are unreachable

    // Rust:
    let a = String::from("hello");
    // let b = a;  // this is a MOVE (see moves.rs), not a copy!
    // println!("{a}"); // ← compile error after a move

    // To have two independent copies, use .clone():
    let b = a.clone();
    println!("a = {a}  b = {b}");
    println!("a and b are separate heap allocations");

    // The key insight: only ONE variable owns the heap memory at any time.
    // When that variable is dropped, the memory is freed.
    // No reference counting, no GC, no double-free possible.
}

fn ownership_and_functions() {
    println!("\n=== Ownership and Functions ===");

    // Passing a String to a function MOVES ownership into the function.
    // After the call, the caller no longer owns the value.

    let s = String::from("moved into function");
    takes_ownership(s);
    // println!("{s}"); // ← compile error: value moved into `takes_ownership`

    // However, passing an i32 COPIES it (i32 implements Copy):
    let n: i32 = 42;
    makes_copy(n);
    println!("n is still usable: {n}"); // works fine

    // Solution 1: pass a reference (borrowing — covered in borrowing.rs)
    let s2 = String::from("borrowed");
    borrows_string(&s2);   // passes a reference, does NOT move
    println!("s2 still usable: {s2}");

    // Solution 2: pass a clone (separate copy on heap — more expensive)
    let s3 = String::from("cloned");
    takes_ownership(s3.clone()); // s3 is still valid
    println!("s3 still usable: {s3}");
}

fn takes_ownership(s: String) {
    // s is now the owner; when this function returns, s is dropped.
    println!("  took ownership of: {s}");
} // s dropped here

fn makes_copy(n: i32) {
    // n is a COPY of the caller's value (i32: Copy)
    println!("  copied value: {n}");
}

fn borrows_string(s: &String) {
    // &String is a reference — borrows without taking ownership
    println!("  borrowed: {s}");
}

fn returning_ownership() {
    println!("\n=== Returning Ownership ===");

    // A function can RETURN a value, transferring ownership to the caller.
    let s1 = gives_ownership();
    println!("s1 = {s1}");

    // Pass in, transform, return: ownership in → ownership out
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s2 is no longer valid (was moved into the function)
    println!("s3 = {s3}");

    // This is verbose — normally you'd use references instead.
    // But understanding this is key to understanding lifetimes later.
}

fn gives_ownership() -> String {
    let s = String::from("from gives_ownership");
    s // moves ownership to caller
}

fn takes_and_gives_back(s: String) -> String {
    s // simply return it — moves ownership back to caller
}

fn drop_order() {
    println!("\n=== Drop Order ===");

    // Values are dropped in REVERSE DECLARATION ORDER (stack discipline).
    // This matters for RAII patterns (mutexes, file handles, etc.)
    let _first  = DropTracer::new("first");
    let _second = DropTracer::new("second");
    let _third  = DropTracer::new("third");
    println!("about to leave scope...");
    // drops: third, second, first  (reverse order)
}

struct DropTracer {
    name: &'static str,
}

impl DropTracer {
    fn new(name: &'static str) -> Self {
        println!("  created: {name}");
        Self { name }
    }
}

impl Drop for DropTracer {
    fn drop(&mut self) {
        println!("  dropped: {}", self.name);
    }
}

// ─── THE THREE OWNERSHIP RULES ───────────────────────────────
// 1. Every value has exactly one owner.
// 2. Only one owner at a time.
// 3. Value is dropped when owner goes out of scope.

// ─── C# MENTAL MODEL ─────────────────────────────────────────
// Think of Rust ownership as: every heap object has exactly one
// `IDisposable using()` block, and the compiler proves it for you.
// There are no dangling references — the compiler prevents them.

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Trying to use a value after moving it — will not compile.
// • Calling .clone() everywhere as a workaround — usually means
//   you should use references (&T) instead.
// • Forgetting that passing to a function moves the value.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Write a function that takes a String, appends " world", and returns it.
// 2. Write two variables pointing to the "same" data using clone(), modify
//    one, and show they diverge.
// 3. Implement Drop for a custom struct that prints "cleaned up".
// 4. Explain why this doesn't compile and fix it:
//    let s = String::from("hi"); let t = s; println!("{}", s);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_ownership_returns_string() {
        let s = gives_ownership();
        assert!(!s.is_empty());
    }

    #[test]
    fn takes_and_gives_back_preserves_content() {
        let original = String::from("test");
        let returned = takes_and_gives_back(original);
        assert_eq!(returned, "test");
    }

    #[test]
    fn clone_is_independent() {
        let a = String::from("hello");
        let mut b = a.clone();
        b.push_str(" world");
        assert_eq!(a, "hello");       // a unchanged
        assert_eq!(b, "hello world"); // b modified
    }
}
