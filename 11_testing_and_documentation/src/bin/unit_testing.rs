// ============================================================
// CONCEPT: Testing in Rust — Unit, Integration, Patterns
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# testing: separate test project, [TestClass]/[TestMethod] (MSTest),
// [Fact]/[Theory] (xUnit), [Test] (NUnit). Tests live in a different assembly.
//
// Rust testing: tests live IN the same file as the code (unit tests)
// or in a tests/ directory (integration tests). No separate project.
//
//   #[test]           ← [Fact] in xUnit / [Test] in NUnit
//   #[should_panic]   ← Assert.Throws<>() / [ExpectedException]
//   #[ignore]         ← [Ignore] / Skip attribute
//
// Run with: cargo test
// Run specific: cargo test <name_filter>
// Show output:  cargo test -- --nocapture
// Run ignored:  cargo test -- --include-ignored
//
// This file demonstrates patterns. The actual tests are in the #[cfg(test)]
// block. Run with: cargo test --bin unit_testing
// ============================================================

fn main() {
    println!("=== Testing in Rust ===\n");
    println!("Run the tests with: cargo test --bin unit_testing");
    println!("Run with output:    cargo test --bin unit_testing -- --nocapture\n");

    demo_test_organisation();
    demo_assert_macros();
    demo_cargo_test_flags();
}

// ─── PRODUCTION CODE — the functions we will test ───────────────────────────

pub fn add(a: i32, b: i32) -> i32 { a + b }

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { return Err("division by zero".to_string()); }
    Ok(a / b)
}

pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    chars == chars.iter().rev().cloned().collect::<Vec<_>>()
}

pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShoppingCart {
    items: Vec<(String, f64)>,
    discount_pct: u8,
}

impl ShoppingCart {
    pub fn new() -> Self { ShoppingCart { items: vec![], discount_pct: 0 } }
    pub fn new_with_discount(pct: u8) -> Self { ShoppingCart { items: vec![], discount_pct: pct } }

    pub fn add_item(&mut self, name: &str, price: f64) {
        if price < 0.0 { panic!("price cannot be negative: {price}"); }
        self.items.push((name.to_string(), price));
    }

    pub fn total(&self) -> f64 {
        let subtotal: f64 = self.items.iter().map(|(_, p)| p).sum();
        subtotal * (1.0 - self.discount_pct as f64 / 100.0)
    }

    pub fn item_count(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for ShoppingCart {
    fn default() -> Self { Self::new() }
}

// ─── DEMO OUTPUT ────────────────────────────────────────────────────────────

fn demo_test_organisation() {
    println!("--- Test Organisation ---\n");
    println!(r#"  Rust test structure (vs C#):

  UNIT TESTS — inside the source file:
  ┌─────────────────────────────────────────────────────┐
  │  // production code                                 │
  │  pub fn add(a: i32, b: i32) -> i32 {{ a + b }}     │
  │                                                     │
  │  #[cfg(test)]          ← only compiled for tests    │
  │  mod tests {{                                       │
  │      use super::*;     ← import everything above    │
  │                                                     │
  │      #[test]           ← marks a test function      │
  │      fn add_works() {{                              │
  │          assert_eq!(add(2, 3), 5);                  │
  │      }}                                             │
  │  }}                                                 │
  └─────────────────────────────────────────────────────┘

  INTEGRATION TESTS — in tests/ directory:
  ┌─────────────────────────────────────────────────────┐
  │  // tests/my_integration_test.rs                    │
  │  use my_crate::MyStruct;     // imports the crate   │
  │                                                     │
  │  #[test]                                            │
  │  fn whole_feature_works() {{ ... }}                 │
  └─────────────────────────────────────────────────────┘

  C#                               Rust
  ─────────────────────────────────────────────────────────────────
  Separate test project (.Tests)   #[cfg(test)] mod inside same file
  [TestClass] on a class           mod tests {{ }} block
  [TestMethod] / [Fact]            #[test] fn
  [SetUp] / constructor            fn setup() called manually, or fixtures
  [TearDown]                       Drop trait on test fixture struct
  Assert.AreEqual(expected, actual) assert_eq!(actual, expected)
  Assert.IsTrue(condition)         assert!(condition)
  Assert.Throws<T>()               #[should_panic(expected = "msg")]
  [Ignore("reason")]               #[ignore = "reason"]
  [TestCase(1), TestCase(2)]       parameterised via loops or proptest
"#);
}

fn demo_assert_macros() {
    println!("--- Assert Macros ---\n");
    println!(r#"  Rust assert macros:
  ─────────────────────────────────────────────────────────────────
  assert!(expr)                    panics if expr is false
  assert!(expr, "msg {{val}}", val)  panics with formatted message
  assert_eq!(left, right)          panics if left != right
  assert_eq!(left, right, "msg")   same, with message
  assert_ne!(left, right)          panics if left == right
  assert!(val > 0.0)               for ordering / range checks

  C# equivalent:
  Assert.IsTrue(expr)              assert!(expr)
  Assert.AreEqual(a, b)            assert_eq!(a, b)
  Assert.AreNotEqual(a, b)         assert_ne!(a, b)
  Assert.IsNull(val)               assert!(val.is_none())
  Assert.IsNotNull(val)            assert!(val.is_some())
  Assert.ThrowsException<T>(...)   #[should_panic] or catch_unwind

  Useful for float comparisons:
  assert!((a - b).abs() < 1e-6);  // because f64 == f64 is exact
"#);
}

fn demo_cargo_test_flags() {
    println!("--- cargo test Flags ---\n");
    println!(r#"  cargo test                         run all tests
  cargo test <filter>                run tests whose name contains <filter>
  cargo test -- --nocapture          show println! output during tests
  cargo test -- --include-ignored    run #[ignore]d tests too
  cargo test -- --test-threads=1     run tests sequentially (not parallel)
  cargo test --bin unit_testing      only this binary's tests
  cargo test -p testing_and_doc...   only this package's tests
  cargo test --workspace             all tests in all packages
  cargo test -- --list               list test names without running

  C# equivalent:
  dotnet test                        cargo test --workspace
  dotnet test --filter Name~Foo      cargo test foo
  dotnet test -- --no-parallel       cargo test -- --test-threads=1
"#);
}

// ─── TESTS ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Basic assertions ───────────────────────────────────────────────────

    #[test]
    fn add_two_positives() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn add_negative_numbers() {
        assert_eq!(add(-1, -2), -3);
    }

    #[test]
    fn add_with_zero() {
        assert_eq!(add(0, 42), 42);
        assert_eq!(add(42, 0), 42);
    }

    // ── Result-returning functions ──────────────────────────────────────────

    #[test]
    fn divide_normal() {
        let result = divide(10.0, 4.0).unwrap();
        assert!((result - 2.5).abs() < 1e-10, "expected 2.5, got {result}");
    }

    #[test]
    fn divide_by_zero_returns_err() {
        let result = divide(5.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "division by zero");
    }

    // ── should_panic — like Assert.Throws in xUnit ─────────────────────────

    #[test]
    #[should_panic(expected = "price cannot be negative")]
    fn add_negative_price_panics() {
        let mut cart = ShoppingCart::new();
        cart.add_item("bad item", -5.0);   // must panic with the expected message
    }

    #[test]
    #[should_panic]  // any panic is acceptable — don't check the message
    fn fibonacci_overflow_panics_in_debug() {
        // fibonacci(100) overflows u64 in debug mode (overflow panics):
        fibonacci(94);  // 94th fib = 19_740_274_219_868_223_167 — fits in u64
        fibonacci(95);  // 95th fib overflows u64
    }

    // ── Custom structs ─────────────────────────────────────────────────────

    #[test]
    fn cart_empty_on_creation() {
        let cart = ShoppingCart::new();
        assert!(cart.is_empty());
        assert_eq!(cart.item_count(), 0);
        assert_eq!(cart.total(), 0.0);
    }

    #[test]
    fn cart_totals_correctly() {
        let mut cart = ShoppingCart::new();
        cart.add_item("Book", 25.0);
        cart.add_item("Pen", 5.0);
        assert_eq!(cart.item_count(), 2);
        assert!((cart.total() - 30.0).abs() < 1e-10);
    }

    #[test]
    fn cart_applies_discount() {
        let mut cart = ShoppingCart::new_with_discount(10);
        cart.add_item("Item", 100.0);
        assert!((cart.total() - 90.0).abs() < 1e-10);
    }

    // ── String / bool functions ────────────────────────────────────────────

    #[test]
    fn palindrome_detection() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(is_palindrome("a"));
        assert!(is_palindrome(""));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
    }

    // ── ignore — like [Ignore] or Skip in xUnit ────────────────────────────

    #[test]
    #[ignore = "slow: takes ~10s — run with --include-ignored"]
    fn fibonacci_large_value() {
        assert_eq!(fibonacci(30), 832_040);
        assert_eq!(fibonacci(40), 102_334_155);
    }

    // ── Helper / setup pattern (no framework magic — just functions) ────────

    fn make_stocked_cart() -> ShoppingCart {
        let mut cart = ShoppingCart::new();
        cart.add_item("Apple",  1.50);
        cart.add_item("Bread",  2.50);
        cart.add_item("Coffee", 8.00);
        cart
    }

    #[test]
    fn stocked_cart_has_three_items() {
        let cart = make_stocked_cart();
        assert_eq!(cart.item_count(), 3);
    }

    #[test]
    fn stocked_cart_total_is_correct() {
        let cart = make_stocked_cart();
        assert!((cart.total() - 12.0).abs() < 1e-10);
    }

    // ── assert_ne! ─────────────────────────────────────────────────────────

    #[test]
    fn different_inputs_give_different_outputs() {
        assert_ne!(add(1, 2), add(1, 3));
        assert_ne!(fibonacci(5), fibonacci(6));
    }

    // ── Testing private functions (possible because tests are in same module) ─

    // In C# you need InternalsVisibleTo or reflection. In Rust:
    // #[cfg(test)] mod tests uses super::* which includes private items.
    fn private_helper(x: i32) -> i32 { x * 2 }  // not pub

    #[test]
    fn can_test_private_function() {
        assert_eq!(private_helper(5), 10);  // works — same module
    }
}
