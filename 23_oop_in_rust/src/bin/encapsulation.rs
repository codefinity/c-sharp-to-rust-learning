// ============================================================
// OOP PILLAR 1: Encapsulation
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Encapsulation means bundling data with the methods that operate
// on it, and hiding internal state from the outside world.
//
// C# achieves this with:
//   - private/protected/internal fields
//   - public properties (get; set;)
//   - constructors that validate
//   - sealed/readonly modifiers
//
// Rust achieves this with:
//   - fields private to the module by default (no keyword needed)
//   - explicit getter/setter methods (no property syntax)
//   - associated fn `new()` returning Result<Self, E> for fallible init
//   - immutability by default (let vs let mut)
//   - pub / pub(crate) / pub(super) for graduated visibility
//
// KEY MINDSET SHIFT:
//   C#: members are private when you say `private`
//   Rust: members are private UNLESS you say `pub`
//
// RUN: cargo run --bin encapsulation
// ============================================================

fn main() {
    println!("=== OOP Pillar 1: Encapsulation ===\n");

    demo_field_visibility();
    demo_getters_setters();
    demo_invariant_protection();
    demo_visibility_modifiers();
    demo_immutability();
}

// ─── 1. FIELD VISIBILITY ─────────────────────────────────────────────────────
//
// C#:
//   public class Person {
//       private string _name;           // hidden from outside
//       public  string Email { get; }   // read-only property
//       internal int   Age;             // same-assembly only
//   }
//
// Rust: fields are private to their MODULE by default.
// Use `pub` to expose them. Rust has no `protected` — trait methods
// are the idiomatic substitute.

pub struct Person {
    pub name: String,        // readable + writable from any code that imports this
    pub(crate) age: u32,     // like C# `internal` — visible within this crate only
    email: String,           // private — only accessible inside this file/module
}

impl Person {
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Person {
            name: name.to_string(),
            age,
            email: email.to_string(),
        }
    }

    // A public getter exposes the private field read-only.
    // Named the same as the field — idiomatic Rust convention.
    pub fn email(&self) -> &str {
        &self.email
    }
}

fn demo_field_visibility() {
    println!("--- 1. Field Visibility ---");
    println!();

    let p = Person::new("Alice", 30, "alice@example.com");

    println!("name  (pub)         = {}", p.name);
    println!("age   (pub(crate))  = {}", p.age);
    // p.email would NOT compile — it is private
    println!("email (via getter)  = {}", p.email());

    println!(r#"
  C#                                   Rust
  ────────────────────────────────────────────────────────────
  public string Name;                  pub name: String,
  private string _email;               email: String,   // default = private
  internal int Age;                    pub(crate) age: u32,
  protected string Role;               (no direct equiv — expose via trait)
"#);
}

// ─── 2. GETTERS AND SETTERS ──────────────────────────────────────────────────
//
// C#:
//   public class BankAccount {
//       private decimal _balance;
//       public  decimal Balance  { get => _balance; }          // read-only
//       public  string  Owner    { get; private set; }         // external read, internal write
//       public  string  Label    { get; set; } = "default";    // full auto-property
//
//       public void Deposit(decimal amount) { ... }
//   }
//
// Rust: no property syntax. Write explicit methods.
// Convention: getter = same name as field, setter = set_<field>.
// Prefer domain methods (deposit, withdraw) over raw setters where possible.

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,   // private — only mutated through domain methods
    label: String,
}

impl BankAccount {
    pub fn new(owner: &str) -> Self {
        BankAccount {
            owner: owner.to_string(),
            balance: 0.0,
            label: "default".to_string(),
        }
    }

    // Read-only getter — equivalent to `public decimal Balance { get; }`:
    pub fn balance(&self) -> f64 { self.balance }

    // Read-only getter — equivalent to `public string Owner { get; private set; }`:
    pub fn owner(&self) -> &str { &self.owner }

    // Getter + setter — equivalent to `public string Label { get; set; }`:
    pub fn label(&self) -> &str { &self.label }
    pub fn set_label(&mut self, label: &str) { self.label = label.to_string(); }

    // Domain methods are preferred over raw setters — they encode business rules:
    pub fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("deposit amount must be positive".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err(format!("insufficient funds: balance is {:.2}", self.balance));
        }
        self.balance -= amount;
        Ok(())
    }
}

fn demo_getters_setters() {
    println!("--- 2. Getters and Setters ---");
    println!();

    let mut account = BankAccount::new("Bob");
    account.set_label("premium");
    account.deposit(1000.0).unwrap();
    account.withdraw(250.0).unwrap();

    println!("owner:   {}", account.owner());
    println!("balance: {:.2}", account.balance());
    println!("label:   {}", account.label());

    let err = account.withdraw(5000.0).unwrap_err();
    println!("withdraw 5000 → error: {err}");

    println!(r#"
  C# property                             Rust equivalent
  ──────────────────────────────────────────────────────────────────
  decimal Balance {{ get; }}              pub fn balance(&self) -> f64
  string Owner {{ get; private set; }}   pub fn owner(&self) + no public setter
  string Label {{ get; set; }}           pub fn label(&self) + pub fn set_label(&mut self, ..)
  void Deposit(decimal d) {{ ... }}      pub fn deposit(&mut self, d: f64) -> Result<(), String>
"#);
}

// ─── 3. INVARIANT PROTECTION ─────────────────────────────────────────────────
//
// C#:
//   public class Temperature {
//       private readonly double _celsius;
//       public Temperature(double c) {
//           if (c < -273.15) throw new ArgumentException("below absolute zero");
//           _celsius = c;
//       }
//   }
//
// Rust: constructors that can fail return Result<Self, E>.
// This forces callers to handle the error — no unchecked exceptions.
// The private field guarantees no one can construct an invalid value
// by any other path.

#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    celsius: f64,  // private: the ONLY way to create this is through new()
}

impl Temperature {
    /// Fails if `celsius` is below absolute zero (−273.15 °C).
    /// C# equivalent: constructor that throws ArgumentException.
    pub fn new(celsius: f64) -> Result<Self, String> {
        if celsius < -273.15 {
            return Err(format!("{celsius:.2} °C is below absolute zero (−273.15 °C)"));
        }
        Ok(Temperature { celsius })
    }

    pub fn celsius(&self)    -> f64 { self.celsius }
    pub fn fahrenheit(&self) -> f64 { self.celsius * 9.0 / 5.0 + 32.0 }
    pub fn kelvin(&self)     -> f64 { self.celsius + 273.15 }
}

fn demo_invariant_protection() {
    println!("--- 3. Invariant Protection ---");
    println!();

    // Valid construction:
    let boiling = Temperature::new(100.0).expect("valid temperature");
    println!("Boiling point: {:.1} °C = {:.1} °F = {:.2} K",
        boiling.celsius(), boiling.fahrenheit(), boiling.kelvin());

    // Invalid construction — no exception, just an Err:
    match Temperature::new(-300.0) {
        Ok(_)  => println!("created (unexpected)"),
        Err(e) => println!("Rejected invalid value: {e}"),
    }

    println!(r#"
  C#                                      Rust
  ────────────────────────────────────────────────────────────────────
  throw new ArgumentException(msg)        return Err(msg.to_string())
  try {{ var t = new Temperature(-300); }}  match Temperature::new(-300.0) {{ Ok / Err }}
  private readonly field                  private field + only set inside new()
  No way to bypass the ctor              No way to bypass — private field
"#);
}

// ─── 4. VISIBILITY MODIFIERS ─────────────────────────────────────────────────

fn demo_visibility_modifiers() {
    println!("--- 4. Visibility Modifiers ---");

    println!(r#"
  C# modifier            Rust equivalent        Scope
  ─────────────────────────────────────────────────────────────────────────
  public                 pub                    Visible everywhere
  private                (default, no keyword)  Visible within the module only
  internal               pub(crate)             Visible within the same crate
  protected              (no direct equiv)      Expose via trait methods instead
  protected internal     pub(super)             Visible to the parent module
  private protected      pub(in path::to::mod)  Restricted to a specific module path

  Key difference: C# classes default to `internal`, members to `private`.
  Rust: everything defaults to private to the current module. `pub` opts out.

  Rust visibility is path-based, not type-based — it does not care about
  the class hierarchy (there is none), only the module hierarchy.
"#);
}

// ─── 5. IMMUTABILITY BY DEFAULT ──────────────────────────────────────────────
//
// C#:
//   public class Point {
//       public readonly double X;
//       public readonly double Y;
//       public Point(double x, double y) { X = x; Y = y; }
//       public Point Translate(double dx, double dy) => new Point(X + dx, Y + dy);
//   }
//
//   // C# record (immutable by default since C# 9):
//   public record Point(double X, double Y) {
//       public Point Translate(double dx, double dy) => this with { X = X + dx, Y = Y + dy };
//   }
//
// Rust: ALL bindings are immutable by default (like every field were `readonly`).
// `let mut` is required to allow mutation. Returning a new value (like `with` in
// C# records) is the standard pattern for immutable transformations.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Returns a NEW point — same as C# record `with` expression.
    // The original is untouched because we take &self (shared borrow).
    pub fn translate(&self, dx: f64, dy: f64) -> Point {
        Point { x: self.x + dx, y: self.y + dy }
    }
}

fn demo_immutability() {
    println!("--- 5. Immutability by Default ---");
    println!();

    let p1 = Point::new(0.0, 0.0);
    // p1.x = 5.0;  ← compile error: cannot assign to `p1.x` — `p1` is not declared as mutable

    let p2 = Point::new(3.0, 4.0);
    println!("p1 = ({}, {})", p1.x, p1.y);
    println!("p2 = ({}, {})", p2.x, p2.y);
    println!("distance p1→p2 = {:.1}", p1.distance_to(&p2));

    // Immutable transformation — p1 is unchanged, p3 is a new value:
    let p3 = p1.translate(1.0, 2.0);
    println!("p3 (p1 translated by 1,2) = ({}, {})", p3.x, p3.y);
    println!("p1 unchanged              = ({}, {})", p1.x, p1.y);

    // mut is required to allow mutation in place:
    let mut p4 = Point::new(10.0, 10.0);
    p4.x += 5.0;
    println!("p4 (mutated in place) = ({}, {})", p4.x, p4.y);

    println!(r#"
  C#                              Rust
  ───────────────────────────────────────────────────────────
  readonly double X;               pub x: f64,  // + let binding (immutable by default)
  var p = new Point(1, 2);         let p = Point::new(1.0, 2.0);
  // p.X = 5; -- compile error     // p.x = 5.0; -- compile error (not mut)
  p with {{ X = p.X + 1 }}         p.translate(1.0, 0.0)  // returns new Point
  // mutable local:                let mut p = Point::new(0.0, 0.0);
  Point p = new Point(0, 0);       p.x += 1.0;  // OK because `mut`
  p = new Point(p.X + 1, p.Y);
"#);
}

// ─── TESTS ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature_valid() {
        let t = Temperature::new(100.0).unwrap();
        assert!((t.fahrenheit() - 212.0).abs() < 0.001);
        assert!((t.kelvin() - 373.15).abs() < 0.001);
    }

    #[test]
    fn temperature_rejects_below_absolute_zero() {
        assert!(Temperature::new(-273.16).is_err());
        assert!(Temperature::new(-273.15).is_ok()); // boundary is allowed
    }

    #[test]
    fn bank_account_rejects_overdraft() {
        let mut acc = BankAccount::new("Test");
        acc.deposit(100.0).unwrap();
        assert!(acc.withdraw(200.0).is_err());
        assert!((acc.balance() - 100.0).abs() < 0.001); // unchanged
    }

    #[test]
    fn point_translate_is_immutable() {
        let p = Point::new(1.0, 2.0);
        let p2 = p.translate(3.0, 4.0);
        assert_eq!(p, Point::new(1.0, 2.0));   // original unchanged
        assert_eq!(p2, Point::new(4.0, 6.0));
    }

    #[test]
    fn person_email_is_private() {
        let p = Person::new("Eve", 25, "eve@example.com");
        assert_eq!(p.email(), "eve@example.com");
        // There is no way to write p.email = "other" — it is private.
    }
}
