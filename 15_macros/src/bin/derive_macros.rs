// ============================================================
// CONCEPT: Derive Macros and Attributes
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses attributes [Serializable], [JsonPropertyName], interfaces
// (IEquatable<T>, IComparable<T>), and source generators to add
// behavior to types at compile time.
//
// Rust uses #[derive(...)] to auto-implement standard traits,
// and custom derive macros (proc macros) for third-party attributes
// like #[derive(Serialize, Deserialize)] from serde.
//
// Built-in derivable traits are shown here. Proc macros (custom
// derive) require a separate crate to implement; this file
// demonstrates the USAGE side.
//
// RUN: cargo run --bin derive_macros
// ============================================================

fn main() {
    println!("=== Derive Macros & Attributes ===\n");

    derive_debug();
    derive_clone_copy();
    derive_eq_ord();
    derive_default();
    derive_hash();
    attributes_overview();
    cfg_attributes();
}

// ---- 1. Debug and Display ----------------------------------------

#[derive(Debug)]          // auto-generates fmt::Debug — C# Object.ToString override
struct Point {
    x: f64,
    y: f64,
}

// Implement Display manually (no derive for Display):
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

fn derive_debug() {
    println!("--- #[derive(Debug)] ---");

    let p = Point { x: 1.0, y: 2.0 };
    println!("Debug:   {:?}", p);
    println!("Pretty:  {:#?}", p);
    println!("Display: {}", p);

    let c = Color::Custom(128, 0, 255);
    println!("Color: {c:?}");
}

// ---- 2. Clone and Copy ------------------------------------------

#[derive(Debug, Clone)]        // Clone — explicit deep copy
struct Config {
    name: String,
    retries: u32,
}

#[derive(Debug, Clone, Copy)]  // Copy — implicit bitwise copy (requires all fields Copy)
struct Vec2 {
    x: f32,
    y: f32,
}

fn derive_clone_copy() {
    println!("\n--- #[derive(Clone, Copy)] ---");

    let cfg1 = Config { name: "prod".to_string(), retries: 3 };
    let cfg2 = cfg1.clone();      // explicit clone (has String, can't be Copy)
    println!("cfg1: {:?}", cfg1);
    println!("cfg2 (cloned): {:?}", cfg2);

    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = v1;                  // implicit copy (all fields are f32 → Copy)
    println!("v1 (still usable after assignment): {:?}", v1);
    println!("v2: {:?}", v2);
}

// ---- 3. PartialEq, Eq, PartialOrd, Ord ---------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// C# analogy: implementing IEquatable<T> and IComparable<T>

fn derive_eq_ord() {
    println!("\n--- #[derive(PartialEq, Eq, PartialOrd, Ord)] ---");

    let v1 = Version { major: 1, minor: 2, patch: 3 };
    let v2 = Version { major: 1, minor: 3, patch: 0 };
    let v3 = v1.clone();

    println!("v1 == v3: {}", v1 == v3);
    println!("v1 != v2: {}", v1 != v2);
    println!("v1 < v2:  {}", v1 < v2);
    println!("v2 > v1:  {}", v2 > v1);

    // Derived Ord means we can sort:
    let mut versions = vec![
        Version { major: 2, minor: 0, patch: 0 },
        Version { major: 1, minor: 0, patch: 0 },
        Version { major: 1, minor: 2, patch: 3 },
    ];
    versions.sort();
    println!("sorted: {:?}", versions);
}

// ---- 4. Default -----------------------------------------------------

#[derive(Debug, Default)]      // C# analogy: default parameter values / default(T)
struct ServerConfig {
    host: String,             // default: ""
    port: u16,                // default: 0
    max_connections: usize,   // default: 0
    tls_enabled: bool,        // default: false
}

// Custom Default (when derived doesn't give the right defaults):
#[derive(Debug)]
struct HttpConfig {
    host: String,
    port: u16,
    timeout_secs: u64,
}

impl Default for HttpConfig {
    fn default() -> Self {
        HttpConfig {
            host: "localhost".to_string(),
            port: 8080,
            timeout_secs: 30,
        }
    }
}

fn derive_default() {
    println!("\n--- #[derive(Default)] ---");

    let s = ServerConfig::default();
    println!("ServerConfig default: {:?}", s);

    // Builder pattern often uses ..Default::default():
    let custom = ServerConfig {
        host: "example.com".to_string(),
        port: 443,
        ..Default::default()
    };
    println!("ServerConfig custom: {:?}", custom);

    let h = HttpConfig::default();
    println!("HttpConfig default: {:?}", h);
}

// ---- 5. Hash --------------------------------------------------------

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]  // Hash enables use as HashMap key
struct UserId(u64);                            // C# analogy: GetHashCode + Equals

fn derive_hash() {
    println!("\n--- #[derive(Hash)] ---");

    let mut users: HashMap<UserId, &str> = HashMap::new();
    users.insert(UserId(1), "Alice");
    users.insert(UserId(2), "Bob");

    println!("user 1: {:?}", users.get(&UserId(1)));
    println!("user 2: {:?}", users.get(&UserId(2)));
}

// ---- 6. Attributes overview ----------------------------------------

// Attributes annotate items — C# analogy: [Attribute] decoration.
// Three syntax forms:
//   #[attr]          — applies to the next item
//   #![attr]         — inner attribute (applies to enclosing item)
//   #[attr = "val"]  — with value
//   #[attr(args)]    — with arguments

// Suppress specific warnings:
#[allow(dead_code)]
fn unused_function() {}

// Make compiler warn when this is not used:
#[must_use]
fn important_result() -> i32 { 42 }

// Deprecated with migration hint:
#[deprecated(since = "1.0.0", note = "use new_fn() instead")]
fn old_fn() {}

// Conditional compilation — only compiled on specific platforms:
#[cfg(target_os = "windows")]
fn platform_specific() { println!("Windows-specific code"); }

#[cfg(not(target_os = "windows"))]
fn platform_specific() { println!("non-Windows code"); }

// Documentation attribute — same as /// comment:
#[doc = "This function does something important."]
fn documented_fn() {}

fn attributes_overview() {
    println!("\n--- Attributes Overview ---");

    let _r = important_result(); // Using it avoids #[must_use] warning
    platform_specific();

    #[allow(deprecated)]
    old_fn();

    println!(r#"
Common attributes:
  #[derive(...)]         — auto-implement traits
  #[allow/warn/deny]     — lint control
  #[must_use]            — warn if return value ignored
  #[deprecated]          — mark as deprecated
  #[cfg(...)]            — conditional compilation
  #[test]                — mark a test function
  #[tokio::test]         — async test with Tokio
  #[inline] / #[cold]    — optimization hints
  #[repr(C/u8/transparent)] — memory layout control
  #[doc = "..."]         — documentation
"#);
}

// ---- 7. cfg attributes in depth ------------------------------------

fn cfg_attributes() {
    println!("--- #[cfg(...)] Conditions ---");

    // cfg can check: target_os, target_arch, feature, debug_assertions, etc.

    if cfg!(debug_assertions) {
        println!("  running in debug mode");
    } else {
        println!("  running in release mode");
    }

    if cfg!(target_os = "windows") {
        println!("  Windows target");
    } else if cfg!(target_os = "linux") {
        println!("  Linux target");
    } else if cfg!(target_os = "macos") {
        println!("  macOS target");
    }

    // target_pointer_width — 32 or 64 bit:
    if cfg!(target_pointer_width = "64") {
        println!("  64-bit pointer width (usize = 8 bytes)");
    }

    println!(r#"
cfg conditions:
  target_os       = "windows" | "linux" | "macos" | "android" | ...
  target_arch     = "x86_64" | "aarch64" | "wasm32" | ...
  target_env      = "msvc" | "gnu" | "musl"
  debug_assertions               (true in debug, false in release)
  feature = "my-feature"         (Cargo feature flags)
  test                           (true when running cargo test)
  doc                            (true when building documentation)
"#);
}

// ---- Derivable traits quick reference -----------------------------
//
// Trait           | C# Equivalent               | Notes
// ----------------|-----------------------------|---------------------
// Debug           | ToString / DebuggerDisplay  | {:?} formatting
// Display         | ToString                    | {} formatting, manual
// Clone           | ICloneable                  | .clone()
// Copy            | value type semantics        | implicit copy
// PartialEq/Eq    | IEquatable<T> + Equals      | == and !=
// PartialOrd/Ord  | IComparable<T>              | <, >, <=, >=
// Hash            | GetHashCode                 | HashMap key
// Default         | default(T)                  | T::default()
// From/Into       | implicit/explicit casts     | type conversion
// Error           | Exception                   | thiserror derive
// Serialize       | [JsonSerializable]          | serde derive
// Deserialize     | JsonSerializer.Deserialize  | serde derive

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_ordering() {
        let v1 = Version { major: 1, minor: 0, patch: 0 };
        let v2 = Version { major: 2, minor: 0, patch: 0 };
        assert!(v1 < v2);
    }

    #[test]
    fn config_default() {
        let c = ServerConfig::default();
        assert_eq!(c.port, 0);
        assert!(!c.tls_enabled);
    }

    #[test]
    fn user_id_as_key() {
        let mut m = std::collections::HashMap::new();
        m.insert(UserId(42), "Alice");
        assert_eq!(m[&UserId(42)], "Alice");
    }
}
