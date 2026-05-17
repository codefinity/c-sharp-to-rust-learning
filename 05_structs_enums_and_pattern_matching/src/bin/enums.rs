// ============================================================
// CONCEPT: Enums — Algebraic Data Types
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# enums are just named integers — they carry no data.
// Rust enums are ALGEBRAIC DATA TYPES (ADTs): each variant can carry
// different data. This is the most powerful feature difference between C# and Rust.
//
// C# discriminated unions (DU) require workarounds:
//   - Sealed class hierarchies
//   - OneOf<T1,T2> libraries
//   - C# 9+ record classes with abstract base
//
// Rust enums ARE discriminated unions — first-class, zero-overhead.
//
// C# analogy: Think of Rust enums as a supercharged version of:
//   sealed abstract record Shape;
//   record Circle(double Radius) : Shape;
//   record Rectangle(double W, double H) : Shape;
//
// RUN: cargo run --bin enums
// ============================================================

fn main() {
    basic_enums();
    enums_with_data();
    option_enum();
    result_enum();
    methods_on_enums();
    complex_enum_example();
}

// ─── BASIC ENUMS (like C# enums) ─────────────────────────────

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn basic_enums() {
    println!("=== Basic Enums ===");

    let dir = Direction::North;
    match dir {
        Direction::North => println!("Heading north"),
        Direction::South => println!("Heading south"),
        Direction::East  => println!("Heading east"),
        Direction::West  => println!("Heading west"),
    }

    // Enums can have explicit discriminant values:
    #[repr(u8)]
    #[derive(Debug)]
    enum Color {
        Red   = 1,
        Green = 2,
        Blue  = 4,
    }
    println!("Red = {}", Color::Red as u8);

    // Compare enum values:
    println!("dir is North: {}", dir == Direction::North);
}

// ─── ENUMS WITH DATA (ADTs) ───────────────────────────────────

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64), // base, height, hypotenuse
    Dot,                      // no data — unit variant
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius }              => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height }    => width * height,
            Shape::Triangle(base, height, _)      => 0.5 * base * height,
            Shape::Dot                            => 0.0,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle    { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle(..)     => "triangle",
            Shape::Dot              => "dot",
        }
    }
}

fn enums_with_data() {
    println!("\n=== Enums With Data (ADTs) ===");

    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
        Shape::Dot,
    ];

    for s in &shapes {
        println!("{}: area = {:.2}", s.name(), s.area());
    }
}

// ─── OPTION<T> ────────────────────────────────────────────────
// Option<T> replaces nullable reference types in C#.
// C# `string?` → Rust `Option<String>`
// There is NO null in safe Rust.

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn option_enum() {
    println!("\n=== Option<T> (replaces null) ===");

    // Option is defined in std as:
    //   enum Option<T> { Some(T), None }
    // It is in scope by default — no need to write std::option::Option

    let present: Option<i32> = Some(42);
    let absent:  Option<i32> = None;

    // Pattern match — exhaustive
    match present {
        Some(v) => println!("got {v}"),
        None    => println!("nothing"),
    }

    // if let — when you only care about Some:
    if let Some(v) = absent {
        println!("value: {v}");
    } else {
        println!("absent was None");
    }

    // Useful Option methods (mirrors C# Nullable<T> and LINQ FirstOrDefault):
    println!("unwrap_or: {}", absent.unwrap_or(0));
    println!("unwrap_or_else: {}", absent.unwrap_or_else(|| 99));
    println!("map: {:?}", present.map(|v| v * 2));
    println!("and_then: {:?}", present.and_then(|v| if v > 40 { Some(v) } else { None }));
    println!("is_some: {}  is_none: {}", present.is_some(), absent.is_none());

    // Real usage:
    let nums = [1, 3, 5, 8, 7];
    match find_first_even(&nums) {
        Some(n) => println!("first even: {n}"),
        None    => println!("no even numbers"),
    }

    // Chaining with ? operator (see error_handling module):
    let doubled = present.map(|x| x * 2);
    println!("doubled: {doubled:?}");
}

// ─── RESULT<T, E> ─────────────────────────────────────────────
// Result<T, E> replaces exceptions for recoverable errors.
// C# `try { return Parse(s); } catch { return default; }`
// Rust: s.parse::<i32>()  →  Result<i32, ParseIntError>

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn result_enum() {
    println!("\n=== Result<T, E> (replaces exceptions for recoverable errors) ===");

    // Defined in std as:
    //   enum Result<T, E> { Ok(T), Err(E) }

    // Pattern match:
    match divide(10.0, 2.0) {
        Ok(v)  => println!("10 / 2 = {v}"),
        Err(e) => println!("error: {e}"),
    }

    match divide(10.0, 0.0) {
        Ok(v)  => println!("result: {v}"),
        Err(e) => println!("error: {e}"),
    }

    // Useful Result methods:
    let r: Result<i32, &str> = Ok(42);
    println!("map: {:?}", r.map(|v| v * 2));
    println!("unwrap_or: {}", r.unwrap_or(0));

    // Parse — returns Result<T, ParseError>:
    let parsed: Result<i32, _> = "42".parse();
    println!("parsed: {parsed:?}");

    let bad: Result<i32, _> = "oops".parse::<i32>();
    println!("bad parse: {bad:?}");

    // Collect a Vec of Results into a single Result<Vec<T>>:
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("all parsed: {numbers:?}");
}

fn methods_on_enums() {
    println!("\n=== Methods on Enums ===");

    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLight {
        fn duration_seconds(&self) -> u32 {
            match self {
                TrafficLight::Red    => 60,
                TrafficLight::Yellow => 5,
                TrafficLight::Green  => 45,
            }
        }

        fn next(&self) -> TrafficLight {
            match self {
                TrafficLight::Red    => TrafficLight::Green,
                TrafficLight::Yellow => TrafficLight::Red,
                TrafficLight::Green  => TrafficLight::Yellow,
            }
        }

        fn is_safe_to_go(&self) -> bool {
            matches!(self, TrafficLight::Green)
        }
    }

    let mut light = TrafficLight::Red;
    for _ in 0..4 {
        println!("{:?}: {} sec, safe={}", light, light.duration_seconds(), light.is_safe_to_go());
        light = light.next();
    }
}

fn complex_enum_example() {
    println!("\n=== Complex Enum: JSON-like Value ===");

    #[derive(Debug)]
    enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        Text(String),
        Array(Vec<JsonValue>),
        Object(std::collections::HashMap<String, JsonValue>),
    }

    impl JsonValue {
        fn type_name(&self) -> &str {
            match self {
                JsonValue::Null      => "null",
                JsonValue::Bool(_)   => "bool",
                JsonValue::Number(_) => "number",
                JsonValue::Text(_)   => "string",
                JsonValue::Array(_)  => "array",
                JsonValue::Object(_) => "object",
            }
        }

        fn is_truthy(&self) -> bool {
            match self {
                JsonValue::Null         => false,
                JsonValue::Bool(b)      => *b,
                JsonValue::Number(n)    => *n != 0.0,
                JsonValue::Text(s)      => !s.is_empty(),
                JsonValue::Array(a)     => !a.is_empty(),
                JsonValue::Object(o)    => !o.is_empty(),
            }
        }
    }

    let values: Vec<JsonValue> = vec![
        JsonValue::Null,
        JsonValue::Bool(true),
        JsonValue::Number(42.0),
        JsonValue::Text("hello".into()),
        JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
    ];

    for v in &values {
        println!("type={} truthy={} value={:?}", v.type_name(), v.is_truthy(), v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area() {
        let c = Shape::Circle { radius: 1.0 };
        let expected = std::f64::consts::PI;
        assert!((c.area() - expected).abs() < 1e-10);
    }

    #[test]
    fn option_map() {
        let x: Option<i32> = Some(5);
        assert_eq!(x.map(|v| v * 2), Some(10));
        let y: Option<i32> = None;
        assert_eq!(y.map(|v| v * 2), None);
    }

    #[test]
    fn result_ok_err() {
        assert!(divide(10.0, 2.0).is_ok());
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn find_even() {
        assert_eq!(find_first_even(&[1, 3, 5]), None);
        assert_eq!(find_first_even(&[1, 4, 5]), Some(4));
    }
}
