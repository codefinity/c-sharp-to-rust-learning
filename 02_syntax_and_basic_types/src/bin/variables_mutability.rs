// ============================================================
// CONCEPT: Variables and Mutability
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C# everything is mutable by default; you use `readonly` or `const` to
// freeze things. Rust is the opposite: variables are IMMUTABLE by default.
// You must explicitly opt in to mutability with `mut`. This catches an entire
// class of bugs at compile time rather than at runtime.
//
// C# analogy:
//   int x = 5;            →  let x: i32 = 5;       (immutable)
//   int x = 5; x = 6;     →  let mut x: i32 = 5; x = 6;
//   const int X = 5;      →  const X: i32 = 5;     (true compile-time const)
//   readonly int x = 5;   →  let x = 5;             (runtime-immutable binding)
//
// RUN: cargo run --bin variables_mutability
// ============================================================

fn main() {
    immutability_demo();
    mutability_demo();
    shadowing_demo();
    type_inference_demo();
    destructuring_demo();
}

fn immutability_demo() {
    println!("=== Immutability ===");

    // let bindings are immutable by default.
    // Attempting `x = 6` here would be a COMPILE ERROR.
    let x = 5;
    println!("x = {x}");

    // This is NOT the same as C# `readonly` — there is no heap indirection.
    // For primitive types (i32, f64, bool, char) immutability is trivial.
    // For owned heap types (String, Vec<T>) it means you cannot call any
    // &mut self methods on them.

    let name = String::from("Alice");
    // name.push_str(" Smith"); // ← would not compile: `name` is not `mut`
    println!("name = {name}");
}

fn mutability_demo() {
    println!("\n=== Mutability ===");

    // `mut` makes the binding mutable.
    let mut count = 0_i32;
    count += 1;
    count += 1;
    println!("count = {count}"); // 2

    let mut message = String::from("Hello");
    message.push_str(", world!"); // mutation through &mut self method
    println!("{message}");

    // Mutable reference to a value already bound:
    let mut nums = vec![3, 1, 2];
    nums.sort();         // sort() takes &mut self
    println!("{nums:?}");
}

fn shadowing_demo() {
    println!("\n=== Shadowing ===");

    // Shadowing re-uses the same name for a NEW binding — unlike C# where
    // redeclaring a variable in the same scope is a compile error.
    // Shadowing lets you change the TYPE of a variable.
    let spaces = "   ";         // type: &str
    let spaces = spaces.len();  // type: usize — shadowed to a different type!
    println!("spaces (len) = {spaces}");

    // Useful for parsing pipelines:
    let input = "42";
    let input: i32 = input.parse().expect("not a number");
    println!("parsed input = {input}");

    // Shadowing inside a block:
    let y = 5;
    let y = {
        let y = y + 1; // inner shadow
        y * 2          // block evaluates to y*2 = 12
    };
    println!("y after block shadow = {y}"); // 12

    // DIFFERENCE FROM `mut`: shadowing creates a NEW binding.
    // The old binding is gone. `mut` mutates in place.
}

fn type_inference_demo() {
    println!("\n=== Type Inference ===");

    // Rust can infer types from context — no need to write `: i32` etc.
    let a = 42;          // i32 (default integer)
    let b = 3.14;        // f64 (default float)
    let c = true;        // bool
    let d = 'R';         // char (Unicode scalar value — 4 bytes, not UTF-16)
    println!("a={a} b={b} c={c} d={d}");

    // You CAN annotate explicitly — required when inference is ambiguous:
    let big: i64 = 10_000_000_000;       // underscores are visual separators
    let precise: f32 = 1.0;
    println!("big={big} precise={precise}");

    // Inference works across statements:
    let mut vec = Vec::new(); // type unknown here
    vec.push(1_u32);          // now inferred as Vec<u32>
    println!("{vec:?}");
}

fn destructuring_demo() {
    println!("\n=== Destructuring ===");

    // Rust lets you destructure tuples and structs in let bindings.
    let point = (3_i32, 7_i32);
    let (x, y) = point;
    println!("x={x} y={y}");

    // Ignore elements with `_`
    let (first, _, third) = (1, 2, 3);
    println!("first={first} third={third}");

    // Destructure a struct inline
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }
    let red = Color { r: 255, g: 0, b: 0 };
    let Color { r, g, b } = red;
    println!("r={r} g={g} b={b}");
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. Immutable by default — you must write `mut` to mutate.
// 2. Shadowing changes the type; mutation changes the value in place.
// 3. No `var` keyword — `let` infers but also allows annotation.
// 4. No `null` — Option<T> is used instead (covered in module 05).
// 5. Destructuring is first-class in `let` bindings.

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Forgetting `mut` and wondering why the compiler says "cannot assign".
// • Confusing shadowing with mutation — shadowing is a new binding.
// • Using `_name` (prefixed underscore) vs `_` — `_name` binds and warns
//   if unused; `_` does NOT bind at all.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Declare an immutable integer, then shadow it with its square.
// 2. Create a mutable String, append three words to it, and print it.
// 3. Use destructuring to swap two variables without a temp variable.
// 4. Demonstrate that you cannot mutate an immutable binding (try it
//    and read the compiler error, then fix it).

#[cfg(test)]
mod tests {
    #[test]
    fn shadowing_changes_type() {
        let x = "hello";
        let x = x.len();
        assert_eq!(x, 5_usize);
    }

    #[test]
    fn mut_modifies_in_place() {
        let mut n = 0_i32;
        n += 10;
        assert_eq!(n, 10);
    }

    #[test]
    fn destructuring_tuple() {
        let (a, b) = (1_i32, 2_i32);
        assert_eq!(a + b, 3);
    }
}
