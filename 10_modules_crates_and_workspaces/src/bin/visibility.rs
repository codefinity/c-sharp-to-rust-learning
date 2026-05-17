// ============================================================
// CONCEPT: Visibility — pub, pub(crate), pub(super)
// ============================================================
// RUN: cargo run --bin visibility
// ============================================================

mod outer {
    // Private — only accessible within this module and its children
    fn private_fn() -> &'static str { "private" }

    // pub — accessible from everywhere the module is accessible
    pub fn public_fn() -> &'static str { "public" }

    // pub(crate) — accessible only within this crate (like C# `internal`)
    pub(crate) fn crate_fn() -> &'static str { "crate-internal" }

    // pub(super) — accessible to the parent module only
    pub(super) fn super_fn() -> &'static str { "visible to parent" }

    pub struct PublicStruct {
        pub    public_field:  i32,      // pub: accessible everywhere struct is
        pub(crate) crate_field: String, // C# internal
        private_field: f64,             // private (default)
    }

    impl PublicStruct {
        pub fn new(public: i32, crate_val: &str, private: f64) -> Self {
            Self {
                public_field: public,
                crate_field: crate_val.to_string(),
                private_field: private,
            }
        }

        pub fn describe(&self) -> String {
            // Can access all fields within the impl:
            format!("pub={} crate={} private={}", self.public_field, self.crate_field, self.private_field)
        }
    }

    pub mod inner {
        // inner can call super::private_fn — parent's private items
        pub fn call_parent_private() -> &'static str {
            super::private_fn()
        }
    }
}

fn main() {
    println!("=== Visibility Levels ===");

    println!("public: {}", outer::public_fn());
    println!("crate_fn: {}", outer::crate_fn()); // ok — same crate
    println!("super_fn: {}", outer::super_fn()); // ok — we ARE the parent

    // outer::private_fn(); // ← compile error

    let s = outer::PublicStruct::new(42, "hello", 3.14);
    println!("public_field: {}", s.public_field);  // ok
    println!("crate_field: {}", s.crate_field);    // ok (same crate)
    // s.private_field; // ← compile error
    println!("{}", s.describe());

    // inner module:
    println!("inner calling parent private: {}", outer::inner::call_parent_private());

    println!(
        r#"
Visibility Summary:
  (no modifier) — private to current module and its children
  pub(self)     — same as no modifier (explicit)
  pub(super)    — visible to parent module
  pub(crate)    — visible throughout the crate (C# internal)
  pub(in path)  — visible in the specified path
  pub           — public to all (C# public)

C# analogy:
  private        → (no modifier) in Rust
  internal       → pub(crate)
  protected      → no direct equivalent (no inheritance)
  public         → pub
"#
    );
}

#[cfg(test)]
mod tests {
    use super::outer;

    #[test]
    fn public_fn_accessible() {
        assert_eq!(outer::public_fn(), "public");
    }

    #[test]
    fn crate_fn_accessible_from_same_crate() {
        assert_eq!(outer::crate_fn(), "crate-internal");
    }
}
