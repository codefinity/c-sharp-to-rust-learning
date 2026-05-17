// ============================================================
// CONCEPT: Structs — Named, Tuple, and Unit
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust structs replace BOTH C# classes AND structs.
// Key differences from C# classes:
//   • No inheritance (use traits for polymorphism)
//   • No virtual methods by default (no vtable unless you use dyn Trait)
//   • Methods are defined in separate `impl` blocks
//   • Structs are value types by default (stack-allocated when not boxed)
//   • No nullability — use Option<T>
//   • No constructors — use associated functions (conventionally `new`)
//
// C# class  → Rust struct + impl block
// C# struct → Rust struct + #[derive(Copy)] (if small enough)
// C# record → Rust struct + #[derive(PartialEq, Clone, Debug)]
//
// RUN: cargo run --bin structs
// ============================================================

fn main() {
    named_structs();
    tuple_structs();
    unit_structs();
    methods_and_associated_functions();
    struct_update_syntax();
    debug_display();
    derived_traits();
}

// ─── NAMED STRUCTS ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // Associated function (no `self`) — conventional constructor
    // C# analogy: static User Create(...) or the constructor itself
    fn new(username: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: 0,
            active: true,
        }
    }

    // Method (takes &self) — immutable access
    fn display_name(&self) -> &str {
        &self.username
    }

    // Mutable method (takes &mut self)
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    // Consuming method (takes self — moves the struct)
    fn deactivate(mut self) -> Self {
        self.active = false;
        self
    }
}

fn named_structs() {
    println!("=== Named Structs ===");

    let mut user1 = User::new("alice", "alice@example.com");
    user1.increment_sign_in();
    user1.increment_sign_in();

    println!("User: {user1:?}");
    println!("Display name: {}", user1.display_name());
    println!("Sign-ins: {}", user1.sign_in_count);

    // Field access (public by default within crate; use `pub` for external access)
    println!("Active: {}", user1.active);

    let deactivated = user1.deactivate(); // user1 moved here
    println!("Active after deactivate: {}", deactivated.active);
}

// ─── TUPLE STRUCTS ───────────────────────────────────────────
// Tuple structs are newtype wrappers — give semantic meaning to primitives.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Kilograms(f64);

// This prevents accidentally mixing up units:
fn calculate_bmi(weight: Kilograms, height: Meters) -> f64 {
    weight.0 / (height.0 * height.0)
}

#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8); // RGB

fn tuple_structs() {
    println!("\n=== Tuple Structs (Newtype Pattern) ===");

    let weight = Kilograms(70.0);
    let height = Meters(1.75);
    let bmi = calculate_bmi(weight, height);
    println!("BMI: {bmi:.2}");

    // cannot_mix(weight, height) because Meters and Kilograms are different types:
    // calculate_bmi(height, weight); // ← compile error: wrong types!

    let red = Color(255, 0, 0);
    println!("Red: {:?}", red);
    println!("R={} G={} B={}", red.0, red.1, red.2);
}

// ─── UNIT STRUCTS ────────────────────────────────────────────
// Unit structs have no fields. They're useful as marker types or
// for implementing traits on a type that needs no data.

struct AlwaysEqual;

struct Validator;

impl Validator {
    fn validate(self, input: &str) -> bool {
        !input.is_empty() && input.len() < 100
    }
}

fn unit_structs() {
    println!("\n=== Unit Structs ===");

    let _ = AlwaysEqual; // zero-sized, takes no stack space
    let v = Validator;
    println!("valid: {}", v.validate("hello"));
    println!("invalid: {}", Validator.validate(""));
}

fn methods_and_associated_functions() {
    println!("\n=== Methods and Associated Functions ===");

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        // Associated function — like C# static method
        fn square(size: f64) -> Self {
            Self { width: size, height: size }
        }

        fn area(&self) -> f64 { self.width * self.height }
        fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
        fn is_square(&self) -> bool { (self.width - self.height).abs() < f64::EPSILON }

        fn scale(&mut self, factor: f64) {
            self.width  *= factor;
            self.height *= factor;
        }

        // Can have multiple impl blocks — useful for organisation
    }

    // Second impl block — perfectly valid
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let mut r = Rectangle { width: 10.0, height: 5.0 };
    println!("area: {}", r.area());
    println!("perimeter: {}", r.perimeter());
    println!("is square: {}", r.is_square());
    r.scale(2.0);
    println!("after scale: {r:?}");

    let sq = Rectangle::square(4.0);
    println!("square: {sq:?}  is_square: {}", sq.is_square());
    println!("r can hold sq: {}", r.can_hold(&sq));
}

fn struct_update_syntax() {
    println!("\n=== Struct Update Syntax ===");

    let user1 = User::new("alice", "alice@example.com");
    // Create user2 based on user1, with different email:
    // C# equivalent: user1 with { Email = "bob@example.com" }  (record with expression)
    let user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        ..user1 // remaining fields from user1
        // ⚠️ bool and u64 are Copy so user1 is still partially valid
    };
    println!("user2: {user2:?}");
}

fn debug_display() {
    println!("\n=== Debug and Display ===");

    // Debug is auto-derivable — for developer output ({:?} and {:#?})
    // Display requires manual impl — for user-facing output ({})

    use std::fmt;

    #[derive(Debug)]
    struct Point { x: f64, y: f64 }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 3.0, y: 4.0 };
    println!("Debug: {:?}", p);   // Point { x: 3.0, y: 4.0 }
    println!("Pretty: {:#?}", p); // multiline
    println!("Display: {p}");     // (3, 4)
}

fn derived_traits() {
    println!("\n=== Common Derived Traits ===");

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct Priority {
        level: u8,
        name: String,
    }

    let p1 = Priority { level: 1, name: "low".into() };
    let p2 = Priority { level: 2, name: "high".into() };

    println!("p1 == p2: {}", p1 == p2);
    println!("p1 < p2: {}", p1 < p2);

    let mut priorities = vec![p2.clone(), p1.clone()];
    priorities.sort();
    println!("sorted: {priorities:?}");

    // Use as HashMap key (requires Hash + Eq):
    use std::collections::HashMap;
    let mut map: HashMap<Priority, &str> = HashMap::new();
    map.insert(p1, "low priority task");
    map.insert(p2, "high priority task");
    println!("map len: {}", map.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_new() {
        let u = User::new("test", "test@example.com");
        assert_eq!(u.username, "test");
        assert!(u.active);
        assert_eq!(u.sign_in_count, 0);
    }

    #[test]
    fn bmi_calculation() {
        let bmi = calculate_bmi(Kilograms(70.0), Meters(1.75));
        assert!((bmi - 22.857).abs() < 0.001);
    }

    #[test]
    fn struct_update_preserves_copy_fields() {
        let u1 = User::new("alice", "a@example.com");
        let count = u1.sign_in_count;
        let u2 = User {
            email: "b@example.com".into(),
            username: "bob".into(),
            ..u1
        };
        assert_eq!(u2.sign_in_count, count);
    }
}
