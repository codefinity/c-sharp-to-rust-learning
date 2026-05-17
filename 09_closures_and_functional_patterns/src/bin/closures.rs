// ============================================================
// CONCEPT: Closures
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust closures ≈ C# lambda expressions and delegates.
// Key differences:
//   • Three closure traits: Fn, FnMut, FnOnce (unlike one delegate type in C#)
//   • Closures capture by the MINIMUM required: &T, &mut T, or T (move)
//   • `move` keyword forces ownership capture
//   • Async closures are stabilised in Rust 1.85 (Edition 2024)
//
// C#:  Func<T,R>  →  Rust: Fn(T) -> R
//      Action<T>  →  Rust: Fn(T)  (no return)
//      Predicate<T> →  Rust: Fn(&T) -> bool
//
// RUN: cargo run --bin closures
// ============================================================

fn main() {
    closure_syntax();
    closure_capturing();
    fn_traits();
    closures_as_parameters();
    closures_as_return_values();
    async_closures();
}

fn closure_syntax() {
    println!("=== Closure Syntax ===");

    // Basic closure: |params| body
    let add = |a: i32, b: i32| -> i32 { a + b };
    println!("add(3,4) = {}", add(3, 4));

    // Type inference — types can usually be omitted:
    let multiply = |a, b| a * b; // inferred from first call
    println!("multiply(3,4) = {}", multiply(3_i32, 4_i32));

    // Single-expression closure (no braces needed):
    let square = |x: i32| x * x;
    println!("square(5) = {}", square(5));

    // Multi-statement closure:
    let process = |s: &str| {
        let trimmed = s.trim();
        let upper = trimmed.to_uppercase();
        format!("Processed: {upper}")
    };
    println!("{}", process("  hello rust  "));

    // Closure that ignores its argument:
    let always_42 = |_: i32| 42;
    println!("always_42(99) = {}", always_42(99));

    // C# equivalent:
    // Func<int,int,int> add = (a, b) => a + b;
    // Func<int,int> square = x => x * x;
    // Action<string> print = s => Console.WriteLine(s);
}

fn closure_capturing() {
    println!("\n=== Closure Capturing ===");

    let message = String::from("Hello");
    let number  = 42_i32;

    // Immutable borrow — closure borrows `message` and `number`
    let print_msg = || println!("{message} {number}"); // borrows message and number
    print_msg();
    print_msg(); // can call multiple times — just borrows
    println!("message still valid: {message}");

    // Mutable borrow — closure mutates `counter`
    let mut count = 0;
    let mut increment = || { count += 1; count };
    println!("increment: {}", increment()); // 1
    println!("increment: {}", increment()); // 2
    // println!("{count}"); // ← cannot borrow count as immutable because `increment` mutably borrows it
    drop(increment); // drop closure to release mutable borrow
    println!("count after closure dropped: {count}");

    // Move capture — closure OWNS the value
    let data = vec![1, 2, 3];
    let owns_data = move || {
        // `data` is MOVED into this closure
        println!("data: {data:?}");
    };
    // println!("{data:?}"); // ← compile error: data moved
    owns_data();
    owns_data(); // Fn — can call multiple times

    // When is `move` required?
    // When the closure outlives the current scope (e.g., spawning a thread):
    let s = String::from("thread-safe data");
    let handle = std::thread::spawn(move || {
        println!("in thread: {s}");
    });
    handle.join().unwrap();
}

fn fn_traits() {
    println!("\n=== Fn, FnMut, FnOnce ===");

    // FnOnce — can only be called ONCE (consumes captured values)
    let text = String::from("consumed");
    let consume = move || {
        // text is moved OUT of the closure when called
        let _taken = text; // this moves text, making the closure FnOnce
    };
    consume(); // text moved out
    // consume(); // ← compile error: cannot call FnOnce more than once

    // FnMut — can be called multiple times, but mutates captured state
    let mut total = 0;
    let mut add_to_total = |x: i32| { total += x; };
    add_to_total(5);
    add_to_total(3);
    drop(add_to_total);
    println!("total: {total}"); // 8

    // Fn — can be called multiple times, only borrows immutably
    let base = 10;
    let add_base = |x: i32| x + base;
    println!("add_base(5) = {}", add_base(5));
    println!("add_base(3) = {}", add_base(3));
    println!("base still: {base}");

    // Hierarchy: Fn ⊂ FnMut ⊂ FnOnce
    // Every Fn is also FnMut and FnOnce.
    // Every FnMut is also FnOnce.
    // A function parameter accepting FnOnce accepts all three.

    println!(
        r#"
Closure type  | Captures by | Called how many times
--------------+-------------+----------------------
FnOnce        | value (move)| once (may consume)
FnMut         | &mut        | many (mutates state)
Fn            | &           | many (read-only)
"#
    );
}

fn closures_as_parameters() {
    println!("=== Closures as Parameters ===");

    // accept Fn
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
    println!("apply square: {}", apply(|x| x * x, 5));

    // accept FnMut — function needs to call the closure multiple times
    fn apply_n_times<F: FnMut(i32) -> i32>(mut f: F, mut x: i32, n: u32) -> i32 {
        for _ in 0..n {
            x = f(x);
        }
        x
    }
    let result = apply_n_times(|x| x * 2, 1, 5);
    println!("double 5 times: {result}"); // 32

    // accept FnOnce — for callbacks called at most once
    fn run_once<F: FnOnce() -> String>(f: F) -> String { f() }
    let name = String::from("Rust");
    let greeting = run_once(move || format!("Hello, {name}!"));
    println!("{greeting}");

    // Higher-order functions with closures:
    fn transform<T, U, F: Fn(T) -> U>(items: Vec<T>, f: F) -> Vec<U> {
        items.into_iter().map(f).collect()
    }
    let nums = vec![1, 2, 3, 4, 5];
    let strings = transform(nums, |n| format!("item_{n}"));
    println!("transform: {strings:?}");
}

fn closures_as_return_values() {
    println!("\n=== Closures as Return Values ===");

    // Return a closure using `impl Fn`:
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add5  = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // Return different closure types using Box<dyn Fn>:
    fn make_op(add: bool) -> Box<dyn Fn(i32) -> i32> {
        if add {
            Box::new(|x| x + 1)
        } else {
            Box::new(|x| x - 1)
        }
    }
    let plus_one  = make_op(true);
    let minus_one = make_op(false);
    println!("plus_one(5) = {}", plus_one(5));
    println!("minus_one(5) = {}", minus_one(5));

    // Composing closures:
    fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
        move |x| g(f(x))
    }
    let double_then_add1 = compose(|x: i32| x * 2, |x| x + 1);
    println!("double_then_add1(5) = {}", double_then_add1(5)); // 11
}

fn async_closures() {
    println!("\n=== Async Closures (Rust 1.85+ / Edition 2024) ===");

    // async closures were stabilised in Rust 1.85.0 (edition 2024)
    // They return Futures and can be awaited.

    println!("Async closure syntax (requires tokio runtime for .await):");
    println!("  let fetch = async |url: &str| -> Result<String, reqwest::Error> {{");
    println!("      reqwest::get(url).await?.text().await");
    println!("  }};");
    println!("  let body = fetch(\"https://example.com\").await?;");
    println!();
    println!("  // Async closures implement AsyncFn, AsyncFnMut, AsyncFnOnce");
    println!("  // analogous to Fn, FnMut, FnOnce");

    // We can demonstrate with a sync closure for now (async demo is in module 14):
    let double_async_style = |x: i32| x * 2; // same signature, sync version
    println!("sync equivalent result: {}", double_async_style(21));
}

#[cfg(test)]
mod tests {
    #[test]
    fn closure_capture_immutable() {
        let base = 10;
        let add = |x: i32| x + base;
        assert_eq!(add(5), 15);
        assert_eq!(base, 10); // base unchanged
    }

    #[test]
    fn closure_capture_mutable() {
        let mut count = 0;
        let mut inc = || { count += 1; };
        inc(); inc(); inc();
        drop(inc);
        assert_eq!(count, 3);
    }

    #[test]
    fn fn_composition() {
        fn compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C {
            move |x| g(f(x))
        }
        let f = compose(|x: i32| x * 2, |x| x.to_string());
        assert_eq!(f(5), "10");
    }
}
