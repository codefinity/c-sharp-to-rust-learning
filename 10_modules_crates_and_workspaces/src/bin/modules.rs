// ============================================================
// CONCEPT: Modules, Paths, and the `use` Statement
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses namespaces (namespace Foo.Bar) + `using Foo.Bar;`.
// Rust uses modules (mod foo { mod bar { } }) + `use foo::bar;`.
//
// Key mapping:
//   C# namespace  → Rust module (mod)
//   C# using      → Rust use
//   C# .csproj    → Rust Cargo.toml [package]
//   C# assembly   → Rust crate (a compiled unit)
//   C# NuGet pkg  → Rust crate from crates.io
//
// Module system rules:
//   • Files ARE modules (src/foo.rs = mod foo)
//   • Directories with mod.rs = mod name { ... }
//   • Or: directories with src/foo.rs + src/foo/ (modern style)
//
// RUN: cargo run --bin modules
// ============================================================

// ─── INLINE MODULES ──────────────────────────────────────────
mod geometry {
    // Private by default — only accessible within this module
    fn helper() -> &'static str { "internal" }

    // `pub` makes items public
    pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    pub mod shapes {
        #[derive(Debug, Clone)]
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn new(radius: f64) -> Self { Self { radius } }
            pub fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
        }

        // Use super:: to refer to the parent module
        pub fn describe(c: &Circle) -> String {
            let _ = super::helper(); // accessing parent's private item — OK from child
            format!("Circle(r={:.2}, area={:.2})", c.radius, c.area())
        }
    }
}

// ─── use DECLARATIONS ────────────────────────────────────────
use geometry::shapes::Circle;
use std::collections::HashMap;

// Renaming with `as`:
use std::fmt::Display as Displayable;

// Multiple items from the same path:
use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    module_basics();
    use_statement_demo();
    absolute_and_relative_paths();
    module_file_structure();
}

fn module_basics() {
    println!("=== Module Basics ===");

    // Access via full path:
    let c1 = geometry::shapes::Circle::new(5.0);
    println!("via full path: {}", geometry::shapes::describe(&c1));

    // Access via `use` (imported at top):
    let c2 = Circle::new(3.0);
    println!("via use: area = {:.2}", c2.area());

    // geometry::helper() is private — cannot be called here
    // geometry::helper(); // ← compile error

    let d = geometry::distance(0.0, 0.0, 3.0, 4.0);
    println!("distance(0,0 → 3,4) = {d:.2}");
}

fn use_statement_demo() {
    println!("\n=== use Statement ===");

    // Glob import — imports everything (like C# `using static` or `using *`)
    // Usually avoided — makes it unclear where names come from
    use geometry::shapes::*; // now Circle is in scope from here too

    let c = Circle::new(1.0);
    println!("glob import: {c:?}");

    // Re-export: make an imported item visible to OTHER modules/crates
    // `pub use` is the pattern for creating a nice public API:
    // pub use geometry::shapes::Circle; // re-exports Circle from this module
    println!("(re-export would be `pub use geometry::shapes::Circle`)");

    // HashMap and Arc were imported at the top with grouped use:
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);

    let shared: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
    let _ = thread::spawn(move || {
        let mut guard = shared.lock().unwrap();
        guard.push(4);
    }).join();
    println!("map: {map:?}");
}

fn absolute_and_relative_paths() {
    println!("\n=== Absolute vs Relative Paths ===");

    println!(
        r#"
Path resolution:
  crate::module::item      — absolute path from crate root
  self::submodule::item    — relative: current module
  super::item              — relative: parent module
  ::std::vec::Vec          — (rare) crate-relative absolute

C# namespace resolution:
  Foo.Bar.Baz             — always fully-qualified OR via `using`
  There's no relative namespace concept in C#

Examples:
  use crate::geometry::shapes::Circle;   // absolute
  use super::helpers::format_name;       // relative (go up one level)
  use self::inner::Impl;                 // relative (stay in current)
"#
    );

    // In Rust 2018+ (and 2024), `use` statements are always absolute
    // unless they start with `self::` or `super::`.
    // C#'s `using System.Collections` is roughly equivalent to
    // Rust's `use std::collections`.
}

fn module_file_structure() {
    println!("\n=== Module File Structure ===");

    println!(
        r#"
Traditional structure (all Rust editions):
  src/
    lib.rs         (or main.rs)  — root module
    foo.rs         — `mod foo;`  (inline module declaration)
    foo/
      mod.rs       — contents of mod foo {{ }}

Modern structure (Rust 2018+, preferred):
  src/
    lib.rs
    foo.rs         — mod foo {{ pub fn bar() {{ }} }}
    foo/
      bar.rs       — `mod bar;` declared in foo.rs

Inline modules (used in this tutorial for self-containment):
  mod foo {{          // everything in the same file
      pub fn bar() {{ }}
  }}

C# analogy:
  src/Foo/Bar.cs with `namespace Foo {{ class Bar {{ }} }}`
  → equivalent to src/foo/bar.rs with `pub struct Bar {{ }}`
     and `mod foo {{ pub mod bar {{ }} }}` in lib.rs

The difference: Rust requires EXPLICIT mod declarations.
A file existing does NOT automatically become a module.
"#
    );
}

#[cfg(test)]
mod tests {
    use super::geometry::shapes::Circle;

    #[test]
    fn circle_area() {
        let c = Circle::new(1.0);
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn distance_calculation() {
        let d = super::geometry::distance(0.0, 0.0, 3.0, 4.0);
        assert!((d - 5.0).abs() < 1e-10);
    }
}
