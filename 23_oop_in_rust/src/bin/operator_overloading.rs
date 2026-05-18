// ============================================================
// OOP PILLAR 4: Abstraction — Operator Overloading, Display & Drop
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# operator overloading and interface implementations:
//   operator +, -, *, /      → std::ops::{Add, Sub, Mul, Div, Neg}
//   IComparable<T>           → std::cmp::{PartialOrd, Ord}
//   IEquatable<T>            → std::cmp::{PartialEq, Eq}
//   ToString() override      → std::fmt::Display
//   IDisposable / using      → std::ops::Drop
//   this[int index]          → std::ops::Index / IndexMut
//   implicit/explicit cast   → std::convert::{From, Into}
//
// In Rust, ALL of these are just regular trait implementations.
// There is no special syntax for operators beyond the trait impls.
//
// RUN: cargo run --bin operator_overloading
// ============================================================

use std::fmt;
use std::ops::{Add, Sub, Mul, Neg, Index};

fn main() {
    println!("=== OOP Pillar 4: Operator Overloading & Special Traits ===\n");

    demo_arithmetic_operators();
    demo_equality_ordering();
    demo_display_debug();
    demo_drop_idisposable();
    demo_index_operator();
    demo_from_into_conversions();
}

// ─── 1. ARITHMETIC OPERATORS ─────────────────────────────────────────────────
//
// C#:
//   public record Vector2(double X, double Y) {
//       public static Vector2 operator +(Vector2 a, Vector2 b) => new(a.X + b.X, a.Y + b.Y);
//       public static Vector2 operator *(Vector2 v, double s) => new(v.X * s, v.Y * s);
//       public static Vector2 operator -(Vector2 v)           => new(-v.X, -v.Y);
//   }
//
// Rust: implement std::ops traits. Each operator maps to exactly one trait.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self { Vector2 { x, y } }
    fn magnitude(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(&self, other: &Vector2) -> f64 { self.x * other.x + self.y * other.y }
}

// operator + → Add trait:
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// operator - (binary) → Sub trait:
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// operator * (scalar) → Mul<f64> trait:
impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

// operator - (unary) → Neg trait:
impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

fn demo_arithmetic_operators() {
    println!("--- 1. Arithmetic Operators ---");
    println!();

    let a = Vector2::new(3.0, 4.0);
    let b = Vector2::new(1.0, 2.0);

    println!("  a            = {:?}", a);
    println!("  b            = {:?}", b);
    println!("  a + b        = {:?}", a + b);
    println!("  a - b        = {:?}", a - b);
    println!("  a * 2.0      = {:?}", a * 2.0);
    println!("  -a           = {:?}", -a);
    println!("  |a|          = {:.2}", a.magnitude());
    println!("  a · b        = {:.2}", a.dot(&b));

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  operator +(Vector2 a, Vector2 b)      impl Add for Vector2 {{ fn add(self, rhs) }}
  operator *(Vector2 v, double s)       impl Mul<f64> for Vector2 {{ fn mul(self, s) }}
  operator -(Vector2 v)  // unary       impl Neg for Vector2 {{ fn neg(self) }}
  operator -(Vector2 a, Vector2 b)      impl Sub for Vector2 {{ fn sub(self, rhs) }}

  Operator     C# attribute    Rust trait
  ───────────────────────────────────────
  +            operator +      std::ops::Add
  -            operator -      std::ops::Sub / Neg
  *            operator *      std::ops::Mul
  /            operator /      std::ops::Div
  %            operator %      std::ops::Rem
  +=           (auto)          std::ops::AddAssign
  &, |, ^      operator &...   std::ops::BitAnd, BitOr, BitXor
"#);
}

// ─── 2. EQUALITY AND ORDERING ────────────────────────────────────────────────
//
// C#:
//   public class Temperature : IEquatable<Temperature>, IComparable<Temperature> {
//       public bool   Equals(Temperature? other) => other is not null && Celsius == other.Celsius;
//       public int    CompareTo(Temperature? other) => Celsius.CompareTo(other?.Celsius ?? 0);
//       public static bool operator ==(Temperature a, Temperature b) => a.Equals(b);
//       public static bool operator < (Temperature a, Temperature b) => a.CompareTo(b) < 0;
//       // ... ==, !=, <, >, <=, >=
//   }
//
// Rust: four composable traits. #[derive] handles the boilerplate.
//
//   PartialEq  — == and !=  (allows NaN-style "not equal to itself")
//   Eq         — marker: == is reflexive (all values equal themselves)
//   PartialOrd — <, >, <=, >=  (allows partial order, e.g., floats with NaN)
//   Ord        — total order (every pair is comparable)

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Celsius(f64);

// For a custom type where we want derive to not apply, implement manually:
#[derive(Debug, Clone)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn new(major: u32, minor: u32, patch: u32) -> Self { Version { major, minor, patch } }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}  // marker — asserts == is reflexive

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare major, then minor, then patch:
        self.major.cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

fn demo_equality_ordering() {
    println!("--- 2. Equality and Ordering ---");
    println!();

    // Celsius — derived PartialEq and PartialOrd (f64 is PartialOrd, not Ord):
    let boiling  = Celsius(100.0);
    let freezing = Celsius(0.0);
    let warm     = Celsius(37.0);

    println!("  boiling == boiling: {}", boiling == boiling);
    println!("  boiling >  warm:    {}", boiling > warm);
    println!("  freezing < warm:    {}", freezing < warm);

    // Version — manual Ord for lexicographic semver comparison:
    let v1 = Version::new(1, 2, 3);
    let v2 = Version::new(1, 3, 0);
    let v3 = Version::new(2, 0, 0);

    let mut versions = vec![v3.clone(), v1.clone(), v2.clone()];
    versions.sort();  // uses Ord
    for v in &versions {
        println!("  v{}.{}.{}", v.major, v.minor, v.patch);
    }

    println!("  v1 < v2: {}", v1 < v2);
    println!("  v2 < v3: {}", v2 < v3);

    println!(r#"
  C#                             Rust             Notes
  ─────────────────────────────────────────────────────────────────
  IEquatable<T>.Equals()         PartialEq        == and !=
  (no separate marker)           Eq               Asserts reflexivity (x == x always)
  IComparable<T>.CompareTo()     PartialOrd       <, >, <=, >=  (allows incomparables)
  (no separate marker)           Ord              Total order — needed for sort()
  operator == / != / < / >...    (auto from traits)

  #[derive(PartialEq, Eq, PartialOrd, Ord)] generates all of the above
  for structs whose fields already implement those traits.
"#);
}

// ─── 3. DISPLAY AND DEBUG ────────────────────────────────────────────────────
//
// C#:
//   public override string ToString() => $"({X}, {Y})";
//
// Rust has TWO formatting traits:
//   fmt::Display — human-readable output   → {} in format strings
//   fmt::Debug   — developer/diagnostic    → {:?} in format strings
//
// #[derive(Debug)] gives you {:?} for free.
// You implement Display yourself for the {} representation.

#[derive(Debug, Clone, Copy)]  // Debug gives us {:?} for free
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self { Complex { real, imag } }
    fn magnitude(&self) -> f64 { (self.real * self.real + self.imag * self.imag).sqrt() }
}

// fmt::Display — like ToString() override in C#:
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{:.2}+{:.2}i", self.real, self.imag)
        } else {
            write!(f, "{:.2}{:.2}i", self.real, self.imag)
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

fn demo_display_debug() {
    println!("--- 3. Display (ToString) and Debug ---");
    println!();

    let c1 = Complex::new(3.0, 4.0);
    let c2 = Complex::new(1.0, -2.0);

    println!("  Display  c1 = {c1}");           // uses fmt::Display → ToString equiv
    println!("  Display  c2 = {c2}");
    println!("  Debug    c1 = {c1:?}");          // uses fmt::Debug
    println!("  Debug    c1 = {c1:#?}");         // pretty-printed debug
    println!("  c1 + c2  = {}", c1 + c2);
    println!("  |c1|     = {:.2}", c1.magnitude());

    // to_string() is automatically available when Display is implemented:
    let s: String = c1.to_string();
    println!("  to_string() = \"{s}\"");

    println!(r#"
  C#                              Rust
  ────────────────────────────────────────────────────────────────────
  override string ToString()      impl fmt::Display for T  (enables {{}})
  [DebuggerDisplay("...")]        impl fmt::Debug for T    (enables {{:?}})
  #[derive(Debug)]                #[derive(Debug)]         (auto {{:?}})
  obj.ToString()                  obj.to_string()          (auto from Display)
  $"val = {{c}}"                  format!("val = {{c}}")
  Console.WriteLine(c)            println!("{{c}}")
"#);
}

// ─── 4. DROP (IDisposable / using) ───────────────────────────────────────────
//
// C#:
//   class Resource : IDisposable {
//       public void Dispose() { /* release native handle */ }
//   }
//   using var r = new Resource();   // Dispose() called at end of scope
//   // or: using (var r = new Resource()) { ... }
//
// Rust: implement Drop. The destructor is called AUTOMATICALLY when the value
// goes out of scope. There is no equivalent to forgetting to call Dispose() —
// the compiler guarantees Drop runs (unless you call std::mem::forget).

struct DatabaseConnection {
    url: String,
    connection_id: u32,
}

impl DatabaseConnection {
    fn new(url: &str, id: u32) -> Self {
        println!("  [DB] Opened connection #{id} to {url}");
        DatabaseConnection { url: url.to_string(), connection_id: id }
    }

    fn query(&self, sql: &str) -> String {
        format!("results of '{sql}' from conn #{}", self.connection_id)
    }
}

// Drop is called automatically when the value leaves scope — like `using` in C#:
impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("  [DB] Closed connection #{} to {}", self.connection_id, self.url);
    }
}

fn demo_drop_idisposable() {
    println!("--- 4. Drop (IDisposable / using statement) ---");
    println!();

    {
        let conn = DatabaseConnection::new("postgres://localhost/mydb", 42);
        let result = conn.query("SELECT * FROM users");
        println!("  Query result: {result}");
        // conn goes out of scope here → Drop::drop() is called automatically
    }
    println!("  (connection was automatically closed)");

    // Explicit early drop — like calling Dispose() before end of scope:
    let conn2 = DatabaseConnection::new("redis://localhost", 99);
    println!("  Using conn2 briefly...");
    drop(conn2);   // explicit drop — calls Drop::drop() now
    println!("  (conn2 already closed, compiler prevents further use)");

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  class R : IDisposable                 struct R {{ ... }}
  void Dispose() {{ ... }}              impl Drop for R {{ fn drop(&mut self) {{ ... }} }}
  using var r = new R();                let r = R::new();  // drop is automatic
  using (var r = new R()) {{ ... }}     {{ let r = R::new(); ... }}  // scope = block
  r.Dispose()  // manual               drop(r)  // explicit early drop
  // Can forget to call Dispose!        // Impossible to forget — compiler guarantees it
"#);
}

// ─── 5. INDEX OPERATOR ───────────────────────────────────────────────────────
//
// C#:
//   class Matrix {
//       public double this[int row, int col] { get => ...; set => ...; }
//   }
//
// Rust: implement Index (read) and IndexMut (read+write).

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![vec![0.0; cols]; rows], rows, cols }
    }
}

// Read-only indexing — matrix[row][col]:
impl Index<usize> for Matrix {
    type Output = Vec<f64>;
    fn index(&self, row: usize) -> &Vec<f64> {
        assert!(row < self.rows, "row index out of bounds");
        &self.data[row]
    }
}

fn demo_index_operator() {
    println!("--- 5. Index Operator ---");
    println!();

    let mut m = Matrix::new(3, 3);
    // Assign via nested Vec indexing:
    m.data[0][0] = 1.0; m.data[1][1] = 2.0; m.data[2][2] = 3.0;

    println!("  m[0][0] = {}", m[0][0]);  // uses our Index impl
    println!("  m[1][1] = {}", m[1][1]);
    println!("  m[2][2] = {}", m[2][2]);

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  double this[int r, int c] {{ get }}   impl Index<usize> for Matrix
  double this[int r, int c] {{ set }}   impl IndexMut<usize> for Matrix
  matrix[0, 1]                          matrix[0][1]  (if Output = Vec<f64>)
"#);
}

// ─── 6. FROM / INTO CONVERSIONS ──────────────────────────────────────────────
//
// C#:
//   public static implicit operator Celsius(double d) => new Celsius(d);
//   public static explicit operator Fahrenheit(Celsius c) => new Fahrenheit(c.Value * 9/5 + 32);
//   double d = someTemp;          // implicit conversion
//   Fahrenheit f = (Fahrenheit)c; // explicit cast
//
// Rust: From<T> and Into<T>. Implement From — Into is derived automatically.
//   From = infallible conversion  (like implicit cast)
//   TryFrom = fallible conversion (like explicit cast that can fail)

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Fahrenheit {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Celsius {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl fmt::Display for Celsius    { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.1}°C", self.0) } }
impl fmt::Display for Fahrenheit { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.1}°F", self.0) } }

fn demo_from_into_conversions() {
    println!("--- 6. From / Into (Implicit/Explicit Casts) ---");
    println!();

    let boiling_c = Celsius(100.0);

    // From:
    let boiling_f = Fahrenheit::from(boiling_c);
    println!("  Fahrenheit::from({boiling_c}) = {boiling_f}");

    // Into — automatically derived from From:
    let freezing_c = Celsius(0.0);
    let freezing_f: Fahrenheit = freezing_c.into();  // turbofish or type annotation needed
    println!("  Celsius(0).into()  = {freezing_f}");

    // Round-trip:
    let body_f = Fahrenheit(98.6);
    let body_c: Celsius = body_f.into();
    println!("  {body_f} → {body_c}");

    // String → owned String is From<&str>:
    let s: String = String::from("hello");
    let s2: String = "world".into();
    println!("  String::from(\"hello\") = {s}");
    println!("  \"world\".into()       = {s2}");

    println!(r#"
  C#                                    Rust
  ──────────────────────────────────────────────────────────────────
  implicit operator Fahrenheit(Celsius) impl From<Celsius> for Fahrenheit
  Fahrenheit f = celsius;               let f: Fahrenheit = celsius.into();
  explicit operator Celsius(Fahrenheit) impl TryFrom<Fahrenheit> for Celsius
  Celsius c = (Celsius)fahrenheit;      let c = Celsius::try_from(f)?;
"#);
}

// ─── COMPLETE OPERATOR REFERENCE ─────────────────────────────────────────────

fn _operator_reference() {
    println!(r#"
Complete C# → Rust Operator Map
────────────────────────────────────────────────────────────────────────
C# operator/interface          Rust trait                 Trait method
────────────────────────────────────────────────────────────────────────
operator +                     std::ops::Add              add(self, rhs)
operator - (binary)            std::ops::Sub              sub(self, rhs)
operator * (binary)            std::ops::Mul              mul(self, rhs)
operator /                     std::ops::Div              div(self, rhs)
operator %                     std::ops::Rem              rem(self, rhs)
operator - (unary)             std::ops::Neg              neg(self)
operator !                     std::ops::Not              not(self)
operator &                     std::ops::BitAnd           bitand(self, rhs)
operator |                     std::ops::BitOr            bitor(self, rhs)
operator ^                     std::ops::BitXor           bitxor(self, rhs)
operator << / >>               std::ops::Shl / Shr        shl / shr
operator +=                    std::ops::AddAssign        add_assign(&mut self, rhs)
this[int i] get                std::ops::Index            index(&self, idx)
this[int i] set                std::ops::IndexMut         index_mut(&mut self, idx)
IEquatable<T>.Equals()         std::cmp::PartialEq        eq(&self, other)
IComparable<T>.CompareTo()     std::cmp::PartialOrd/Ord   partial_cmp / cmp
ToString()                     std::fmt::Display          fmt(&self, f)
[DebuggerDisplay]              std::fmt::Debug            fmt(&self, f)
IDisposable.Dispose()          std::ops::Drop             drop(&mut self)
implicit operator              std::convert::From         from(val)
explicit operator              std::convert::TryFrom      try_from(val)
GetHashCode()                  std::hash::Hash            hash(&self, state)
────────────────────────────────────────────────────────────────────────
"#);
}

// ─── TESTS ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(a + b, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn vector2_neg() {
        let a = Vector2::new(3.0, -4.0);
        assert_eq!(-a, Vector2::new(-3.0, 4.0));
    }

    #[test]
    fn complex_display() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.to_string(), "3.00+4.00i");
    }

    #[test]
    fn complex_negative_imag_display() {
        let c = Complex::new(1.0, -2.0);
        assert_eq!(c.to_string(), "1.00-2.00i");
    }

    #[test]
    fn version_ordering() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);
        let v3 = Version::new(2, 0, 0);
        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn celsius_to_fahrenheit() {
        let c = Celsius(100.0);
        let f = Fahrenheit::from(c);
        assert!((f.0 - 212.0).abs() < 0.001);
    }

    #[test]
    fn fahrenheit_to_celsius() {
        let f = Fahrenheit(32.0);
        let c: Celsius = f.into();
        assert!(c.0.abs() < 0.001);
    }

    #[test]
    fn matrix_index() {
        let mut m = Matrix::new(2, 2);
        m.data[0][1] = 7.0;
        assert!((m[0][1] - 7.0).abs() < 0.001);
    }
}
