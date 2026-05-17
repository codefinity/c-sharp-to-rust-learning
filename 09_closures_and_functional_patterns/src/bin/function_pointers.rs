// ============================================================
// CONCEPT: Function Pointers and Higher-Order Functions
// ============================================================
// RUN: cargo run --bin function_pointers
// ============================================================

fn main() {
    function_pointers();
    fn_in_structs();
    dispatch_tables();
    strategy_pattern();
}

fn add(a: i32, b: i32) -> i32 { a + b }
fn sub(a: i32, b: i32) -> i32 { a - b }
fn mul(a: i32, b: i32) -> i32 { a * b }

fn function_pointers() {
    println!("=== Function Pointers (fn type) ===");

    // fn pointer — points to a named function (not a closure with captures)
    // C# analogy: delegate or Func<T,R> pointing to a static method
    let op: fn(i32, i32) -> i32 = add;
    println!("add(3,4) via fn ptr = {}", op(3, 4));

    let ops: [fn(i32, i32) -> i32; 3] = [add, sub, mul];
    for (i, f) in ops.iter().enumerate() {
        println!("  ops[{i}](10, 3) = {}", f(10, 3));
    }

    // fn pointers coerce to Fn trait — use where Fn is expected:
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
    fn double(x: i32) -> i32 { x * 2 }

    println!("apply double: {}", apply(double, 5));  // fn pointer as Fn trait object
    println!("apply closure: {}", apply(|x| x + 1, 5)); // closure works too

    // fn pointers can be used in const contexts:
    const TRIPLE: fn(i32) -> i32 = |x| x * 3;
    println!("TRIPLE(7) = {}", TRIPLE(7));
}

fn fn_in_structs() {
    println!("\n=== Function Pointers in Structs ===");

    // Like C# delegates stored in fields
    struct Calculator {
        operation: fn(f64, f64) -> f64,
        label:     &'static str,
    }

    impl Calculator {
        fn compute(&self, a: f64, b: f64) -> f64 {
            (self.operation)(a, b)
        }
    }

    let calcs = [
        Calculator { operation: |a, b| a + b, label: "add" },
        Calculator { operation: |a, b| a * b, label: "multiply" },
        Calculator { operation: f64::max,      label: "max" },
    ];

    for calc in &calcs {
        println!("  {} (10.0, 3.0) = {}", calc.label, calc.compute(10.0, 3.0));
    }
}

fn dispatch_tables() {
    println!("\n=== Dispatch Tables (like C# Dictionary<string, Action>) ===");

    use std::collections::HashMap;

    let commands: HashMap<&str, fn()> = [
        ("hello",   (|| println!("  Hello, World!")) as fn()),
        ("version", (|| println!("  v1.95.0")) as fn()),
        ("quit",    (|| println!("  Goodbye!")) as fn()),
    ].into_iter().collect();

    for cmd in &["hello", "version", "unknown", "quit"] {
        match commands.get(cmd) {
            Some(f) => f(),
            None    => println!("  unknown command: {cmd}"),
        }
    }
}

fn strategy_pattern() {
    println!("\n=== Strategy Pattern with Closures ===");

    // C# Strategy pattern typically uses interfaces.
    // In Rust, use generic types with trait bounds or Box<dyn Fn>:

    struct Sorter<T> {
        compare: Box<dyn Fn(&T, &T) -> std::cmp::Ordering>,
    }

    impl<T> Sorter<T> {
        fn new(compare: impl Fn(&T, &T) -> std::cmp::Ordering + 'static) -> Self {
            Self { compare: Box::new(compare) }
        }

        fn sort(&self, items: &mut Vec<T>) {
            items.sort_by(|a, b| (self.compare)(a, b));
        }
    }

    let mut names = vec!["Charlie", "Alice", "Bob", "Dave"];

    let alpha_sorter = Sorter::new(|a: &&str, b: &&str| a.cmp(b));
    alpha_sorter.sort(&mut names);
    println!("alphabetical: {names:?}");

    let len_sorter = Sorter::new(|a: &&str, b: &&str| a.len().cmp(&b.len()));
    len_sorter.sort(&mut names);
    println!("by length: {names:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_pointer_call() {
        let f: fn(i32, i32) -> i32 = |a, b| a + b;
        assert_eq!(f(3, 4), 7);
    }

    #[test]
    fn fn_array() {
        let ops: [fn(i32) -> i32; 3] = [|x| x + 1, |x| x * 2, |x| x - 1];
        assert_eq!(ops[1](5), 10);
    }
}
