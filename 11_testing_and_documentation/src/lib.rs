// ============================================================
// CONCEPT: Testing and Documentation
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# testing: xUnit/NUnit/MSTest in separate test projects.
// Rust testing: unit tests IN the source file, integration tests in tests/,
//               doc tests IN the documentation comments.
//
// Commands:
//   cargo test                  — run all tests
//   cargo test my_fn            — run tests matching "my_fn"
//   cargo test -- --nocapture   — show stdout in tests
//   cargo doc --open            — generate HTML documentation
//   cargo test --doc            — run doc tests only
//
// ============================================================

/// Returns the factorial of n.
///
/// # Examples
///
/// ```
/// use testing_and_documentation::factorial;
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// Panics if n > 20 (would overflow u64).
pub fn factorial(n: u64) -> u64 {
    assert!(n <= 20, "n={n} would overflow u64");
    (1..=n).product()
}

/// Checks if a number is prime.
///
/// # Examples
///
/// ```
/// use testing_and_documentation::is_prime;
/// assert!(is_prime(7));
/// assert!(!is_prime(4));
/// assert!(!is_prime(1));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let limit = (n as f64).sqrt() as u64 + 1;
    !(3..=limit).step_by(2).any(|i| n % i == 0)
}

/// A simple stack data structure.
///
/// # Examples
///
/// ```
/// use testing_and_documentation::Stack;
/// let mut s = Stack::new();
/// s.push(1);
/// s.push(2);
/// assert_eq!(s.pop(), Some(2));
/// assert_eq!(s.len(), 1);
/// ```
#[derive(Debug, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self { Self { items: Vec::new() } }

    /// Pushes an item onto the stack.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Pops the top item from the stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Returns a reference to the top item without popping it.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Returns the number of items in the stack.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the stack has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ─── UNIT TESTS ───────────────────────────────────────────────
// Unit tests live in the same file as the code they test.
// This gives them access to private internals — unlike C# where tests
// must be in a separate project (or use InternalsVisibleTo).

#[cfg(test)] // this module is compiled only during `cargo test`
mod tests {
    use super::*; // bring all items from parent module into scope

    // ── Basic tests ──────────────────────────────────────────

    #[test]
    fn factorial_zero() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_five() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn factorial_twenty() {
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    #[should_panic(expected = "would overflow")]
    fn factorial_overflow_panics() {
        factorial(21); // should panic
    }

    // ── Tests with setup ─────────────────────────────────────

    fn make_test_stack() -> Stack<i32> {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(3);
        s
    }

    #[test]
    fn stack_push_pop() {
        let mut s = make_test_stack();
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn stack_len() {
        let mut s = make_test_stack();
        assert_eq!(s.len(), 3);
        s.pop();
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn stack_is_empty() {
        let mut s: Stack<i32> = Stack::new();
        assert!(s.is_empty());
        s.push(1);
        assert!(!s.is_empty());
    }

    // ── Parameterised-style tests ─────────────────────────────

    #[test]
    fn primes_up_to_20() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19];
        for &p in &primes {
            assert!(is_prime(p), "{p} should be prime");
        }
    }

    #[test]
    fn non_primes_up_to_20() {
        let non_primes = [0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20];
        for &n in &non_primes {
            assert!(!is_prime(n), "{n} should not be prime");
        }
    }

    // ── Error cases ──────────────────────────────────────────

    #[test]
    fn peek_empty_stack() {
        let s: Stack<i32> = Stack::new();
        assert_eq!(s.peek(), None);
    }

    #[test]
    fn peek_does_not_pop() {
        let mut s = Stack::new();
        s.push(42);
        assert_eq!(s.peek(), Some(&42));
        assert_eq!(s.peek(), Some(&42)); // still there
        assert_eq!(s.len(), 1);
    }
}

// ─── PROPERTY-BASED TESTS ────────────────────────────────────
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        // Test invariants with random inputs
        #[test]
        fn factorial_non_negative(n in 0_u64..=20) {
            let result = factorial(n);
            prop_assert!(result >= 1, "factorial({n}) = {result} should be >= 1");
        }

        #[test]
        fn factorial_monotone(n in 1_u64..=19) {
            prop_assert!(factorial(n + 1) > factorial(n));
        }

        #[test]
        fn stack_len_matches_push_count(count in 0usize..100) {
            let mut s: Stack<i32> = Stack::new();
            for i in 0..count { s.push(i as i32); }
            prop_assert_eq!(s.len(), count);
        }
    }
}
