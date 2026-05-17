// ============================================================
// CONCEPT: Generics
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust generics work similarly to C# generics, but are MONOMORPHISED:
// the compiler generates a separate copy of the function for each type.
// This means zero-cost abstractions — no boxing, no virtual dispatch.
//
// C#:   List<T>, IComparer<T>, IEnumerable<T>
// Rust: Vec<T>, Ord trait bound, Iterator<Item=T>
//
// Key differences:
//   • Rust generics: monomorphised → concrete code per type (like C++ templates)
//   • C# generics: erased at runtime → one JIT-compiled version
//   • Rust uses trait bounds (T: Trait) instead of C# generic constraints (where T: IInterface)
//   • Rust has associated types (type Item = T;) for more expressive trait definitions
//
// RUN: cargo run --bin generics
// ============================================================

use std::fmt::Display;
use std::ops::Add;

fn main() {
    generic_functions();
    generic_structs();
    generic_enums();
    multiple_bounds();
    where_clauses();
    associated_types();
    const_generics_advanced();
}

// ─── GENERIC FUNCTIONS ───────────────────────────────────────

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn print_all<T: Display>(items: &[T]) {
    for item in items {
        print!("{item} ");
    }
    println!();
}

fn generic_functions() {
    println!("=== Generic Functions ===");

    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    print_all(&[1, 2, 3, 4, 5]);
    print_all(&["hello", "world", "rust"]);
}

// ─── GENERIC STRUCTS ─────────────────────────────────────────

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("largest is first: {}", self.first);
        } else {
            println!("largest is second: {}", self.second);
        }
    }
}

// A generic stack — like Stack<T> in C#
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

fn generic_structs() {
    println!("\n=== Generic Structs ===");

    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let str_pair = Pair::new("hello", "world");
    str_pair.cmp_display();

    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack: {stack:?}");
    println!("peek: {:?}", stack.peek());
    println!("pop: {:?}", stack.pop());
    println!("stack after pop: {stack:?}");
}

// ─── GENERIC ENUMS ───────────────────────────────────────────

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L: Display, R: Display> Either<L, R> {
    fn value(&self) -> String {
        match self {
            Either::Left(v)  => format!("Left({v})"),
            Either::Right(v) => format!("Right({v})"),
        }
    }
}

fn generic_enums() {
    println!("\n=== Generic Enums ===");

    let left: Either<i32, &str> = Either::Left(42);
    let right: Either<i32, &str> = Either::Right("hello");

    println!("{}", left.value());
    println!("{}", right.value());
}

// ─── MULTIPLE BOUNDS ─────────────────────────────────────────

// T must implement both Display AND Add<Output=T>
fn double_and_print<T>(val: T) -> T
where
    T: Display + Add<Output = T> + Copy,
{
    let result = val + val;
    println!("{val} + {val} = {result}");
    result
}

fn multiple_bounds() {
    println!("\n=== Multiple Trait Bounds ===");

    double_and_print(5_i32);
    double_and_print(3.14_f64);

    // Bound syntax: T: Trait1 + Trait2 + Trait3
    // or with where clause for readability
}

fn where_clauses() {
    println!("\n=== Where Clauses ===");

    // Complex bounds are cleaner with `where`:
    fn compare_and_display<T, U>(t: &T, u: &U) -> bool
    where
        T: Display + PartialEq<U>,
        U: Display,
    {
        println!("Comparing '{t}' with '{u}'");
        t == u
    }

    // Simpler with trait alias pattern:
    trait Printable: Display + std::fmt::Debug {}
    impl<T: Display + std::fmt::Debug> Printable for T {}

    let equal = compare_and_display(&"hello", &"hello");
    println!("equal: {equal}");
}

// ─── ASSOCIATED TYPES ────────────────────────────────────────
// Associated types make trait definitions cleaner when the output type
// is uniquely determined by the implementing type.
// C# has no direct equivalent — it's like a associated generic that's fixed.

trait Converter {
    type Output;                       // associated type
    fn convert(&self) -> Self::Output;
}

struct Celsius(f64);
struct Fahrenheit(f64);

impl Converter for Celsius {
    type Output = Fahrenheit;
    fn convert(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}
impl Display for Fahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}°F", self.0)
    }
}

fn associated_types() {
    println!("\n=== Associated Types ===");

    let boiling = Celsius(100.0);
    let converted = boiling.convert();
    println!("{boiling} = {converted}");

    let freezing = Celsius(0.0);
    println!("{freezing} = {}", freezing.convert());

    // Why associated types instead of generic parameters?
    // With a generic: trait Converter<Output> { ... }  — a type could implement
    //   Converter<Fahrenheit> AND Converter<Kelvin> simultaneously.
    // With associated type: ONE implementation per type — simpler, clearer intent.
    // C# uses both patterns — generic interfaces and interface with typedefs.
}

fn const_generics_advanced() {
    println!("\n=== Advanced Const Generics ===");

    // Const generics let you parameterise over VALUES, not just types.
    // Perfect for fixed-size containers without heap allocation.

    #[derive(Debug)]
    struct FixedQueue<T, const N: usize> {
        data: [Option<T>; N],
        head: usize,
        len:  usize,
    }

    impl<T: Copy + Default, const N: usize> FixedQueue<T, N> {
        fn new() -> Self {
            Self { data: [None; N], head: 0, len: 0 }
        }

        fn push(&mut self, val: T) -> bool {
            if self.len == N { return false; } // full
            let idx = (self.head + self.len) % N;
            self.data[idx] = Some(val);
            self.len += 1;
            true
        }

        fn pop(&mut self) -> Option<T> {
            if self.len == 0 { return None; }
            let val = self.data[self.head].take();
            self.head = (self.head + 1) % N;
            self.len -= 1;
            val
        }

        fn capacity(&self) -> usize { N }
        fn len(&self) -> usize { self.len }
    }

    let mut q: FixedQueue<i32, 4> = FixedQueue::new();
    q.push(1); q.push(2); q.push(3); q.push(4);
    println!("full queue (cap={}): pushed 4 items, push fails: {}", q.capacity(), !q.push(5));
    println!("pop: {:?}", q.pop());
    println!("pop: {:?}", q.pop());
    q.push(5);
    println!("len: {}", q.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_numbers() {
        assert_eq!(*largest(&[3, 1, 4, 1, 5, 9, 2, 6]), 9);
    }

    #[test]
    fn stack_push_pop() {
        let mut s: Stack<i32> = Stack::new();
        s.push(1); s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn celsius_to_fahrenheit() {
        let c = Celsius(0.0);
        let Fahrenheit(f) = c.convert();
        assert!((f - 32.0).abs() < 0.001);
    }
}
