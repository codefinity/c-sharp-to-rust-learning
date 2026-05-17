// ============================================================
// CONCEPT: Dangling References — What Rust Prevents
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C#, the GC ensures that as long as a reference exists, the object it
// points to is alive. Dangling references are impossible at the language level.
//
// In Rust (no GC), dangling references would cause undefined behaviour.
// The borrow checker prevents them at compile time.
//
// This file shows WHAT WOULD GO WRONG without the borrow checker,
// and how Rust's rules prevent each class of problem.
//
// RUN: cargo run --bin dangling_refs
// ============================================================

fn main() {
    use_after_free_prevented();
    iterator_invalidation_prevented();
    safe_alternatives();
    understanding_error_messages();
}

fn use_after_free_prevented() {
    println!("=== Use-After-Free: Prevented by Borrow Checker ===");

    // The following code is COMMENTED OUT because it will NOT compile.
    // It demonstrates what would be a use-after-free bug in C/C++:
    //
    // fn dangle() -> &String {       // ← returns reference to local
    //     let s = String::from("hi"); // s is created here
    //     &s                          // we return a reference to s
    // }                               // s is DROPPED here!
    //                                 // the returned reference would dangle!
    //
    // let ref_to_nothing = dangle(); // ← compile error in Rust!

    // The fix: return an owned value (transfer ownership):
    fn no_dangle() -> String {
        let s = String::from("hi");
        s // ownership moves to caller — no drop, no dangle
    }
    let s = no_dangle();
    println!("safe owned return: {s}");

    // Or accept the data and return a slice of it:
    fn first_half(s: &str) -> &str {
        // Borrow checker ensures return borrows from input `s`
        &s[..s.len() / 2]
    }
    let sentence = "Hello World";
    let half = first_half(sentence);
    println!("first half: '{half}'");
}

fn iterator_invalidation_prevented() {
    println!("\n=== Iterator Invalidation: Prevented by Borrow Checker ===");

    // In C#, modifying a collection while iterating throws InvalidOperationException
    // at RUNTIME. Rust prevents this at COMPILE TIME.
    //
    // ILLEGAL (won't compile):
    // let mut v = vec![1, 2, 3, 4, 5];
    // for x in &v {
    //     if *x == 3 { v.push(6); } // ← compile error: cannot borrow `v` as
    // }                              //   mutable because it is also borrowed
    //                                //   as immutable

    // Correct pattern: collect changes, apply after iteration:
    let v = vec![1, 2, 3, 4, 5];
    let mut to_add = Vec::new();

    for &x in &v {
        if x == 3 {
            to_add.push(x * 2); // collect, don't modify v yet
        }
    }

    let mut v = v; // rebind as mutable (v's borrow from for loop is done)
    v.extend(to_add);
    println!("v after safe modification: {v:?}");

    // Or use retain for filtering:
    let mut nums = vec![1, 2, 3, 4, 5, 6];
    nums.retain(|&x| x % 2 == 0); // remove while iterating — safe API
    println!("evens: {nums:?}");
}

fn safe_alternatives() {
    println!("\n=== Safe Alternatives for Common Patterns ===");

    // PATTERN 1: Store a reference to a Vec element while modifying the Vec.
    // ❌ Unsafe:
    //   let mut v = vec![1, 2, 3];
    //   let first = &v[0];     // immutable borrow
    //   v.push(4);             // mutable borrow ← compile error
    //   println!("{first}");

    // ✅ Safe version A: use the index, not a reference
    let mut v = vec![1, 2, 3];
    let first_idx = 0;
    v.push(4);
    println!("first: {}", v[first_idx]); // index is Copy, always safe

    // ✅ Safe version B: clone the value
    let mut v2 = vec![String::from("a"), String::from("b")];
    let first_clone = v2[0].clone(); // own a copy
    v2.push(String::from("c"));
    println!("first_clone: {first_clone}  v2: {v2:?}");

    // PATTERN 2: Self-referential structs.
    // Rust makes self-referential structs difficult — use indices instead.
    // (Advanced: Pin<T> enables them, covered in module 14.)

    // PATTERN 3: Returning a reference to a temporary.
    // ❌ fn bad() -> &str { &String::from("temp") }   // compile error
    // ✅ Return owned: fn good() -> String { "temp".to_string() }

    let owned: String = "owned return".to_string();
    println!("{owned}");
}

fn understanding_error_messages() {
    println!("\n=== Reading Borrow Checker Error Messages ===");

    println!(
        r#"
COMMON ERROR: cannot borrow `x` as mutable because it is also borrowed as immutable
→ You have an immutable borrow active when you try to mutate.
→ Fix: ensure all immutable borrows end before the mutable borrow.
→ Use NLL: borrow ends at LAST USE, not scope end.

COMMON ERROR: cannot move out of `x` because it is borrowed
→ You tried to move x while a reference to it exists.
→ Fix: clone x before moving, or restructure to borrow instead.

COMMON ERROR: x does not live long enough
→ A reference outlives the value it points to.
→ Fix: make the value live longer (move it to outer scope),
       or return an owned value instead of a reference.

COMMON ERROR: lifetime may not live long enough
→ A function returns a reference that might dangle.
→ Fix: add lifetime annotations to relate input and output lifetimes.

READING THE ERROR:
  error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    --> src/main.rs:5:5
     |
  4  |     let r = &v;        ← immutable borrow occurs here
  5  |     v.push(4);         ← mutable borrow occurs here
  6  |     println!("{{r}}");   <- immutable borrow later used here
     |
  The '→ later used here' line tells you WHY the borrow is still live.
  Removing that last use (or moving it before the mutation) fixes it.
"#
    );
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. C# uses GC to prevent dangling refs at runtime; Rust prevents at compile time.
// 2. Iterator invalidation is a runtime exception in C#; compile error in Rust.
// 3. Rust error messages are detailed and point to the exact conflict.
// 4. No undefined behaviour from dangling refs in safe Rust — ever.
// 5. The borrow checker "error" is the compiler helping you, not blocking you.

#[cfg(test)]
mod tests {
    #[test]
    fn retain_filters_in_place() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn no_dangle_returns_owned() {
        fn no_dangle() -> String {
            String::from("hi")
        }
        let s = no_dangle();
        assert_eq!(s, "hi");
    }
}
