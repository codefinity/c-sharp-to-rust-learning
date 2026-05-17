// ============================================================
// CONCEPT: Newtype Pattern
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses `record struct EmailAddress(string Value)` or value objects
// to wrap primitives with semantic meaning. Rust's newtype is a
// zero-cost wrapper: a tuple struct with exactly one field.
//
// Cost: struct Meters(f64) compiles to the same machine code as f64.
// Benefit: you can't accidentally pass a Meters where Kilograms is expected.
//
// RUN: cargo run --bin newtype_pattern
// ============================================================

use std::fmt;
use std::ops::Add;

fn main() {
    println!("=== Newtype Pattern ===\n");

    units_of_measure();
    validated_newtypes();
    impl_std_traits();
    newtype_iterator();
}

// ---- 1. Units of measure (prevent unit confusion) ------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilograms(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Seconds(f64);

impl Meters {
    fn value(self) -> f64 { self.0 }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters { Meters(self.0 + rhs.0) }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

fn calculate_speed(distance: Meters, time: Seconds) -> f64 {
    distance.value() / time.0
}

fn units_of_measure() {
    println!("--- Units of Measure ---");

    let d = Meters(100.0);
    let t = Seconds(9.58);
    let speed = calculate_speed(d, t);
    println!("Bolt's speed: {:.2} m/s", speed);

    let total = Meters(50.0) + Meters(75.0);
    println!("total distance: {total}");

    // This would be a COMPILE ERROR — type mismatch:
    // calculate_speed(Kilograms(100.0), Seconds(9.58));
    println!("(Passing Kilograms where Meters expected is a compile error!)");
}

// ---- 2. Validated newtypes -----------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EmailAddress(String);

#[derive(Debug, thiserror::Error)]
#[error("invalid email: '{0}'")]
struct InvalidEmail(String);

impl EmailAddress {
    fn new(s: impl Into<String>) -> Result<Self, InvalidEmail> {
        let s = s.into();
        if s.contains('@') && s.contains('.') {
            Ok(EmailAddress(s))
        } else {
            Err(InvalidEmail(s))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NonNegativeInt(u32);

impl NonNegativeInt {
    fn new(n: u32) -> Self { NonNegativeInt(n) }
    fn get(self) -> u32 { self.0 }
}

fn validated_newtypes() {
    println!("\n--- Validated Newtypes ---");

    match EmailAddress::new("alice@example.com") {
        Ok(e)  => println!("valid email: {e}"),
        Err(e) => println!("error: {e}"),
    }

    match EmailAddress::new("not-an-email") {
        Ok(e)  => println!("valid email: {e}"),
        Err(e) => println!("error: {e}"),
    }

    let age = NonNegativeInt::new(42);
    println!("age: {}", age.get());
}

// ---- 3. Implementing standard traits on newtypes -------------------

// Wrapper around Vec<i32> — we want a custom Display:
struct IntList(Vec<i32>);

impl fmt::Display for IntList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{v}")?;
        }
        write!(f, "]")
    }
}

// Implement Deref so IntList can be used as &[i32] automatically:
impl std::ops::Deref for IntList {
    type Target = Vec<i32>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<Vec<i32>> for IntList {
    fn from(v: Vec<i32>) -> Self { IntList(v) }
}

fn impl_std_traits() {
    println!("\n--- Implementing Traits on Newtypes ---");

    let list: IntList = vec![1, 2, 3, 4, 5].into();
    println!("IntList Display: {list}");
    println!("len via Deref:   {}", list.len());  // Deref → Vec<i32>::len
    println!("sum via Deref:   {}", list.iter().sum::<i32>());
}

// ---- 4. Newtype wrapping an iterator ------------------------------

// A newtype around an iterator that adds extra behaviour — like
// C#'s yield-based extension methods on IEnumerable<T>.

struct Evens(std::ops::RangeFrom<u64>);

impl Evens {
    fn new() -> Self { Evens(0..) }
}

impl Iterator for Evens {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        // Skip odd numbers:
        let n = self.0.next()?;
        Some(if n % 2 == 0 { n } else { self.0.next()? })
    }
}

fn newtype_iterator() {
    println!("\n--- Newtype Iterator ---");

    let first_five_evens: Vec<u64> = Evens::new().take(5).collect();
    println!("first 5 evens: {first_five_evens:?}");

    let sum: u64 = Evens::new().take(10).sum();
    println!("sum of first 10 evens: {sum}");
}

// ---- Summary -------------------------------------------------------
//
// Pattern                | When to use
// -----------------------|----------------------------------------
// Unit wrapper           | Prevent mixing incompatible units
// Validated wrapper      | Enforce invariants at construction
// Impl Deref             | Transparent access to inner type
// Impl From/Into         | Easy conversion from wrapped type
// Impl Display           | Custom string formatting
// Iterator newtype       | Add lazy transformation to iteration
//
// Cost: always zero — newtype is erased at compile time (same as inner type)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valid() {
        assert!(EmailAddress::new("test@example.com").is_ok());
    }

    #[test]
    fn email_invalid() {
        assert!(EmailAddress::new("notanemail").is_err());
    }

    #[test]
    fn meters_add() {
        assert_eq!(Meters(1.0) + Meters(2.0), Meters(3.0));
    }

    #[test]
    fn evens_iterator() {
        let v: Vec<u64> = Evens::new().take(4).collect();
        assert_eq!(v, vec![0, 2, 4, 6]);
    }
}
