// ============================================================
// CONCEPT: Traits — Rust's Interface System
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust traits are similar to C# interfaces, but with key differences:
//   • Traits can provide DEFAULT method implementations (like C# default interface methods)
//   • You can implement traits for types you don't own (blanket impls)
//   • No inheritance hierarchy — composition via trait bounds
//   • Traits replace: interfaces, abstract classes, base classes, extension methods
//   • No virtual dispatch by default — static dispatch (monomorphisation)
//
// C# interface:
//   interface IAnimal { void Speak(); string Name { get; } }
//   class Dog : IAnimal { public void Speak() => Console.WriteLine("Woof!"); }
//
// Rust trait:
//   trait Animal { fn speak(&self); fn name(&self) -> &str; }
//   impl Animal for Dog { fn speak(&self) { println!("Woof!"); } }
//
// RUN: cargo run --bin traits
// ============================================================

use std::fmt;

fn main() {
    basic_traits();
    default_methods();
    multiple_traits();
    trait_as_parameter();
    blanket_impls();
    standard_traits();
}

// ─── DEFINING AND IMPLEMENTING TRAITS ────────────────────────

trait Animal {
    // Required methods — must be implemented by every type
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    // Default method — can be overridden, but has a default implementation
    fn speak(&self) {
        println!("{} says: {}", self.name(), self.sound());
    }

    // Default method that calls required methods — always works
    fn description(&self) -> String {
        format!("I am {} and I say {}", self.name(), self.sound())
    }
}

struct Dog { name: String }
struct Cat { name: String }
struct Duck;

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Woof" }
    // speak() uses the default implementation
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn sound(&self) -> &str { "Meow" }
    fn speak(&self) {
        // Override the default
        println!("🐱 {} whispers: {}", self.name(), self.sound());
    }
}

impl Animal for Duck {
    fn name(&self) -> &str { "Donald" }
    fn sound(&self) -> &str { "Quack" }
}

fn basic_traits() {
    println!("=== Basic Traits ===");

    let dog = Dog { name: "Rex".into() };
    let cat = Cat { name: "Whiskers".into() };
    let duck = Duck;

    dog.speak();
    cat.speak();
    duck.speak();
    println!("{}", dog.description());
}

// ─── DEFAULT METHOD OVERRIDING ───────────────────────────────

trait Greet {
    fn greeting(&self) -> String;

    // Default: uses greeting()
    fn greet(&self) {
        println!("{}", self.greeting());
    }

    // Default: enriched version
    fn greet_loudly(&self) {
        println!("*** {} ***", self.greeting().to_uppercase());
    }
}

struct EnglishGreeter;
struct SpanishGreeter;

impl Greet for EnglishGreeter {
    fn greeting(&self) -> String { "Hello, World!".into() }
}

impl Greet for SpanishGreeter {
    fn greeting(&self) -> String { "¡Hola, Mundo!".into() }
    fn greet(&self) {
        println!("🇪🇸 {}", self.greeting());
    }
}

fn default_methods() {
    println!("\n=== Default Methods ===");

    let en = EnglishGreeter;
    let es = SpanishGreeter;

    en.greet();
    en.greet_loudly();
    es.greet();
    es.greet_loudly();
}

// ─── MULTIPLE TRAIT IMPLEMENTATION ───────────────────────────

trait Drawable {
    fn draw(&self);
    fn bounding_box(&self) -> (f64, f64, f64, f64); // x, y, w, h
}

trait Clickable {
    fn on_click(&self) -> String;
}

#[derive(Debug)]
struct Button {
    label: String,
    x: f64, y: f64,
    width: f64, height: f64,
}

impl Drawable for Button {
    fn draw(&self) {
        println!("Drawing button '{}' at ({},{}) {}x{}", self.label, self.x, self.y, self.width, self.height);
    }
    fn bounding_box(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Clickable for Button {
    fn on_click(&self) -> String {
        format!("Button '{}' clicked!", self.label)
    }
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Button({})", self.label)
    }
}

fn multiple_traits() {
    println!("\n=== Multiple Trait Implementations ===");

    let btn = Button { label: "OK".into(), x: 10.0, y: 20.0, width: 80.0, height: 30.0 };
    btn.draw();
    println!("{}", btn.on_click());
    println!("{}", btn); // Display
    println!("{:?}", btn); // Debug... wait, we didn't derive Debug
}

// ─── TRAITS AS PARAMETERS ────────────────────────────────────

// Syntax 1: impl Trait in parameter position — sugar for generic
fn make_sound(animal: &impl Animal) {
    animal.speak();
}

// Syntax 2: Trait bound with where clause — more readable for complex bounds
fn describe_animal<T>(animal: &T)
where
    T: Animal + fmt::Debug,
{
    println!("{} — {:?}", animal.description(), animal);
}

// Syntax 3: return impl Trait — return a type that implements the trait
fn new_animal(is_dog: bool) -> impl Animal {
    if is_dog {
        Dog { name: "Buddy".into() }
        // Note: both arms must return the SAME concrete type with impl Trait
        // Use Box<dyn Animal> for different types (see trait_objects.rs)
    } else {
        Dog { name: "Fido".into() } // same type — works
    }
}

impl fmt::Debug for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

fn trait_as_parameter() {
    println!("\n=== Traits as Parameters ===");

    let dog = Dog { name: "Buddy".into() };
    let cat = Cat { name: "Luna".into() };

    make_sound(&dog);
    make_sound(&cat);
    describe_animal(&dog);

    let animal = new_animal(true);
    animal.speak();
}

// ─── BLANKET IMPLEMENTATIONS ─────────────────────────────────
// Implement a trait for ALL types that implement another trait.

trait Summary {
    fn summarize(&self) -> String;
}

// Blanket impl: implement Summary for anything that implements Display
impl<T: fmt::Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Summary: {self}")
    }
}

fn blanket_impls() {
    println!("\n=== Blanket Implementations ===");

    // i32 implements Display, so it now implements Summary too:
    println!("{}", 42_i32.summarize());
    println!("{}", "hello".summarize());
    println!("{}", 3.14_f64.summarize());
}

fn standard_traits() {
    println!("\n=== Important Standard Library Traits ===");
    println!(
        r#"
Trait       | C# Equivalent           | Purpose
-----------+--------------------------+----------------------------------
Display     | ToString()              | User-facing string representation
Debug       | ToString() with [Debug] | Developer string ({{:?}})
Clone       | ICloneable / .Clone()   | Explicit deep copy
Copy        | Value type semantics    | Implicit bitwise copy
PartialEq   | ==, .Equals()           | Equality comparison
Eq          | IEquatable<T> guarantee | Total equality (reflexive, etc)
PartialOrd  | IComparable<T>          | Partial ordering (<, >, <=, >=)
Ord         | IComparable<T> total    | Total ordering (.sort(), .min())
Hash        | GetHashCode()           | Use in HashMap/HashSet as key
Default     | new T() / default(T)    | Zero/empty value construction
From/Into   | implicit/explicit cast  | Value conversions
Iterator    | IEnumerable<T>          | Lazy sequences, LINQ
Drop        | IDisposable.Dispose()   | Cleanup on scope exit
Send        | Thread-safe reference   | Safe to send across threads
Sync        | Thread-safe shared ref  | Safe to share across threads
"#
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dog_speaks() {
        let d = Dog { name: "Rex".into() };
        assert_eq!(d.sound(), "Woof");
    }

    #[test]
    fn default_description() {
        let duck = Duck;
        assert!(duck.description().contains("Donald"));
    }

    #[test]
    fn blanket_summary() {
        assert_eq!(42_i32.summarize(), "Summary: 42");
    }
}
