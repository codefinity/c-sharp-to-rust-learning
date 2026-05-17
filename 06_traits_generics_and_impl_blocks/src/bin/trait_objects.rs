// ============================================================
// CONCEPT: Trait Objects — dyn Trait (Dynamic Dispatch)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses polymorphism through virtual method tables (vtables) by default
// for classes. You can call methods on `IAnimal dog` without knowing the
// concrete type at compile time — this is always DYNAMIC DISPATCH in C#.
//
// Rust defaults to STATIC DISPATCH (monomorphisation — no vtable).
// For DYNAMIC DISPATCH, use `dyn Trait` — creates a fat pointer with a vtable.
//
// C#:   IAnimal animal = new Dog(); animal.Speak();  // always dynamic
// Rust: impl Trait → static  (preferred, zero-cost)
//       dyn Trait  → dynamic (needed for heterogeneous collections)
//
// Use `dyn Trait` when:
//   • You need a heterogeneous Vec<Box<dyn Trait>>
//   • The concrete type is not known at compile time
//   • You want to reduce binary size (one copy vs many monomorphised copies)
//
// RUN: cargo run --bin trait_objects
// ============================================================

use std::fmt;

fn main() {
    static_vs_dynamic();
    heterogeneous_collections();
    dyn_in_structs();
    object_safety();
    combining_dyn_and_generics();
}

// ─── TRAITS FOR EXAMPLES ─────────────────────────────────────

trait Shape: fmt::Debug {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Circle   { radius: f64 }
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }
#[derive(Debug)]
struct Triangle  { a: f64, b: f64, c: f64 }

impl Shape for Circle {
    fn area(&self)      -> f64   { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self)      -> &str  { "circle" }
    fn perimeter(&self) -> f64   { 2.0 * std::f64::consts::PI * self.radius }
}

impl Shape for Rectangle {
    fn area(&self)      -> f64   { self.width * self.height }
    fn name(&self)      -> &str  { "rectangle" }
    fn perimeter(&self) -> f64   { 2.0 * (self.width + self.height) }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn name(&self)      -> &str  { "triangle" }
    fn perimeter(&self) -> f64   { self.a + self.b + self.c }
}

// ─── STATIC VS DYNAMIC DISPATCH ──────────────────────────────

// STATIC DISPATCH: compiler generates a separate function for each T
fn print_shape_static<S: Shape>(shape: &S) {
    println!("[static] {} area={:.2}", shape.name(), shape.area());
}

// DYNAMIC DISPATCH: single function, vtable lookup at runtime
fn print_shape_dynamic(shape: &dyn Shape) {
    println!("[dynamic] {} area={:.2}", shape.name(), shape.area());
}

fn static_vs_dynamic() {
    println!("=== Static vs Dynamic Dispatch ===");

    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };

    // Static: zero-cost, but binary has two copies of the function
    print_shape_static(&c);
    print_shape_static(&r);

    // Dynamic: one copy of the function, vtable lookup per call
    print_shape_dynamic(&c);
    print_shape_dynamic(&r);

    println!(
        r#"
Size of concrete references:
  &Circle    = {} bytes (just a pointer)
  &Rectangle = {} bytes
  &dyn Shape = {} bytes (fat pointer: data ptr + vtable ptr)
"#,
        std::mem::size_of::<&Circle>(),
        std::mem::size_of::<&Rectangle>(),
        std::mem::size_of::<&dyn Shape>(),
    );
}

// ─── HETEROGENEOUS COLLECTIONS ───────────────────────────────

fn heterogeneous_collections() {
    println!("=== Heterogeneous Collections (Vec<Box<dyn Trait>>) ===");

    // C# List<IShape> — this is the Rust equivalent:
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 4.0, height: 5.0 }),
        Box::new(Triangle { a: 3.0, b: 4.0, c: 5.0 }),
    ];

    // Each iteration does a vtable lookup — like C# virtual method calls
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area: {total_area:.2}");

    for shape in &shapes {
        println!("  {}: area={:.2} perimeter={:.2}", shape.name(), shape.area(), shape.perimeter());
    }

    // Box<dyn Trait> owns the trait object (heap allocated).
    // &dyn Trait borrows it (stack or heap, borrowed).
    // Arc<dyn Trait> for shared ownership across threads.
}

// ─── DYN IN STRUCTS ──────────────────────────────────────────

fn dyn_in_structs() {
    println!("\n=== dyn Trait in Struct Fields ===");

    // A renderer that holds any drawable
    struct Canvas {
        shapes: Vec<Box<dyn Shape>>,
    }

    impl Canvas {
        fn new() -> Self { Canvas { shapes: Vec::new() } }

        fn add(&mut self, shape: Box<dyn Shape>) {
            self.shapes.push(shape);
        }

        fn render(&self) {
            println!("Canvas with {} shapes:", self.shapes.len());
            for s in &self.shapes {
                println!("  {:?}", s);
            }
        }

        fn total_area(&self) -> f64 {
            self.shapes.iter().map(|s| s.area()).sum()
        }
    }

    let mut canvas = Canvas::new();
    canvas.add(Box::new(Circle { radius: 1.0 }));
    canvas.add(Box::new(Rectangle { width: 2.0, height: 3.0 }));
    canvas.render();
    println!("total area: {:.2}", canvas.total_area());
}

fn object_safety() {
    println!("\n=== Object Safety ===");
    println!(
        r#"
A trait is "object safe" (can be used as dyn Trait) when:
  1. It has no generic methods (fn foo<T>(&self))
  2. It doesn't use `Self` in argument or return positions
     (except &self / &mut self receivers)
  3. It doesn't require Sized (fn foo(self: Box<Self>))

OBJECT SAFE:
  trait Drawable {{ fn draw(&self); }}           ✓
  trait Named   {{ fn name(&self) -> &str; }}    ✓

NOT OBJECT SAFE:
  trait Clone   {{ fn clone(&self) -> Self; }}   ✗ — returns Self
  trait Sized   {{ fn size() -> usize; }}        ✗ — generic method
  trait Builder {{ fn build<T>(&self) -> T; }}   ✗ — generic method

Work-arounds for non-object-safe traits:
  • Clone: use Arc<T> or require explicit clone
  • Associated types: specify them in the bound (dyn Trait<Output=Foo>)
"#
    );
}

fn combining_dyn_and_generics() {
    println!("=== Combining dyn Trait with Generics ===");

    // A generic logger that can work with any Output type:
    trait Logger {
        fn log(&self, msg: &str);
    }

    struct ConsoleLogger;
    struct SilentLogger;

    impl Logger for ConsoleLogger {
        fn log(&self, msg: &str) { println!("[LOG] {msg}"); }
    }
    impl Logger for SilentLogger {
        fn log(&self, _msg: &str) {}
    }

    // Static dispatch version — zero-cost, logger type known at compile time:
    fn process_static<L: Logger>(logger: &L, data: &[i32]) {
        logger.log("processing data");
        let sum: i32 = data.iter().sum();
        logger.log(&format!("sum = {sum}"));
    }

    // Dynamic dispatch version — logger type resolved at runtime:
    fn process_dynamic(logger: &dyn Logger, data: &[i32]) {
        logger.log("processing data (dynamic)");
        let sum: i32 = data.iter().sum();
        logger.log(&format!("sum = {sum}"));
    }

    let console = ConsoleLogger;
    let silent  = SilentLogger;
    let data = [1, 2, 3, 4, 5];

    process_static(&console, &data);
    process_static(&silent, &data); // no output

    let loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(ConsoleLogger),
        Box::new(SilentLogger),
        Box::new(ConsoleLogger),
    ];
    for logger in &loggers {
        process_dynamic(logger.as_ref(), &data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_collection_total_area() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle { width: 2.0, height: 3.0 }),
        ];
        let total: f64 = shapes.iter().map(|s| s.area()).sum();
        let expected = std::f64::consts::PI + 6.0;
        assert!((total - expected).abs() < 1e-10);
    }

    #[test]
    fn fat_pointer_size() {
        assert_eq!(std::mem::size_of::<&dyn Shape>(), 16); // 2 × pointer on 64-bit
    }
}
