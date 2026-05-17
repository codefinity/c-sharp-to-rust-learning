// ============================================================
// CONCEPT: Move Semantics
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C#, assignment copies a reference for reference types:
//   var a = new List<int> { 1, 2 };
//   var b = a; // b and a both point to the SAME list
//   b.Add(3);  // modifies the shared list — a sees it too
//
// In Rust, assigning a non-Copy value MOVES ownership:
//   let a = vec![1, 2];
//   let b = a;  // ownership MOVES to b — a is no longer valid
//   // println!("{:?}", a); // compile error!
//
// Move semantics are the default for any type that does NOT implement Copy.
// Copy types (primitives, small value types) are implicitly copied.
//
// C# HAS move semantics in one place: `ref struct` + Span<T> types,
// but Rust applies this universally to all heap-owning types.
//
// RUN: cargo run --bin moves
// ============================================================

fn main() {
    basic_moves();
    move_in_assignments();
    move_in_match();
    partial_moves();
    move_closures();
}

fn basic_moves() {
    println!("=== Basic Moves ===");

    let v1 = vec![1, 2, 3];   // v1 owns the Vec
    let v2 = v1;               // ownership MOVES from v1 to v2
    // println!("{v1:?}");     // ← compile error: "value moved here"
    println!("v2 = {v2:?}");

    // The same move happens when passing to a function:
    let s = String::from("hello");
    let len = calculate_length(s); // s moved into function
    // println!("{s}");            // ← compile error
    println!("length was {len}");
}

fn calculate_length(s: String) -> usize {
    s.len()
} // s dropped here

fn move_in_assignments() {
    println!("\n=== Moves in Complex Assignments ===");

    // Move in struct initialization:
    let name = String::from("Alice");
    let user = User { name, age: 30 };
    // println!("{name}"); // ← name was moved into `user`
    println!("{} is {}", user.name, user.age);

    // Move out of struct — the struct is partially moved:
    let user2 = User { name: String::from("Bob"), age: 25 };
    let _name = user2.name; // moves `name` out
    // println!("{}", user2.name); // ← compile error: partially moved
    println!("Bob's age: {}", user2.age); // age is Copy, still accessible

    // Struct update syntax — remaining fields are moved/copied from source:
    let user3 = User {
        name: String::from("Carol"),
        // age is Copy, so it's copied from user2
        ..user2
    };
    println!("{} is {}", user3.name, user3.age);
    // user2.age still accessible (was copied), but user2.name was already moved
}

#[derive(Debug)]
struct User {
    name: String,
    age:  u32,
}

fn move_in_match() {
    println!("\n=== Moves in match ===");

    // Matching on a non-Copy value MOVES it into the pattern variable:
    let s = Some(String::from("hello"));

    match s {
        Some(val) => println!("got: {val}"), // val owns the String
        None      => println!("nothing"),
    }
    // s is now moved — cannot use it
    // println!("{s:?}"); // ← compile error

    // To avoid moving, match on a reference:
    let s = Some(String::from("world"));
    match &s {
        Some(val) => println!("borrowed: {val}"), // val: &String
        None      => println!("nothing"),
    }
    println!("s still valid: {s:?}"); // s was only borrowed
}

fn partial_moves() {
    println!("\n=== Partial Moves ===");

    // You can move individual fields out of a struct.
    // After a partial move, the struct is "partially moved" — you can
    // only access the fields that were NOT moved.

    #[derive(Debug)]
    struct Pair {
        first:  String,
        second: String,
    }

    let pair = Pair {
        first:  String::from("alpha"),
        second: String::from("beta"),
    };

    let first = pair.first; // moves `first` out of pair
    println!("first = {first}");
    println!("second = {}", pair.second); // second still accessible
    // println!("{pair:?}"); // ← compile error: pair is partially moved

    // Solution: destructure the whole struct at once:
    let pair2 = Pair {
        first:  String::from("gamma"),
        second: String::from("delta"),
    };
    let Pair { first, second } = pair2; // destructure — both moved at once
    println!("first={first} second={second}");
}

fn move_closures() {
    println!("\n=== Move Closures ===");

    // The `move` keyword forces a closure to take OWNERSHIP of captured
    // variables, rather than borrowing them.
    // This is necessary when the closure outlives the current scope
    // (e.g., spawning a thread).

    let greeting = String::from("Hello from closure");

    // Without move: closure borrows greeting
    let borrow_closure = || println!("{greeting}");
    borrow_closure();
    println!("greeting still here: {greeting}");

    // With move: closure OWNS greeting
    let move_closure = move || {
        println!("{greeting}"); // greeting is OWNED by the closure now
    };
    // println!("{greeting}"); // ← compile error if uncommented: greeting moved
    move_closure();

    // Typical use: spawning threads that need owned data
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        // `data` is moved into this thread — safe because the closure owns it
        println!("thread data: {data:?}");
    });
    handle.join().unwrap();
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. Assignment of non-Copy types MOVES (not copies reference).
// 2. After a move, the source is UNUSABLE — compile-time check.
// 3. Use .clone() for an explicit deep copy.
// 4. match arms move values unless you match on a reference.
// 5. `move` closures take ownership of captured variables.

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Expecting `let b = a` to share a reference (like C# classes).
//   In Rust it transfers ownership.
// • Iterating over a collection by value — moves every element:
//   `for x in vec { ... }` moves each element. Use `for x in &vec`.
// • Partial moves confusing the struct's usability.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Create a Vec<String> and move it into a function. Try to use it
//    after the call — observe the error, then fix it using a reference.
// 2. Write a `move` closure that captures a Vec and spawns a thread.
// 3. Demonstrate partial moves: take one field from a struct and
//    show that you can still access other fields.

#[cfg(test)]
mod tests {
    #[test]
    fn move_and_use_in_closure() {
        let s = String::from("owned");
        let f = move || s.len();
        assert_eq!(f(), 5);
    }

    #[test]
    fn clone_allows_reuse() {
        let v = vec![1, 2, 3];
        let _v2 = v.clone();
        assert_eq!(v.len(), 3); // v still usable
    }
}
