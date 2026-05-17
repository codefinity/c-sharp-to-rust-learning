// ============================================================
// MIGRATION GUIDE: C# Classes vs Rust Structs + Traits
// ============================================================
//
// C# has: classes (reference types), structs (value types),
//   interfaces, abstract classes, inheritance.
//
// Rust has: structs/enums (always value types), traits (like
//   interfaces but more powerful), NO inheritance of data —
//   only behaviour via traits.
//
// This is the most fundamental mindset shift for C# developers.
//
// RUN: cargo run --bin classes_vs_structs
// ============================================================

fn main() {
    println!("=== C# Classes vs Rust Structs ===\n");

    class_to_struct();
    interface_to_trait();
    inheritance_to_composition();
    polymorphism_comparison();
    access_modifiers();
}

// ---- 1. Class → Struct + impl -----------------------------------

// C#:
//   public class Rectangle {
//       public double Width { get; set; }
//       public double Height { get; set; }
//       public Rectangle(double w, double h) { Width = w; Height = h; }
//       public double Area() => Width * Height;
//       public double Perimeter() => 2 * (Width + Height);
//       public override string ToString() => $"Rect({Width}×{Height})";
//   }

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Constructor → associated function (no `new` keyword)
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    // Methods take &self (immutable) or &mut self (mutable)
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
    fn scale(&mut self, factor: f64) { self.width *= factor; self.height *= factor; }
}

// Display = ToString() in C#
impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rect({}×{})", self.width, self.height)
    }
}

fn class_to_struct() {
    println!("--- Class → Struct + impl ---");

    let mut r = Rectangle::new(4.0, 3.0);
    println!("rect: {r}");
    println!("area: {}", r.area());
    println!("perimeter: {}", r.perimeter());

    r.scale(2.0);
    println!("scaled: {r}");

    // Structs are VALUE types (like C# structs, not classes):
    let r2 = r.clone(); // explicit copy
    println!("clone: {r2}");
}

// ---- 2. Interface → Trait ----------------------------------------

// C#:
//   public interface IShape {
//       double Area();
//       double Perimeter();
//       string Describe() => $"Area={Area():.2f}"; // default method
//   }

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default method — C# default interface method:
    fn describe(&self) -> String {
        format!("area={:.2}", self.area())
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.area() }
    fn perimeter(&self) -> f64 { self.perimeter() }
}

#[derive(Debug)]
struct Circle { radius: f64 }
impl Circle {
    fn new(radius: f64) -> Self { Circle { radius } }
}
impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

fn interface_to_trait() {
    println!("\n--- Interface → Trait ---");

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(4.0, 3.0)),
        Box::new(Circle::new(5.0)),
    ];

    for s in &shapes {
        println!("  {}", s.describe());
    }

    // C# analogy: List<IShape> shapes = new() { new Rectangle(4,3), new Circle(5) };
}

// ---- 3. Inheritance → Composition --------------------------------

// C# OOP inheritance:
//   class Animal { string Name; virtual void Speak(); }
//   class Dog : Animal { override void Speak() => "Woof"; }
//   class Cat : Animal { override void Speak() => "Meow"; }
//
// Rust: no struct inheritance — use traits + composition instead.

trait Speaks {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

// The "base class" behaviour lives in a default trait method or
// in a helper struct that is CONTAINED (not inherited):
struct AnimalBase { name: String }
impl AnimalBase {
    fn greet(&self, speaker: &dyn Speaks) {
        println!("  {} says: {}", self.name, speaker.speak());
    }
}

struct Dog { base: AnimalBase }
struct Cat { base: AnimalBase }

impl Dog {
    fn new(name: &str) -> Self { Dog { base: AnimalBase { name: name.to_string() } } }
}
impl Cat {
    fn new(name: &str) -> Self { Cat { base: AnimalBase { name: name.to_string() } } }
}

impl Speaks for Dog {
    fn speak(&self) -> &str { "Woof!" }
    fn name(&self)  -> &str { &self.base.name }
}
impl Speaks for Cat {
    fn speak(&self) -> &str { "Meow!" }
    fn name(&self)  -> &str { &self.base.name }
}

fn inheritance_to_composition() {
    println!("\n--- Inheritance → Composition ---");

    let animals: Vec<Box<dyn Speaks>> = vec![
        Box::new(Dog::new("Rex")),
        Box::new(Cat::new("Whiskers")),
    ];

    for a in &animals {
        println!("  {}: {}", a.name(), a.speak());
    }

    println!(r#"
Key mindset shift:
  C#: Dog extends Animal (IS-A relationship, shared data)
  Rust: Dog contains AnimalBase (HAS-A) + implements Speaks (CAN-DO)
  Rust trait = C# interface (behaviour contract only, no shared data)
"#);
}

// ---- 4. Polymorphism comparison ----------------------------------

fn polymorphism_comparison() {
    println!("--- Static vs Dynamic Dispatch ---");

    // Static dispatch (monomorphisation) — like C# generics:
    // Generates separate code per concrete type, zero runtime overhead.
    fn print_area_static(s: &impl Shape) {
        println!("  [static] area = {:.2}", s.area());
    }

    // Dynamic dispatch (vtable) — like C# virtual/interface calls:
    // One code path, looks up method in vtable at runtime.
    fn print_area_dynamic(s: &dyn Shape) {
        println!("  [dynamic] area = {:.2}", s.area());
    }

    let r = Rectangle::new(3.0, 4.0);
    let c = Circle::new(5.0);

    print_area_static(&r);   // compiled as print_area_for_Rectangle
    print_area_static(&c);   // compiled as print_area_for_Circle
    print_area_dynamic(&r);  // single fn, vtable lookup
    print_area_dynamic(&c);  // single fn, vtable lookup

    println!(r#"
C# analogy:
  void PrintArea<T>(T s) where T : IShape → impl Shape (static)
  void PrintArea(IShape s)                → &dyn Shape (dynamic)
"#);
}

// ---- 5. Access modifiers ----------------------------------------

fn access_modifiers() {
    println!("--- Access Modifiers ---");

    println!(r#"
C#              | Rust            | Notes
----------------|-----------------|----------------------------------
public          | pub             | Visible everywhere
private         | (default)       | Private to current module
protected       | pub(super)      | Visible to parent module
internal        | pub(crate)      | Visible within the current crate
protected internal | pub(super) or custom | No direct equivalent
private protected  | (no equivalent) | Use module boundaries
static          | (no keyword)    | Associated fn — fn new() not &self
abstract        | (trait w/ no default) | Required method in trait
sealed          | (default — no inheritance) | All Rust types are "sealed"
virtual         | (trait default method) | Default method in trait
override        | (trait impl)    | Implementing a trait method
readonly field  | immutable by default | let x: T; or &self fields
const field     | const NAME: T   | Compile-time constant in impl
"#);
}
