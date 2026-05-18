// ============================================================
// OOP PILLAR 3: Polymorphism
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Polymorphism lets you write code that works with values of different
// types through a common interface. C# does this with:
//   - Interfaces           (IShape shape = new Circle())
//   - Virtual/override     (runtime dispatch through vtable)
//   - Generics             (List<T>, where T : IShape)
//   - Method overloading   (same name, different signatures)
//
// Rust has TWO distinct kinds of polymorphism:
//   1. Static dispatch  — resolved at compile time (generics / impl Trait)
//                         Fast: no vtable. Monomorphised to one copy per type.
//   2. Dynamic dispatch — resolved at runtime  (dyn Trait)
//                         Flexible: one code path, pointer-to-vtable overhead.
//
// C# defaults to dynamic dispatch (virtual/interface).
// Rust defaults to static dispatch (generics). Use dyn Trait when you need
// to store mixed types at runtime.
//
// RUN: cargo run --bin polymorphism
// ============================================================

fn main() {
    println!("=== OOP Pillar 3: Polymorphism ===\n");

    demo_static_dispatch();
    demo_dynamic_dispatch();
    demo_abstract_class_to_trait();
    demo_mixed_collections();
    demo_overloading_alternatives();
    demo_return_impl_trait();
}

// ─── SHARED SETUP ────────────────────────────────────────────────────────────

trait Shape {
    fn area(&self)      -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self)      -> &str;
    fn describe(&self) -> String {
        format!("{}: area={:.2}, perimeter={:.2}", self.name(), self.area(), self.perimeter())
    }
}

#[derive(Debug, Clone)]
struct Circle    { radius: f64 }
#[derive(Debug, Clone)]
struct Rectangle { width: f64, height: f64 }
#[derive(Debug, Clone)]
struct Triangle  { a: f64, b: f64, c: f64 }

impl Shape for Circle {
    fn area(&self)      -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn name(&self)      -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self)      -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn name(&self)      -> &str { "Rectangle" }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 { self.a + self.b + self.c }
    fn name(&self)      -> &str { "Triangle" }
}

// ─── 1. STATIC DISPATCH (impl Trait / generics) ──────────────────────────────
//
// The compiler generates a SEPARATE copy of the function for each concrete type.
// No runtime overhead — the call is direct, not through a vtable.
//
// C# generics with constraints are the closest analogy:
//   static double TotalArea<T>(T shape) where T : IShape => shape.Area();
//
// Rust:
//   fn print_info(shape: &impl Shape)     ← sugar for the generic form below
//   fn print_info<S: Shape>(shape: &S)    ← explicit generic

// Sugar syntax — `impl Shape` in argument position means "some concrete type that impls Shape":
fn print_info(shape: &impl Shape) {
    println!("  {}", shape.describe());
}

// Equivalent explicit generic form — useful when you need to name the type parameter:
fn largest_area<S: Shape>(a: &S, b: &S) -> f64 {
    f64::max(a.area(), b.area())
}

// Multiple trait bounds — like C# `where T : IShape, IComparable<T>`:
use std::fmt::Debug;
fn print_debug_and_area<S: Shape + Debug>(shape: &S) {
    println!("  debug={shape:?}  area={:.2}", shape.area());
}

fn demo_static_dispatch() {
    println!("--- 1. Static Dispatch (generics / impl Trait) ---");
    println!();
    println!("  Equivalent to C# generics with constraints — zero runtime overhead.");
    println!();

    let c = Circle    { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };
    let t = Triangle  { a: 3.0, b: 4.0, c: 5.0 };

    print_info(&c);
    print_info(&r);
    print_info(&t);

    let c2 = Circle { radius: 5.0 };
    println!("  largest(circle r=3, circle r=5) area = {:.2}", largest_area(&c, &c2));

    print_debug_and_area(&c);
    print_debug_and_area(&r);

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  void Print<T>(T s) where T : IShape   fn print_info<S: Shape>(s: &S)
  void Print(IShape s)  // virtual      fn print_info(s: &impl Shape) // monomorphised
  where T : IShape, IDebug              S: Shape + Debug
"#);
}

// ─── 2. DYNAMIC DISPATCH (dyn Trait) ─────────────────────────────────────────
//
// A `dyn Trait` is a "fat pointer": a pointer to the data + a pointer to the vtable.
// The exact method to call is resolved at RUNTIME — just like C# interface dispatch.
//
// C#:
//   IShape shape = new Circle(3);      // heap-allocated reference, vtable dispatch
//   shape.Area();
//
// Rust:
//   let shape: &dyn Shape = &Circle { radius: 3.0 };   // stack data, fat pointer
//   let shape: Box<dyn Shape> = Box::new(Circle { radius: 3.0 }); // heap allocation

fn print_dyn(shape: &dyn Shape) {
    // shape is a fat pointer — vtable lookup at runtime
    println!("  {}", shape.describe());
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn demo_dynamic_dispatch() {
    println!("--- 2. Dynamic Dispatch (dyn Trait) ---");
    println!();
    println!("  Equivalent to C# interface references — vtable at runtime.");
    println!();

    // &dyn Shape — borrowed reference, no heap allocation for the shape itself:
    let c = Circle { radius: 3.0 };
    let r = Rectangle { width: 4.0, height: 5.0 };

    let shapes_ref: Vec<&dyn Shape> = vec![&c, &r];
    for s in &shapes_ref {
        print_dyn(*s);
    }

    // Box<dyn Shape> — heap-allocated, owned, mixed types in one Vec:
    let shapes_owned: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle    { radius: 5.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
        Box::new(Triangle  { a: 3.0, b: 4.0, c: 5.0 }),
    ];
    println!("  Total area of all shapes: {:.2}", total_area(&shapes_owned));

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  IShape shape = new Circle(3);         let s: &dyn Shape = &Circle {{ radius: 3.0 }};
  List<IShape> shapes = new List<>();   let shapes: Vec<Box<dyn Shape>> = vec![...];
  shape.Area()  // vtable lookup        s.area()  // vtable lookup (same cost)

  When to use dyn Trait:
    ✓ Storing mixed types in a collection
    ✓ Returning different types from a function at runtime
    ✓ Plugin / callback patterns where type is unknown at compile time
  When to prefer impl Trait / generics:
    ✓ Single known type per call site (compiler can optimise)
    ✓ No heap allocation needed
    ✓ Performance-critical code
"#);
}

// ─── 3. ABSTRACT CLASS → TRAIT WITH DEFAULT METHODS ─────────────────────────
//
// C#:
//   abstract class Logger {
//       public abstract void WriteRaw(string msg);         // must implement
//       public virtual  void Log(string msg)               // may override
//           => WriteRaw($"[LOG] {msg}");
//       public void LogError(string msg)                   // sealed behaviour
//           => WriteRaw($"[ERROR] {msg}");
//   }
//   class FileLogger : Logger {
//       public override void WriteRaw(string msg) { /* write to file */ }
//   }
//
// Rust: trait with required + default methods is a direct translation.

trait Logger {
    // Required — must implement:
    fn write_raw(&self, msg: &str);

    // Default — may override (like `virtual`):
    fn log(&self, msg: &str) {
        self.write_raw(&format!("[LOG] {msg}"));
    }

    // Default — typically not overridden (like non-virtual / sealed):
    fn log_error(&self, msg: &str) {
        self.write_raw(&format!("[ERROR] {msg}"));
    }

    fn log_warn(&self, msg: &str) {
        self.write_raw(&format!("[WARN] {msg}"));
    }
}

struct ConsoleLogger;
struct PrefixLogger { prefix: String }

impl Logger for ConsoleLogger {
    fn write_raw(&self, msg: &str) { println!("  CONSOLE | {msg}"); }
}

impl Logger for PrefixLogger {
    fn write_raw(&self, msg: &str) {
        println!("  [{}] {msg}", self.prefix);
    }
    // Override log() to add timestamp-like prefix:
    fn log(&self, msg: &str) {
        self.write_raw(&format!("[LOG][{}] {msg}", self.prefix));
    }
}

fn demo_abstract_class_to_trait() {
    println!("--- 3. Abstract Class → Trait with Default Methods ---");
    println!();

    let console = ConsoleLogger;
    let prefix  = PrefixLogger { prefix: "APP".to_string() };

    console.log("Server started");
    console.log_error("Disk full");
    prefix.log("User logged in");       // uses overridden log()
    prefix.log_warn("Memory low");      // uses default log_warn()

    println!(r#"
  C# abstract class                    Rust trait
  ──────────────────────────────────────────────────────────────────
  abstract void WriteRaw(string msg)   fn write_raw(&self, msg: &str);
  virtual  void Log(string msg) {{..}} fn log(&self, msg: &str) {{ ... }}
  /* sealed */ void LogError(..) {{..}}fn log_error(&self, msg: &str) {{ ... }}
  class FileLogger : Logger {{ ... }}  impl Logger for FileLogger {{ ... }}
  override void WriteRaw(string m)     fn write_raw(&self, m: &str) {{ ... }}
"#);
}

// ─── 4. MIXED COLLECTIONS ────────────────────────────────────────────────────
//
// C#: List<IAnimal> animals = new() { new Dog(), new Cat(), new Parrot() };
// Rust: Vec<Box<dyn Animal>> — or an enum if the set of types is closed.

trait Speak { fn speak(&self) -> String; }

#[derive(Debug)] struct Parrot { phrase: String }
impl Speak for Parrot  { fn speak(&self) -> String { format!("Polly says: {}", self.phrase) } }

#[derive(Debug)] struct Cow;
impl Speak for Cow     { fn speak(&self) -> String { "Moo!".to_string() } }

#[derive(Debug)] struct Penguin;
impl Speak for Penguin { fn speak(&self) -> String { "Noot noot!".to_string() } }

// Alternative: enum dispatch — no heap allocation, closed set of types:
#[derive(Debug)]
enum Animal { Parrot(Parrot), Cow(Cow), Penguin(Penguin) }

impl Speak for Animal {
    fn speak(&self) -> String {
        match self {
            Animal::Parrot(p)  => p.speak(),
            Animal::Cow(c)     => c.speak(),
            Animal::Penguin(p) => p.speak(),
        }
    }
}

fn demo_mixed_collections() {
    println!("--- 4. Mixed Collections ---");
    println!();

    // dyn Trait: open set — add new types without changing existing code:
    let zoo_dyn: Vec<Box<dyn Speak>> = vec![
        Box::new(Parrot  { phrase: "Pretty bird!".to_string() }),
        Box::new(Cow),
        Box::new(Penguin),
    ];
    println!("  dyn Trait collection:");
    for animal in &zoo_dyn { println!("    {}", animal.speak()); }

    // Enum dispatch: closed set — exhaustive match, no heap allocation:
    let zoo_enum = vec![
        Animal::Parrot(Parrot { phrase: "Hello!".to_string() }),
        Animal::Cow(Cow),
        Animal::Penguin(Penguin),
    ];
    println!("  Enum dispatch collection:");
    for animal in &zoo_enum { println!("    {}", animal.speak()); }

    println!(r#"
  Strategy          Heap?  Open set?  Match coverage  C# equivalent
  ─────────────────────────────────────────────────────────────────────
  Vec<Box<dyn T>>   Yes    Yes        N/A              List<IAnimal>
  Vec<EnumType>     No     No         exhaustive       (no direct equiv)

  Prefer enum when you know all the variants — compiler will tell you
  if you add a variant and forget to handle it somewhere.
"#);
}

// ─── 5. METHOD OVERLOADING ALTERNATIVES ──────────────────────────────────────
//
// Rust does NOT support method overloading (same name, different signatures).
// C#: void Print(int x), void Print(string x), void Print(int x, string label)
//
// Rust alternatives:
//   a) Different function names   (most common)
//   b) Generic functions          (when logic is the same, types differ)
//   c) Into<T> / From<T>          (accept multiple input types)
//   d) Enum argument              (when variants have meaning)
//   e) Default values via builder (when parameters are optional)

// a) Different names:
fn print_int(x: i32)    { println!("  int:    {x}"); }
fn print_str(x: &str)   { println!("  string: {x}"); }

// b) Generic — works for any Display type:
fn print_val<T: std::fmt::Display>(x: T) { println!("  value:  {x}"); }

// c) Into<T> — accept &str, String, and anything that converts to String:
fn greet(name: impl Into<String>) {
    let name = name.into();
    println!("  Hello, {name}!");
}

// d) Enum argument — when variants carry meaning:
#[derive(Debug)]
enum LogLevel { Info, Warn, Error }

fn log_message(level: LogLevel, msg: &str) {
    println!("  [{level:?}] {msg}");
}

fn demo_overloading_alternatives() {
    println!("--- 5. Method Overloading Alternatives ---");
    println!();

    // a) Different names:
    print_int(42);
    print_str("hello");

    // b) Generic:
    print_val(3.14_f64);
    print_val("world");

    // c) Into<String> — accepts &str literal AND owned String:
    greet("Alice");
    greet(String::from("Bob"));

    // d) Enum argument:
    log_message(LogLevel::Info,  "Server ready");
    log_message(LogLevel::Error, "Disk full");

    println!(r#"
  C# overloading                        Rust alternative
  ────────────────────────────────────────────────────────────────────
  void Print(int x)                     fn print_int(x: i32)
  void Print(string x)                  fn print_str(x: &str)
  void Print<T>(T x) where T : ...      fn print_val<T: Display>(x: T)
  void Greet(string name)               fn greet(name: impl Into<String>)
  void Log(LogLevel lvl, string msg)    fn log_message(level: LogLevel, msg: &str)
"#);
}

// ─── 6. RETURN POSITION impl Trait ───────────────────────────────────────────
//
// C# 8+: IShape MakeShape(bool circle) => circle ? new Circle() : new Rect();
//         // fine — both implement IShape, returned as interface reference
//
// Rust `impl Trait` in return position means "one specific concrete type,
// determined by the function body." You CANNOT return two different concrete
// types with `impl Trait` — use `Box<dyn Trait>` for that.

fn make_circle(radius: f64) -> impl Shape {
    Circle { radius }         // caller gets a Shape, but compiler knows it's a Circle
}

// When the concrete type varies at runtime, use Box<dyn Trait>:
fn make_shape(circle: bool) -> Box<dyn Shape> {
    if circle {
        Box::new(Circle    { radius: 3.0 })
    } else {
        Box::new(Rectangle { width: 2.0, height: 5.0 })
    }
}

fn demo_return_impl_trait() {
    println!("--- 6. Return-position impl Trait vs Box<dyn Trait> ---");
    println!();

    let s1 = make_circle(4.0);
    println!("  impl Shape return: {}", s1.describe());

    let s2 = make_shape(true);
    let s3 = make_shape(false);
    println!("  Box<dyn Shape> (circle):    {}", s2.describe());
    println!("  Box<dyn Shape> (rectangle): {}", s3.describe());

    println!(r#"
  C#                                    Rust
  ────────────────────────────────────────────────────────────────────
  IShape MakeShape() => new Circle()    fn make() -> impl Shape {{ Circle {{ .. }} }}
  IShape Choose(bool b) => b ? ...      fn choose(b: bool) -> Box<dyn Shape> {{ ... }}

  impl Trait return = one concrete type, zero heap allocation.
  Box<dyn Trait>   = runtime-chosen type, one heap allocation.
"#);
}

// ─── TESTS ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area_static() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn dynamic_dispatch_total_area() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle { width: 2.0, height: 3.0 }),
            Box::new(Rectangle { width: 1.0, height: 1.0 }),
        ];
        assert!((total_area(&shapes) - 7.0).abs() < 1e-6);
    }

    #[test]
    fn enum_dispatch_cow_speaks() {
        let a = Animal::Cow(Cow);
        assert_eq!(a.speak(), "Moo!");
    }

    #[test]
    fn make_shape_returns_correct_type() {
        let c = make_shape(true);
        let r = make_shape(false);
        assert!(c.area() > 0.0);
        assert!((r.area() - 10.0).abs() < 1e-6); // 2 * 5
    }

    #[test]
    fn trait_default_method_used_in_logger() {
        // ConsoleLogger only implements write_raw; log_error uses the default.
        // We just verify it doesn't panic.
        let logger = ConsoleLogger;
        logger.log_error("test error");
    }
}
