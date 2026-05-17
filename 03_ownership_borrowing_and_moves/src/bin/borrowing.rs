// ============================================================
// CONCEPT: Borrowing and References
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C# you pass objects around freely — the GC ensures they're not
// collected as long as any reference exists. In Rust, passing
// ownership is expensive (moves can be disruptive). Borrowing lets you
// USE a value without taking ownership of it.
//
// BORROWING RULES (compile-time enforced):
//   AT ANY GIVEN TIME, for a value, you may have EITHER:
//     • Any number of IMMUTABLE references (&T)
//     • OR exactly ONE MUTABLE reference (&mut T)
//   But NEVER both at the same time.
//
// This is the "readers-writer lock at compile time" model.
// It prevents data races at compile time — no need for runtime locking
// on single-threaded code.
//
// C# analogy: like ReadWriteLockSlim but enforced by the compiler.
//
// RUN: cargo run --bin borrowing
// ============================================================

fn main() {
    immutable_references();
    mutable_references();
    borrow_rules_demo();
    string_slices_as_borrows();
    borrowing_in_structs();
}

fn immutable_references() {
    println!("=== Immutable References (&T) ===");

    let s = String::from("hello");

    // Create an immutable reference — does NOT take ownership:
    let r1 = &s;
    let r2 = &s; // multiple immutable references are fine
    println!("r1={r1} r2={r2} s={s}"); // all three usable

    // Passing to a function that accepts &String:
    let len = string_length(&s);
    println!("length = {len}");
    println!("s still valid: {s}"); // s was only borrowed, not moved

    // &T coerces to &U when Deref<Target=U> is implemented:
    // &String coerces to &str automatically (Deref coercion):
    let len2 = str_length(&s); // &String passed where &str expected
    println!("len2 = {len2}");
}

fn string_length(s: &String) -> usize {
    s.len()
    // s goes out of scope but the borrowed value is NOT dropped
}

fn str_length(s: &str) -> usize {
    s.len()
}

fn mutable_references() {
    println!("\n=== Mutable References (&mut T) ===");

    let mut s = String::from("hello");

    // Only ONE mutable reference at a time:
    {
        let r = &mut s; // mutable borrow begins
        r.push_str(", world");
        println!("via mutable ref: {r}");
    } // mutable borrow ends here

    println!("s after mutation: {s}");

    // A function that mutates through a reference:
    append_exclamation(&mut s);
    println!("after append: {s}");

    // You CANNOT have an immutable and mutable reference simultaneously:
    let mut v = vec![1, 2, 3];
    let first = &v[0]; // immutable borrow
    println!("first = {first}"); // last use of `first`
    // Now it's safe to mutably borrow:
    v.push(4); // mutable borrow after the immutable one ended
    println!("v = {v:?}");
}

fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn borrow_rules_demo() {
    println!("\n=== Borrow Rules in Practice ===");

    let mut data = vec![1, 2, 3, 4, 5];

    // CASE 1: Multiple immutable borrows — always OK
    let a = &data[0];
    let b = &data[1];
    println!("a={a} b={b}"); // both used, borrows end here

    // CASE 2: One mutable borrow — OK when no immutable borrows active
    data.push(6);
    println!("pushed: {data:?}");

    // CASE 3: This would NOT compile:
    // let r = &data;
    // data.push(7);   // ← mutable borrow while immutable borrow `r` is live
    // println!("{r}");

    // Non-Lexical Lifetimes (NLL): borrows end when LAST USED, not at scope end.
    // This allows the following to compile:
    let r = &data;
    println!("r = {r:?}"); // last use of r
    data.push(7);           // OK: r's borrow already ended
    println!("data = {data:?}");
}

fn string_slices_as_borrows() {
    println!("\n=== String Slices as Borrows ===");

    let s = String::from("hello world");

    // &str is a borrowed string slice — a reference into s's data:
    let word = first_word(&s);
    println!("first word: '{word}'");
    // s still valid, word is a view into s

    // ⚠️ Cannot invalidate s while word is alive:
    // s.clear(); // ← would not compile while `word` borrows s
    println!("s = '{s}'"); // last use of s AND word

    // Returning a slice that refers to function parameter — fine:
    let hello = &s[0..5];
    println!("hello slice: '{hello}'");
}

fn first_word(s: &str) -> &str {
    // Returns a slice (borrowed view) — caller's ownership is unchanged.
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

fn borrowing_in_structs() {
    println!("\n=== Borrowing in Structs (preview of lifetimes) ===");

    // Structs can hold REFERENCES — but then they need lifetime annotations
    // (covered fully in module 04). For now, a simple example:

    struct Excerpt<'a> {
        text: &'a str, // borrows from something that lives >= 'a
    }

    impl<'a> Excerpt<'a> {
        fn announce(&self) {
            println!("Excerpt: '{}'", self.text);
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("could not find a '.'");

    let excerpt = Excerpt { text: first_sentence };
    excerpt.announce();
    // `excerpt` cannot outlive `novel` — the compiler proves this.
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. Borrowing rules are enforced at COMPILE TIME — no runtime cost.
// 2. You cannot mutate while an immutable reference is alive.
// 3. Only ONE mutable reference at a time — prevents data races.
// 4. NLL means borrows end at last use, not at closing brace.
// 5. &str is a borrowed string view — not a type like C#'s string.

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Creating a mutable reference while an immutable one is alive.
// • Iterating with `for x in vec` (moves elements) instead of
//   `for x in &vec` (borrows elements).
// • Returning a reference to a local variable — compile error.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Write a function that takes a &[i32] and returns the largest value.
// 2. Write a function that takes &mut Vec<i32> and removes all even numbers.
// 3. Demonstrate that you cannot modify a Vec while you hold a reference
//    to one of its elements (see the borrow checker error, then fix it).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word_finds_space() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn first_word_no_space() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn multiple_immutable_refs() {
        let s = String::from("test");
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1.len(), r2.len());
    }

    #[test]
    fn mutable_ref_modifies() {
        let mut s = String::from("hi");
        append_exclamation(&mut s);
        assert_eq!(s, "hi!");
    }
}
