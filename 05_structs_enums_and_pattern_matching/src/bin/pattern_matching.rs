// ============================================================
// CONCEPT: Pattern Matching — match, if let, while let, let else
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# 7+ has pattern matching via `switch` expressions and `is` patterns,
// but Rust's pattern matching is more powerful and EXHAUSTIVE — the compiler
// forces you to handle every case.
//
// C# switch expression (C# 8+):
//   var result = shape switch {
//       Circle c => Math.PI * c.Radius * c.Radius,
//       Rectangle r => r.Width * r.Height,
//       _ => throw new Exception("unknown"),
//   };
//
// Rust match is the same idea but:
//   • Exhaustiveness is a compile-time check (no runtime exception)
//   • Works with ANY type (not just classes)
//   • Supports tuple patterns, range patterns, binding (@), guards
//   • Is an EXPRESSION returning a value
//
// RUN: cargo run --bin pattern_matching
// ============================================================

fn main() {
    match_basics();
    tuple_patterns();
    destructuring_patterns();
    guards_and_bindings();
    nested_patterns();
    matches_macro();
    advanced_patterns();
}

fn match_basics() {
    println!("=== match Basics ===");

    let x = 5_i32;

    // Every possible value must be covered (exhaustive):
    let description = match x {
        1          => "one",
        2 | 3      => "two or three",           // OR
        4..=6      => "four through six",        // range
        n if n < 0 => "negative",               // guard
        _          => "something else",          // wildcard
    };
    println!("{x} is {description}");

    // match is an expression — assign the result:
    let parity = match x % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("{x} is {parity}");

    // match on enum (exhaustive check catches missing variants at compile time):
    #[derive(Debug)]
    enum Coin { Penny, Nickel, Dime, Quarter }

    fn value_in_cents(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny   => 1,
            Coin::Nickel  => 5,
            Coin::Dime    => 10,
            Coin::Quarter => 25,
        }
    }

    for coin in &[Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!("{coin:?} = {} cents", value_in_cents(coin));
    }
}

fn tuple_patterns() {
    println!("\n=== Tuple Patterns ===");

    let point = (1_i32, -1_i32);
    let quadrant = match point {
        (x, y) if x > 0 && y > 0 => "Q1 (+,+)",
        (x, y) if x < 0 && y > 0 => "Q2 (-,+)",
        (x, y) if x < 0 && y < 0 => "Q3 (-,-)",
        (x, y) if x > 0 && y < 0 => "Q4 (+,-)",
        (0, _) | (_, 0)           => "on axis",
        _                         => "origin",
    };
    println!("({},{}) is in {quadrant}", point.0, point.1);

    // Matching pairs of Options:
    let a: Option<i32> = Some(5);
    let b: Option<i32> = Some(3);
    match (a, b) {
        (Some(x), Some(y)) => println!("both: {} + {} = {}", x, y, x + y),
        (Some(x), None)    => println!("only a: {x}"),
        (None, Some(y))    => println!("only b: {y}"),
        (None, None)       => println!("neither"),
    }
}

fn destructuring_patterns() {
    println!("\n=== Destructuring Patterns ===");

    #[derive(Debug)]
    struct Point3D { x: f64, y: f64, z: f64 }

    let p = Point3D { x: 1.0, y: 2.0, z: 3.0 };

    // Destructure in match arm:
    let Point3D { x, y, z } = p;
    println!("x={x} y={y} z={z}");

    // Destructure struct in match with rename:
    let p2 = Point3D { x: 0.0, y: 5.0, z: 0.0 };
    match p2 {
        Point3D { x: 0.0, z: 0.0, y } => println!("on Y axis at y={y}"),
        Point3D { x, y, z }            => println!("at ({x},{y},{z})"),
    }

    // Destructure enum:
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("hello".into()),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in &messages {
        match msg {
            Message::Quit                => println!("Quit"),
            Message::Move { x, y }      => println!("Move to ({x},{y})"),
            Message::Write(text)         => println!("Write: {text}"),
            Message::ChangeColor(r,g,b)  => println!("Color: #{r:02X}{g:02X}{b:02X}"),
        }
    }
}

fn guards_and_bindings() {
    println!("\n=== Guards (@) and Bindings ===");

    // Match guards — additional conditions
    let num = 7_i32;
    match num {
        n @ 1..=9 if n % 2 == 0 => println!("{n} is a single-digit even"),
        n @ 1..=9               => println!("{n} is a single-digit odd"),
        n                       => println!("{n} is multi-digit"),
    }

    // @ binding — capture matched value AND apply pattern:
    let x = 5_u32;
    match x {
        n @ 1..=12 => println!("month: {n}"),
        n          => println!("not a month: {n}"),
    }

    // @ with enum variants:
    let msg = Some(42_i32);
    match msg {
        Some(n @ 1..=50) => println!("Some with small positive: {n}"),
        Some(n)          => println!("Some with other: {n}"),
        None             => println!("None"),
    }
}

fn nested_patterns() {
    println!("\n=== Nested Patterns ===");

    #[derive(Debug)]
    enum Inner { A, B(i32) }

    #[derive(Debug)]
    enum Outer { X(Inner), Y }

    let val = Outer::X(Inner::B(42));

    match val {
        Outer::X(Inner::A)    => println!("X(A)"),
        Outer::X(Inner::B(n)) => println!("X(B({n}))"),
        Outer::Y              => println!("Y"),
    }

    // Slice patterns:
    let v = vec![1, 2, 3, 4, 5];
    match v.as_slice() {
        []            => println!("empty"),
        [x]           => println!("one: {x}"),
        [x, y]        => println!("two: {x}, {y}"),
        [first, .., last] => println!("many: first={first} last={last}"),
    }

    // Nested Option patterns:
    let nested: Option<Option<i32>> = Some(Some(7));
    match nested {
        Some(Some(n)) if n > 5 => println!("nested Some(Some) > 5: {n}"),
        Some(Some(n))          => println!("nested Some(Some): {n}"),
        Some(None)             => println!("Some(None)"),
        None                   => println!("None"),
    }
}

fn matches_macro() {
    println!("\n=== matches! macro ===");

    // `matches!` is a boolean check against a pattern — useful in conditions
    // without needing a full match expression.
    // C# equivalent: `x is SomePattern`

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let has_even = v.iter().any(|&x| matches!(x, n if n % 2 == 0));
    println!("has even: {has_even}");

    let opt: Option<i32> = Some(42);
    println!("is Some: {}", matches!(opt, Some(_)));
    println!("is Some(42): {}", matches!(opt, Some(42)));
    println!("is Some(1..=50): {}", matches!(opt, Some(1..=50)));
}

fn advanced_patterns() {
    println!("\n=== Advanced: Ref Patterns ===");

    // When matching on a reference, use `ref` to avoid moving:
    let strings = vec!["hello", "world"];
    for s in &strings {
        // s is &&str — pattern match against the reference:
        match s {
            s if s.starts_with('h') => println!("starts with h: {s}"),
            _                       => println!("other: {s}"),
        }
    }

    // `ref` in destructuring — takes a reference to the field rather than moving it:
    #[derive(Debug)]
    struct Config { name: String, value: i32 }
    let cfg = Config { name: String::from("timeout"), value: 30 };
    let Config { name: ref n, value: v } = cfg;
    println!("name ref: {n}  value: {v}");
    println!("cfg still valid: {cfg:?}"); // cfg not moved because we used `ref`
}

#[cfg(test)]
mod tests {
    #[test]
    fn match_is_expression() {
        let x = 7;
        let s = match x {
            1..=6  => "small",
            7..=12 => "medium",
            _      => "large",
        };
        assert_eq!(s, "medium");
    }

    #[test]
    fn matches_macro_works() {
        let x: Option<i32> = Some(5);
        assert!(matches!(x, Some(1..=10)));
        assert!(!matches!(x, None));
    }

    #[test]
    fn tuple_pattern() {
        let p = (3_i32, 4_i32);
        let result = match p {
            (x, y) if x == y => "equal",
            (x, y) if x < y  => "less",
            _                => "greater",
        };
        assert_eq!(result, "less");
    }
}
