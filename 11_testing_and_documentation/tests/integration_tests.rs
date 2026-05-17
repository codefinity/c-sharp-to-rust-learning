// Integration tests — compiled as separate crate, test the public API only.
// Like a C# test project that references your library.
// Place in tests/ directory — automatically discovered by `cargo test`.

use testing_and_documentation::{factorial, is_prime, Stack};

#[test]
fn factorial_integration() {
    assert_eq!(factorial(10), 3_628_800);
}

#[test]
fn prime_sieve_integration() {
    let primes: Vec<u64> = (2..50).filter(|&n| is_prime(n)).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]);
}

#[test]
fn stack_full_lifecycle() {
    let mut stack = Stack::new();

    // Fill
    for i in 0..10 { stack.push(i); }
    assert_eq!(stack.len(), 10);

    // Drain
    let mut count = 9;
    while let Some(top) = stack.pop() {
        assert_eq!(top, count);
        if count == 0 { break; }
        count -= 1;
    }
    assert!(stack.is_empty());
}
