// ============================================================
// CONCEPT: Macros by Example — macro_rules!
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has no hygienic macro system. C# alternatives are:
//   - Source generators (compile-time code generation)
//   - Expression trees + Roslyn analyzers
//   - T4 templates
//
// Rust macros operate on the token stream BEFORE type-checking,
// so they can generate code that normal functions cannot:
//   • Variadic arguments:  println!("x={}", x, y, z)
//   • DSLs:               vec![1,2,3],  matches!(x, Some(_))
//   • Repetition:         implement a trait for all numeric types
//
// macro_rules! = "macros by example" — pattern matching on tokens.
//
// RUN: cargo run --bin macro_rules
// ============================================================

fn main() {
    println!("=== macro_rules! ===\n");

    basic_macros();
    repetition_macros();
    recursive_macros();
    tt_muncher();
    pattern_macros();
}

// ---- 1. Basic macro ------------------------------------------------

// A macro that adds any two expressions.
// C# analogy: there's no equivalent; closest is `T Add<T>(T a, T b)` but
//             macros work at the syntactic level before types are resolved.
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Macro that creates a HashMap in one expression:
macro_rules! map {
    ($( $key:expr => $val:expr ),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $val); )*
        m
    }};
}

fn basic_macros() {
    println!("--- Basic Macros ---");

    let x = add!(3, 4);
    println!("add!(3, 4) = {x}");

    let m = map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("map! keys: {:?}", {
        let mut k: Vec<_> = m.keys().collect();
        k.sort();
        k
    });
}

// ---- 2. Repetition with $( ... )* ----------------------------------

// assert_all! checks every expression in its list.
// C# analogy: you'd need a helper method or parameterized tests.
macro_rules! assert_all {
    ($( $cond:expr ),+ $(,)?) => {
        $( assert!($cond, "failed: {}", stringify!($cond)); )+
    };
}

// Implement a trait for multiple types with one macro invocation:
trait Describable {
    fn describe(&self) -> String;
}

macro_rules! impl_describable_int {
    ($( $t:ty ),+) => {
        $(
            impl Describable for $t {
                fn describe(&self) -> String {
                    format!("{}({})", stringify!($t), self)
                }
            }
        )+
    };
}

impl_describable_int!(i32, i64, u32, u64, usize);

fn repetition_macros() {
    println!("\n--- Repetition ---");

    assert_all!(1 + 1 == 2, "hello".len() == 5, true);
    println!("assert_all! passed");

    let n: i32 = 42;
    println!("describe i32: {}", n.describe());
    let m: u64 = 100;
    println!("describe u64: {}", m.describe());
}

// ---- 3. Recursive macros -------------------------------------------

// min! of any number of values:
macro_rules! min {
    ($x:expr) => { $x };
    ($x:expr, $( $rest:expr ),+) => {
        std::cmp::min($x, min!($( $rest ),+))
    };
}

// Nested vec — creates a Vec<Vec<T>>:
macro_rules! nested_vec {
    ( $( [ $( $inner:expr ),* ] ),* ) => {
        vec![ $( vec![ $( $inner ),* ] ),* ]
    };
}

fn recursive_macros() {
    println!("\n--- Recursive Macros ---");

    let m = min!(5, 3, 8, 1, 7);
    println!("min!(5,3,8,1,7) = {m}");

    let nv = nested_vec!([1, 2, 3], [4, 5], [6]);
    println!("nested_vec: {nv:?}");
}

// ---- 4. Token-tree muncher (parsing mini-DSL) ---------------------

// A tiny calculator DSL parsed entirely at compile time.
// Edition 2024: expr fragments cannot be followed by operators.
// Use tt (token tree) for the operands so operators can follow.
macro_rules! calc {
    ($a:tt + $b:tt) => { $a + $b };
    ($a:tt - $b:tt) => { $a - $b };
    ($a:tt * $b:tt) => { $a * $b };
    ($a:tt / $b:tt) => { $a / $b };
}

fn tt_muncher() {
    println!("\n--- Token-Tree Muncher (mini DSL) ---");

    println!("calc!(10 + 5) = {}", calc!(10 + 5));
    println!("calc!(10 * 3) = {}", calc!(10 * 3));
    println!("calc!(20 / 4) = {}", calc!(20 / 4));
}

// ---- 5. Macro patterns — multiple match arms -----------------------

// A flexible debug printer:
macro_rules! dbg_label {
    ($label:literal, $val:expr) => {
        println!("[{}] {} = {:?}", $label, stringify!($val), $val)
    };
    ($val:expr) => {
        println!("{} = {:?}", stringify!($val), $val)
    };
}

// A retry macro — useful for flaky I/O:
macro_rules! retry {
    ($attempts:expr, $body:block) => {{
        let mut _last_err = String::new();
        let mut _success = false;
        for _i in 0..$attempts {
            let result: Result<_, String> = (|| -> Result<_, String> { $body })();
            match result {
                Ok(v) => { _success = true; break; }
                Err(e) => { _last_err = e; }
            }
        }
        if !_success {
            eprintln!("retry failed after {} attempts: {}", $attempts, _last_err);
        }
    }};
}

fn pattern_macros() {
    println!("\n--- Pattern Macros ---");

    let v = vec![1, 2, 3];
    dbg_label!("main", v);
    dbg_label!(42_i32 + 1);

    let mut attempt = 0;
    retry!(3, {
        attempt += 1;
        if attempt < 3 {
            Err(format!("attempt {attempt} failed"))
        } else {
            println!("retry succeeded on attempt {attempt}");
            Ok(())
        }
    });
}

// ---- Fragment specifiers quick reference ---------------------------
//
// $name:expr    — any expression
// $name:ident   — identifier (variable/function name)
// $name:ty      — type
// $name:pat     — pattern (used in match arms)
// $name:stmt    — statement
// $name:block   — block expression { ... }
// $name:item    — an item (fn, struct, impl, use, ...)
// $name:meta    — attribute content (#[...])
// $name:literal — a literal value (1, "hello", true, ...)
// $name:tt      — a single token tree (most flexible)
// $name:lifetime— a lifetime ('a)
// $name:vis     — a visibility qualifier (pub, pub(crate), ...)
//
// Repetition operators:
//   $( ... )*   — zero or more
//   $( ... )+   — one or more
//   $( ... )?   — zero or one

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_macro() {
        assert_eq!(add!(3, 4), 7);
        assert_eq!(add!(10, -3), 7);
    }

    #[test]
    fn min_macro() {
        assert_eq!(min!(5), 5);
        assert_eq!(min!(5, 3, 8, 1), 1);
    }

    #[test]
    fn map_macro() {
        let m = map!("a" => 1, "b" => 2);
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
    }
}
