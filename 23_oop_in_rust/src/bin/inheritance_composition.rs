// ============================================================
// OOP PILLAR 2: Inheritance → Composition
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust has NO class inheritance. This is a deliberate design choice.
// Problems inheritance causes (that Rust avoids):
//   - Diamond problem: class D : B, C where B : A and C : A
//   - Fragile base class: changing Animal breaks Dog, Cat, Parrot...
//   - Tight coupling: subclass depends on base class internals
//   - Unexpected behaviour: virtual method called in constructor
//
// C# inheritance:
//   abstract class Animal {
//       public string Name { get; }
//       public abstract void Speak();
//       public virtual string Describe() => $"{Name} says: ";
//   }
//   class Dog : Animal {
//       public override void Speak() => Console.WriteLine("Woof!");
//   }
//
// Rust replaces inheritance with three composable tools:
//   1. Traits         — shared behaviour (like interfaces + abstract methods)
//   2. Default methods — reusable implementations (like non-abstract base methods)
//   3. Composition     — embed structs as fields (has-a instead of is-a)
//
// RUN: cargo run --bin inheritance_composition
// ============================================================

fn main() {
    println!("=== OOP Pillar 2: Inheritance → Composition ===\n");

    demo_trait_as_abstract_class();
    demo_composition();
    demo_delegation();
    demo_trait_inheritance();
    demo_mixin_via_traits();
    demo_diamond_problem_solved();
}

// ─── 1. TRAIT AS ABSTRACT CLASS ──────────────────────────────────────────────
//
// C#:
//   abstract class Animal {
//       public string Name { get; init; }
//       public abstract void Speak();                    // must override
//       public virtual string Describe()                 // may override
//           => $"I am {Name}.";
//       public void Sleep() => Console.WriteLine($"{Name} is sleeping."); // sealed behaviour
//   }
//
// Rust: a trait with required methods + default methods maps directly.
// No field sharing across impls (each struct owns its own data).

trait Animal {
    // Required — every impl must define this (like `abstract` in C#):
    fn name(&self) -> &str;
    fn speak(&self);

    // Default implementation — impls may override this (like `virtual` in C#):
    fn describe(&self) -> String {
        format!("I am {}.", self.name())
    }

    // Default implementation that impls typically do NOT override (like non-virtual):
    fn sleep(&self) {
        println!("{} is sleeping.", self.name());
    }
}

struct Dog { name: String }
struct Cat { name: String, indoor: bool }

impl Animal for Dog {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) { println!("{}: Woof!", self.name); }
    // describe() uses default — no override needed
}

impl Animal for Cat {
    fn name(&self) -> &str { &self.name }
    fn speak(&self) { println!("{}: Meow!", self.name); }

    // Override the default (like `override` in C#):
    fn describe(&self) -> String {
        let location = if self.indoor { "indoor" } else { "outdoor" };
        format!("I am {}, an {} cat.", self.name(), location)
    }
}

fn demo_trait_as_abstract_class() {
    println!("--- 1. Trait as Abstract Class ---");
    println!();

    let dog = Dog { name: "Rex".to_string() };
    let cat = Cat { name: "Whiskers".to_string(), indoor: true };

    dog.speak();
    cat.speak();
    println!("{}", dog.describe());  // uses default impl
    println!("{}", cat.describe());  // uses overridden impl
    dog.sleep();
    cat.sleep();

    println!(r#"
  C# abstract class                    Rust trait
  ─────────────────────────────────────────────────────────────────
  abstract void Speak();               fn speak(&self);             // required
  virtual string Describe() {{ ... }}  fn describe(&self) -> String {{ ... }} // default
  void Sleep() {{ ... }}               fn sleep(&self) {{ ... }}    // default (non-virtual feel)
  public string Name {{ get; }}        fn name(&self) -> &str;      // required getter
  class Dog : Animal {{ ... }}         impl Animal for Dog {{ ... }}
  override void Speak() {{ ... }}      fn speak(&self) {{ ... }}    // same syntax as required
"#);
}

// ─── 2. COMPOSITION (HAS-A) ──────────────────────────────────────────────────
//
// C# inheritance is "is-a": a Manager IS-A Employee.
// Rust composition is "has-a": a Manager HAS-A Employee (stored as a field).
//
// C#:
//   class Employee {
//       public string Name { get; }
//       public decimal Salary { get; protected set; }
//       public virtual string Role() => "Employee";
//   }
//   class Manager : Employee {
//       public int Reports { get; }
//       public override string Role() => "Manager";
//   }
//
// Rust: Manager contains an Employee. Access employee data via the field.

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: f64,
}

impl Employee {
    fn new(name: &str, salary: f64) -> Self {
        Employee { name: name.to_string(), salary }
    }
    fn name(&self)   -> &str { &self.name }
    fn salary(&self) -> f64  { self.salary }
    fn role(&self)   -> &str { "Employee" }

    fn give_raise(&mut self, amount: f64) {
        self.salary += amount;
    }
}

#[derive(Debug)]
struct Manager {
    employee: Employee,  // "has-a" Employee — composition
    reports: u32,
}

impl Manager {
    fn new(name: &str, salary: f64, reports: u32) -> Self {
        Manager { employee: Employee::new(name, salary), reports }
    }

    // Access base data via the field:
    fn name(&self)    -> &str { self.employee.name() }
    fn salary(&self)  -> f64  { self.employee.salary() }
    fn reports(&self) -> u32  { self.reports }
    fn role(&self)    -> &str { "Manager" }        // "override"

    fn give_raise(&mut self, amount: f64) {
        self.employee.give_raise(amount);           // delegate to inner struct
    }
}

fn demo_composition() {
    println!("--- 2. Composition (has-a instead of is-a) ---");
    println!();

    let emp = Employee::new("Alice", 70_000.0);
    let mut mgr = Manager::new("Bob", 95_000.0, 5);

    println!("{} is a {} earning ${:.0}", emp.name(), emp.role(), emp.salary());
    println!("{} is a {} managing {} reports, earning ${:.0}",
        mgr.name(), mgr.role(), mgr.reports(), mgr.salary());

    mgr.give_raise(5_000.0);
    println!("After raise: {} earns ${:.0}", mgr.name(), mgr.salary());

    println!(r#"
  C# inheritance                       Rust composition
  ─────────────────────────────────────────────────────────────────
  class Manager : Employee             struct Manager {{ employee: Employee, reports: u32 }}
  base.GiveRaise(amount)               self.employee.give_raise(amount)
  this.Name (inherited)                self.employee.name()  // explicit path
  override string Role() {{ ... }}     fn role(&self) -> &str {{ "Manager" }}

  Benefit: Manager can contain multiple independent structs.
  C# can only have ONE base class. Rust has no such limit on composition.
"#);
}

// ─── 3. DELEGATION WITH DEREF ────────────────────────────────────────────────
//
// When you want call syntax that feels like inheritance (obj.method()
// instead of obj.inner.method()), implement std::ops::Deref.
//
// C#:  manager.Name  (inherited from Employee — no indirection visible)
// Rust: implement Deref<Target = Employee> so manager.name() just works.

use std::ops::Deref;

struct SeniorManager {
    manager: Manager,
    budget: f64,
}

impl SeniorManager {
    fn new(name: &str, salary: f64, reports: u32, budget: f64) -> Self {
        SeniorManager {
            manager: Manager::new(name, salary, reports),
            budget,
        }
    }
    fn budget(&self) -> f64 { self.budget }
    fn role(&self) -> &str { "Senior Manager" }
}

// Deref lets SeniorManager.name(), .salary(), .reports() work directly
// by auto-dereferencing to Manager (which in turn can deref to Employee).
impl Deref for SeniorManager {
    type Target = Manager;
    fn deref(&self) -> &Manager { &self.manager }
}

fn demo_delegation() {
    println!("--- 3. Delegation via Deref ---");
    println!();

    let sm = SeniorManager::new("Carol", 120_000.0, 12, 500_000.0);

    // These call Manager's methods via Deref — no explicit .manager:
    println!("{} | role: {} | reports: {} | budget: ${:.0}",
        sm.name(), sm.role(), sm.reports(), sm.budget());

    println!(r#"
  Deref<Target = Manager> on SeniorManager means:
    sm.name()    → (*sm).name()    → sm.manager.employee.name()
    sm.reports() → (*sm).reports() → sm.manager.reports()

  This gives call-site ergonomics similar to inheritance without coupling.
  Use sparingly — it can make code hard to follow if overused.
"#);
}

// ─── 4. TRAIT INHERITANCE ────────────────────────────────────────────────────
//
// Traits can require other traits — this is "trait inheritance".
// It is NOT data inheritance; it is capability inheritance.
//
// C#:
//   interface IShape { double Area(); }
//   interface IPrintable : IShape { void Print(); }   // IPrintable requires IShape
//
// Rust: same idea with trait bounds on the trait definition itself.

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

// Printable REQUIRES Shape — any impl of Printable must also impl Shape:
trait Printable: Shape {
    fn print(&self) {
        // Can call Shape methods here because Printable: Shape guarantees them:
        println!("[{}] area = {:.2}", self.name(), self.area());
    }
}

#[derive(Debug)]
struct Circle { radius: f64 }

#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}
impl Printable for Circle {}  // inherits default print() from Printable

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn name(&self) -> &str { "Rectangle" }
}
impl Printable for Rectangle {
    // Override default print() — like C# `new` hiding or explicit implementation:
    fn print(&self) {
        println!("[Rectangle {:.1}×{:.1}] area = {:.2}", self.width, self.height, self.area());
    }
}

fn demo_trait_inheritance() {
    println!("--- 4. Trait Inheritance ---");
    println!();

    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };

    c.print();  // uses default Printable::print
    r.print();  // uses overridden print

    println!(r#"
  C#                                   Rust
  ─────────────────────────────────────────────────────────────────
  interface IPrintable : IShape        trait Printable: Shape
  // impl must also impl IShape        // impl must also impl Shape
  void Print() {{ ... }}               fn print(&self) {{ ... }}  // default method
"#);
}

// ─── 5. MIXIN PATTERN VIA MULTIPLE TRAITS ────────────────────────────────────
//
// C# has no mixins, but you can approximate with default interface members.
// Rust traits with default methods ARE mixins — implement the trait, get the behaviour.
//
// A type can implement any number of traits (unlike C# single-inheritance classes).

trait Flyable {
    fn altitude(&self) -> f64;
    fn fly(&self) {
        println!("{} is flying at {:.0}m altitude.", self.creature_name(), self.altitude());
    }
    fn creature_name(&self) -> &str;
}

trait Swimmable {
    fn depth(&self) -> f64;
    fn swim(&self) {
        println!("{} is swimming at {:.0}m depth.", self.creature_name2(), self.depth());
    }
    fn creature_name2(&self) -> &str;
}

struct Duck { name: String }

impl Flyable for Duck {
    fn altitude(&self) -> f64 { 50.0 }
    fn creature_name(&self) -> &str { &self.name }
    // fly() is inherited from the trait default
}

impl Swimmable for Duck {
    fn depth(&self) -> f64 { 1.5 }
    fn creature_name2(&self) -> &str { &self.name }
    // swim() is inherited from the trait default
}

fn demo_mixin_via_traits() {
    println!("--- 5. Mixin Pattern via Multiple Traits ---");
    println!();

    let duck = Duck { name: "Donald".to_string() };
    duck.fly();   // from Flyable mixin
    duck.swim();  // from Swimmable mixin

    println!(r#"
  Duck implements BOTH Flyable and Swimmable.
  Each trait contributes default behaviour — this is the mixin pattern.

  C# equivalent (C# 8+ default interface methods):
    interface IFlyable {{ void Fly() {{ Console.WriteLine("flying"); }} }}
    interface ISwimmable {{ void Swim() {{ Console.WriteLine("swimming"); }} }}
    class Duck : IFlyable, ISwimmable {{ }}

  Rust advantage: traits can be implemented for types from other crates
  (blanket impls), which C# interfaces cannot do retroactively.
"#);
}

// ─── 6. DIAMOND PROBLEM — SOLVED ─────────────────────────────────────────────
//
// C# solves the diamond problem with explicit interface implementation.
// Rust avoids it entirely: if two traits define a method with the same name,
// you call them with fully-qualified syntax — no ambiguity, no silent override.
//
//   C# diamond:             Rust diamond:
//   interface IA { void M(); }   trait TraitA { fn m(&self); }
//   interface IB : IA { }        trait TraitB { fn m(&self); }
//   interface IC : IA { }        struct Foo;
//   class D : IB, IC { }         impl TraitA for Foo { fn m(&self) { ... } }
//                                impl TraitB for Foo { fn m(&self) { ... } }

trait TraitA { fn describe(&self) -> &str; }
trait TraitB { fn describe(&self) -> &str; }

struct Foo;
impl TraitA for Foo { fn describe(&self) -> &str { "TraitA::describe" } }
impl TraitB for Foo { fn describe(&self) -> &str { "TraitB::describe" } }

fn demo_diamond_problem_solved() {
    println!("--- 6. Diamond Problem — Solved by Design ---");
    println!();

    let foo = Foo;

    // Fully-qualified syntax disambiguates:
    println!("{}", TraitA::describe(&foo));
    println!("{}", TraitB::describe(&foo));

    println!(r#"
  Both TraitA and TraitB have `describe`. There is no silent override.
  The compiler forces you to use TraitA::describe(&foo) or TraitB::describe(&foo).

  C# requires `void IB.M() {{ ... }}` explicit interface implementation.
  Rust requires fully-qualified syntax: TraitA::describe(&foo).

  Since Rust has no data inheritance, the classic diamond data-sharing
  problem (who owns the shared base fields?) simply cannot occur.
"#);
}

// ─── TESTS ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dog_uses_default_describe() {
        let dog = Dog { name: "Buddy".to_string() };
        assert_eq!(dog.describe(), "I am Buddy.");
    }

    #[test]
    fn cat_overrides_describe() {
        let cat = Cat { name: "Luna".to_string(), indoor: false };
        assert!(cat.describe().contains("outdoor"));
    }

    #[test]
    fn manager_delegates_raise_to_employee() {
        let mut mgr = Manager::new("Dave", 80_000.0, 3);
        mgr.give_raise(10_000.0);
        assert!((mgr.salary() - 90_000.0).abs() < 0.001);
    }

    #[test]
    fn circle_area() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn diamond_disambiguation() {
        let foo = Foo;
        assert_eq!(TraitA::describe(&foo), "TraitA::describe");
        assert_eq!(TraitB::describe(&foo), "TraitB::describe");
    }
}
