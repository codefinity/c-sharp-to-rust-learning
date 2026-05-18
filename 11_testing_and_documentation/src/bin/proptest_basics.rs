// ============================================================
// CONCEPT: Property-Based Testing with proptest
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Example-based tests (xUnit [Fact]) test specific cases you think of.
// Property-based tests define RULES that hold for ALL inputs, and the
// framework generates hundreds of random inputs to try to break them.
//
// C# equivalent: FsCheck (port of Haskell's QuickCheck)
// Rust: proptest (most popular) or quickcheck
//
// C# FsCheck:
//   [Property]
//   public Property ReverseIsInvolution(int[] xs) =>
//       xs.Reverse().Reverse().SequenceEqual(xs).ToProperty();
//
// Rust proptest:
//   proptest! {
//       #[test]
//       fn reverse_is_involution(xs: Vec<i32>) {
//           let r: Vec<_> = xs.iter().rev().cloned().collect();
//           let rr: Vec<_> = r.iter().rev().cloned().collect();
//           prop_assert_eq!(xs, rr);
//       }
//   }
//
// Key advantage: when proptest finds a failure, it SHRINKS the input
// to the smallest example that still fails — like FsCheck's shrinking.
//
// RUN: cargo run --bin proptest_basics
// RUN TESTS: cargo test --bin proptest_basics
// ============================================================

fn main() {
    println!("=== Property-Based Testing with proptest ===\n");
    demo_what_is_property_testing();
    demo_shrinking_explanation();
    demo_strategies_overview();
    println!("Run the actual property tests with:");
    println!("  cargo test --bin proptest_basics\n");
}

// ─── PRODUCTION CODE ────────────────────────────────────────────────────────

pub fn sort_and_deduplicate(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v.dedup();
    v
}

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    assert!(min <= max, "min must be <= max");
    value.max(min).min(max)
}

pub fn add_commutative(a: i32, b: i32) -> i32 { a + b }

pub fn encode_decode(s: &str) -> String {
    // Trivial "encoding": reverse the string
    s.chars().rev().collect()
}

pub fn decode(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn safe_divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 { None } else { Some(a / b) }
}

// ─── DEMO OUTPUT ────────────────────────────────────────────────────────────

fn demo_what_is_property_testing() {
    println!("--- What Is Property-Based Testing? ---\n");
    println!(r#"  Example-based (xUnit/NUnit):
    You write specific inputs and expected outputs.
    [Fact] void Add_2_and_3_equals_5() {{ Assert.Equal(5, Add(2, 3)); }}

    PROBLEM: You only test cases you thought of. Edge cases slip through.

  Property-based (proptest/FsCheck):
    You write a RULE that must hold for ALL inputs.
    proptest! {{ fn add_is_commutative(a: i32, b: i32) {{
        prop_assert_eq!(add(a, b), add(b, a));
    }} }}

    The framework runs this with 256 random (a, b) pairs automatically.
    If any pair fails, it shrinks to the minimal failing example.

  Properties to look for:
    ✓ Round-trip:     decode(encode(x)) == x
    ✓ Idempotent:     f(f(x)) == f(x)   (sort is idempotent)
    ✓ Commutative:    f(a, b) == f(b, a) (add, max, min)
    ✓ Identity:       f(x, identity) == x (x + 0 == x)
    ✓ Invariant:      sorted(x) is sorted for all x
    ✓ Bounds:         clamp(x, lo, hi) is always in [lo, hi]
    ✓ Size:           len(filter(x)) <= len(x) always
"#);
}

fn demo_shrinking_explanation() {
    println!("--- Shrinking: Finding the Minimal Failing Case ---\n");
    println!(r#"  When proptest finds a failing input like:
    vec![5, -3, 999, 0, 42, 17, -100, 8]

  It automatically shrinks it by trying smaller versions:
    vec![5, -3, 999, 0, 42]   → still fails?
    vec![5, -3, 999]           → still fails?
    vec![-3, 999]              → still fails?
    vec![999]                  → passes? try others...
    vec![-3]                   → fails!  ← minimal case

  You see: "test failed for input: vec![-3]" — much easier to debug.

  C# FsCheck does the same thing automatically.
  xUnit [InlineData] does NOT shrink — you see the raw generated input.
"#);
}

fn demo_strategies_overview() {
    println!("--- Proptest Strategies (Input Generators) ---\n");
    println!(r#"  Strategy           What it generates
  ─────────────────────────────────────────────────────────────────
  any::<i32>()         any i32 in full range
  0_i32..100           integers 0..100
  ".*"                 any string matching the regex
  vec(any::<u8>(), 0..50)  Vec<u8> with 0–50 elements
  prop_oneof![...]     randomly pick one of the listed strategies
  Just(42)             always generates 42 (useful for edge cases)

  Derive-able: #[derive(Arbitrary)] on your own structs
               (via proptest-derive crate)

  C# FsCheck equivalent:
  Arb.Generate<int>()      ≈  any::<i32>()
  Gen.Choose(0, 100)        ≈  0_i32..100
  Arb.From(gen)             ≈  custom strategy
"#);
}

// ─── PROPERTY TESTS ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // ── 1. Commutative property ─────────────────────────────────────────────
    //
    // C# FsCheck: [Property] Property AddCommutative(int a, int b) =>
    //   (Add(a, b) == Add(b, a)).ToProperty();

    proptest! {
        #[test]
        fn add_is_commutative(a: i32, b: i32) {
            prop_assert_eq!(
                add_commutative(a, b),
                add_commutative(b, a)
            );
        }
    }

    // ── 2. Round-trip property ──────────────────────────────────────────────
    //
    // encode then decode must give back the original.
    // This catches asymmetries in codec implementations.

    proptest! {
        #[test]
        fn encode_decode_roundtrip(s in ".*") {
            let encoded = encode_decode(&s);
            let decoded = decode(&encoded);
            prop_assert_eq!(&s, &decoded);
        }
    }

    // ── 3. Invariant: clamp output always in bounds ─────────────────────────

    proptest! {
        #[test]
        fn clamp_output_always_in_range(
            value in any::<i32>(),
            lo in -1000_i32..=0_i32,
            hi in 0_i32..=1000_i32,
        ) {
            let result = clamp(value, lo, hi);
            prop_assert!(result >= lo, "result {result} < lo {lo}");
            prop_assert!(result <= hi, "result {result} > hi {hi}");
        }
    }

    // ── 4. Idempotent: sort_and_deduplicate applied twice == once ───────────

    proptest! {
        #[test]
        fn sort_deduplicate_is_idempotent(v: Vec<i32>) {
            let once  = sort_and_deduplicate(v.clone());
            let twice = sort_and_deduplicate(once.clone());
            prop_assert_eq!(&once, &twice);
        }
    }

    // ── 5. Sorted output is actually sorted ─────────────────────────────────

    proptest! {
        #[test]
        fn output_is_sorted(v: Vec<i32>) {
            let sorted = sort_and_deduplicate(v);
            for window in sorted.windows(2) {
                prop_assert!(
                    window[0] <= window[1],
                    "not sorted: {} > {}", window[0], window[1]
                );
            }
        }
    }

    // ── 6. No duplicates after deduplicate ──────────────────────────────────

    proptest! {
        #[test]
        fn no_duplicates_after_dedup(v: Vec<i32>) {
            let result = sort_and_deduplicate(v);
            for window in result.windows(2) {
                prop_assert_ne!(window[0], window[1], "duplicate found: {}", window[0]);
            }
        }
    }

    // ── 7. safe_divide: result * b == a when b != 0 ─────────────────────────

    proptest! {
        #[test]
        fn safe_divide_none_only_when_zero(a: i64, b: i64) {
            match safe_divide(a, b) {
                None    => prop_assert_eq!(b, 0),
                Some(_) => prop_assert_ne!(b, 0),
            }
        }
    }

    // ── 8. Combining with specific edge cases ───────────────────────────────
    //
    // prop_oneof! lets you mix random with hand-crafted edge cases:

    proptest! {
        #[test]
        fn clamp_with_edge_cases(
            value in prop_oneof![
                Just(i32::MIN),          // minimum possible i32
                Just(i32::MAX),          // maximum possible i32
                Just(0_i32),             // zero
                any::<i32>(),            // random
            ],
        ) {
            let result = clamp(value, -100, 100);
            prop_assert!(result >= -100);
            prop_assert!(result <= 100);
        }
    }

    // ── Traditional example-based tests alongside proptest ──────────────────
    // Both coexist naturally in the same module.

    #[test]
    fn clamp_known_values() {
        assert_eq!(clamp(50,  0, 100), 50);   // in range
        assert_eq!(clamp(-5,  0, 100),  0);   // below min
        assert_eq!(clamp(200, 0, 100), 100);  // above max
    }

    #[test]
    fn safe_divide_by_zero_is_none() {
        assert_eq!(safe_divide(42, 0), None);
    }

    #[test]
    fn safe_divide_normal() {
        assert_eq!(safe_divide(10, 2), Some(5));
    }
}
