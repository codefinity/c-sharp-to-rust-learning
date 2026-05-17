// ============================================================
// CONCEPT: Advanced Traits — operator overloading, From/Into,
//          Deref, Index, and newtype with traits
// ============================================================
//
// RUN: cargo run --bin advanced_traits
// ============================================================

use std::ops::{Add, Mul, Neg, Index};
use std::fmt;

fn main() {
    operator_overloading();
    from_into_conversions();
    deref_coercion();
    index_trait();
    iterator_trait();
}

// ─── OPERATOR OVERLOADING ────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Self { x, y } }
    fn dot(&self, other: &Vec2) -> f64 { self.x * other.x + self.y * other.y }
    fn magnitude(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

fn operator_overloading() {
    println!("=== Operator Overloading ===");

    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);

    println!("a = {a}  b = {b}");
    println!("a + b = {}", a + b);
    println!("a * 2 = {}", a * 2.0);
    println!("-a = {}", -a);
    println!("a.dot(b) = {}", a.dot(&b));
    println!("b.magnitude = {:.4}", b.magnitude());
}

// ─── FROM/INTO CONVERSIONS ───────────────────────────────────

#[derive(Debug)]
struct Email(String);

#[derive(Debug)]
struct EmailError(String);

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid email: {}", self.0)
    }
}

// TryFrom for fallible conversion (like explicit cast in C#)
impl TryFrom<String> for Email {
    type Error = EmailError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.contains('@') {
            Ok(Email(s))
        } else {
            Err(EmailError(s))
        }
    }
}

// From<&str> for Email — infallible (panics if invalid? No — we validate)
impl From<&str> for Vec2 {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().and_then(|p| p.trim().parse().ok()).unwrap_or(0.0);
        let y = parts.next().and_then(|p| p.trim().parse().ok()).unwrap_or(0.0);
        Vec2::new(x, y)
    }
}

fn from_into_conversions() {
    println!("\n=== From/Into Conversions ===");

    // From<&str> for Vec2 (we implemented it above)
    let v: Vec2 = "1.0, 2.0".into(); // Into::into() calls From::from()
    println!("from string: {v}");

    let v2 = Vec2::from("3.5, -1.5");
    println!("Vec2::from: {v2}");

    // TryFrom for fallible conversion:
    let valid: Result<Email, _> = "user@example.com".to_string().try_into();
    println!("valid email: {valid:?}");

    let invalid: Result<Email, _> = "not-an-email".to_string().try_into();
    println!("invalid email: {invalid:?}");

    // Standard library From impls:
    let s: String = String::from("hello");        // From<&str> for String
    let n: i64    = i64::from(42_i32);            // From<i32> for i64
    let v: Vec<_> = Vec::from([1, 2, 3]);         // From<[T;N]> for Vec<T>
    println!("s={s} n={n} v={v:?}");
}

// ─── DEREF COERCION ──────────────────────────────────────────

fn deref_coercion() {
    println!("\n=== Deref Coercion ===");

    // Deref coercion: &String → &str, &Vec<T> → &[T], &Box<T> → &T
    // This is why functions taking &str accept &String automatically.

    fn count_chars(s: &str) -> usize { s.chars().count() }

    let owned = String::from("hello");
    let boxed = Box::new("world");

    println!("owned: {}", count_chars(&owned)); // &String → &str via Deref
    println!("boxed: {}", count_chars(&boxed)); // &Box<&str> → &&str → &str

    // Manual Deref implementation:
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T { &self.0 }
    }

    let b = MyBox(String::from("Rust"));
    println!("MyBox deref: {}", count_chars(&b)); // MyBox<String> → String → str
}

// ─── INDEX TRAIT ─────────────────────────────────────────────

struct Matrix {
    data: [[f64; 3]; 3],
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        &self.data[row][col]
    }
}

impl Matrix {
    fn identity() -> Self {
        let mut data = [[0.0; 3]; 3];
        data[0][0] = 1.0; data[1][1] = 1.0; data[2][2] = 1.0;
        Self { data }
    }
}

fn index_trait() {
    println!("\n=== Index Trait ===");

    let m = Matrix::identity();
    println!("m[(0,0)] = {}", m[(0, 0)]);
    println!("m[(0,1)] = {}", m[(0, 1)]);
    println!("m[(1,1)] = {}", m[(1, 1)]);
}

// ─── ITERATOR TRAIT ──────────────────────────────────────────

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self { Self { count: 0, max } }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn iterator_trait() {
    println!("\n=== Custom Iterator ===");

    let counter = Counter::new(5);
    // Once Iterator is implemented, ALL iterator adapters work:
    let sum: u32 = counter.sum();
    println!("sum 1..=5: {sum}");

    let doubled: Vec<u32> = Counter::new(5).map(|x| x * 2).collect();
    println!("doubled: {doubled:?}");

    let pairs: Vec<(u32, u32)> = Counter::new(3)
        .zip(Counter::new(3).skip(1))
        .collect();
    println!("pairs: {pairs:?}");

    // Advanced: implementing iterator adapters by hand:
    let evens: Vec<u32> = Counter::new(10).filter(|&x| x % 2 == 0).collect();
    println!("evens 1-10: {evens:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn email_try_from_valid() {
        let e: Result<Email, _> = "a@b.com".to_string().try_into();
        assert!(e.is_ok());
    }

    #[test]
    fn counter_sum() {
        assert_eq!(Counter::new(5).sum::<u32>(), 15);
    }

    #[test]
    fn matrix_index() {
        let m = Matrix::identity();
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 0.0);
    }
}
