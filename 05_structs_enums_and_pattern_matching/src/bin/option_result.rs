// ============================================================
// CONCEPT: Option<T> and Result<T,E> — Deep Dive
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses:
//   • `null` for absence of a value  →  Rust uses `Option<T>`
//   • exceptions for errors           →  Rust uses `Result<T, E>`
//
// The `?` operator in Rust propagates errors just like `throw` in C#,
// but it's explicit and type-safe — the return type must be Result or Option.
//
// C# exception throwing:
//   int Parse(string s) => int.Parse(s); // throws on failure
//
// Rust ? operator:
//   fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
//       Ok(s.trim().parse::<i32>()?) // ? propagates the error
//   }
//
// RUN: cargo run --bin option_result
// ============================================================

use std::num::ParseIntError;
use std::fmt;

fn main() {
    option_combinators();
    result_combinators();
    question_mark_operator();
    custom_error_types();
    converting_between_option_result();
}

fn option_combinators() {
    println!("=== Option Combinators ===");

    let some: Option<i32> = Some(10);
    let none: Option<i32> = None;

    // map — transform the inner value (like LINQ Select)
    println!("map: {:?}", some.map(|x| x * 2));     // Some(20)
    println!("map on None: {:?}", none.map(|x| x)); // None

    // and_then — flatMap / chain (like LINQ SelectMany on Option)
    // C# null-conditional: val?.Method()
    fn try_double(x: i32) -> Option<i32> {
        if x < 100 { Some(x * 2) } else { None }
    }
    println!("and_then: {:?}", some.and_then(try_double)); // Some(20)
    println!("and_then None: {:?}", none.and_then(try_double)); // None
    println!("and_then filtered: {:?}", Some(200).and_then(try_double)); // None

    // or / or_else — provide a fallback
    println!("or: {:?}", none.or(Some(99)));            // Some(99)
    println!("or_else: {:?}", none.or_else(|| Some(42))); // Some(42)

    // unwrap_or / unwrap_or_else / unwrap_or_default
    println!("unwrap_or: {}", none.unwrap_or(0));
    println!("unwrap_or_else: {}", none.unwrap_or_else(|| 42));
    println!("unwrap_or_default: {}", none.unwrap_or_default()); // i32 default = 0

    // filter — keep Some if predicate holds
    println!("filter(>5): {:?}", some.filter(|&x| x > 5));   // Some(10)
    println!("filter(>20): {:?}", some.filter(|&x| x > 20)); // None

    // zip — combine two Options into a tuple
    let a = Some(1_i32);
    let b = Some("hello");
    println!("zip: {:?}", a.zip(b)); // Some((1, "hello"))

    // flatten — Option<Option<T>> → Option<T>
    let nested: Option<Option<i32>> = Some(Some(7));
    println!("flatten: {:?}", nested.flatten()); // Some(7)
}

fn result_combinators() {
    println!("\n=== Result Combinators ===");

    let ok: Result<i32, &str> = Ok(10);
    let err: Result<i32, &str> = Err("failed");

    // map — transform Ok value
    println!("map: {:?}", ok.map(|x| x * 2));
    println!("map err: {:?}", err.map(|x| x * 2));

    // map_err — transform Err value
    println!("map_err: {:?}", err.map_err(|e| format!("Error: {e}")));

    // and_then — chain operations that can fail
    fn parse_and_double(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| e.to_string())
            .and_then(|n| if n >= 0 { Ok(n * 2) } else { Err("negative".into()) })
    }
    println!("chain '5': {:?}", parse_and_double("5"));
    println!("chain 'x': {:?}", parse_and_double("x"));
    println!("chain '-1': {:?}", parse_and_double("-1"));

    // or / or_else — provide fallback
    println!("or: {:?}", err.or(Ok::<i32, &str>(99)));

    // unwrap_or — get value or default on error
    println!("unwrap_or: {}", err.unwrap_or(0));

    // collect Vec<Result<T,E>> → Result<Vec<T>,E>
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect all ok: {numbers:?}");

    let bad_strings = vec!["1", "two", "3"];
    let bad_numbers: Result<Vec<i32>, _> = bad_strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("collect with error: {}", bad_numbers.is_err());
}

fn question_mark_operator() {
    println!("\n=== The ? Operator ===");

    // ? is syntactic sugar for:
    //   match result { Ok(v) => v, Err(e) => return Err(e.into()) }
    // It propagates errors up the call stack.
    // C# analogy: if a method throws, the exception propagates.

    fn read_number(s: &str) -> Result<i32, ParseIntError> {
        let n = s.trim().parse::<i32>()?; // ? propagates ParseIntError
        Ok(n * 2)
    }

    fn double_pair(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let x = read_number(a)?; // propagate if a fails to parse
        let y = read_number(b)?; // propagate if b fails to parse
        Ok(x + y)
    }

    println!("'5' '3': {:?}", double_pair("5", "3"));   // Ok(16) = (5*2)+(3*2)
    println!("'5' 'x': {:?}", double_pair("5", "x"));   // Err(...)

    // ? also works with Option<T> — returns None on None:
    fn first_char(s: &str) -> Option<char> {
        let c = s.chars().next()?; // ? returns None if chars() is empty
        Some(c.to_ascii_uppercase())
    }
    println!("first_char('hello'): {:?}", first_char("hello"));
    println!("first_char(''): {:?}", first_char(""));
}

// ─── CUSTOM ERROR TYPES ───────────────────────────────────────

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    OutOfRange(i32),
    Empty,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e)       => write!(f, "parse error: {e}"),
            AppError::OutOfRange(n)  => write!(f, "value {n} is out of range"),
            AppError::Empty          => write!(f, "input was empty"),
        }
    }
}

// Implement From to enable ? to convert ParseIntError → AppError:
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn parse_score(input: &str) -> Result<i32, AppError> {
    if input.is_empty() {
        return Err(AppError::Empty);
    }
    let n: i32 = input.trim().parse()?; // ? uses From<ParseIntError> → AppError
    if !(0..=100).contains(&n) {
        return Err(AppError::OutOfRange(n));
    }
    Ok(n)
}

fn custom_error_types() {
    println!("\n=== Custom Error Types ===");

    let inputs = ["85", "", "abc", "150", "42"];
    for input in inputs {
        match parse_score(input) {
            Ok(n)  => println!("  '{input}' → score: {n}"),
            Err(e) => println!("  '{input}' → error: {e}"),
        }
    }
}

fn converting_between_option_result() {
    println!("\n=== Converting Between Option and Result ===");

    // Option → Result:
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("was None");
    println!("ok_or: {res:?}");

    let none: Option<i32> = None;
    let res2: Result<i32, &str> = none.ok_or("was None");
    println!("none.ok_or: {res2:?}");

    // Result → Option:
    let ok: Result<i32, &str> = Ok(42);
    println!("ok.ok(): {:?}", ok.ok());     // Some(42)
    println!("ok.err(): {:?}", ok.err());   // None

    let err: Result<i32, &str> = Err("bad");
    println!("err.ok(): {:?}", err.ok());   // None
    println!("err.err(): {:?}", err.err()); // Some("bad")

    // Transpose: Option<Result<T,E>> ↔ Result<Option<T>,E>
    let or: Option<Result<i32, &str>> = Some(Ok(5));
    let ro: Result<Option<i32>, &str> = or.transpose();
    println!("transpose: {ro:?}"); // Ok(Some(5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_chain() {
        let result = Some(5_i32)
            .filter(|&x| x > 3)
            .map(|x| x * 2)
            .unwrap_or(0);
        assert_eq!(result, 10);
    }

    #[test]
    fn parse_score_valid() {
        assert_eq!(parse_score("75"), Ok(75));
    }

    #[test]
    fn parse_score_empty() {
        assert!(matches!(parse_score(""), Err(AppError::Empty)));
    }

    #[test]
    fn parse_score_out_of_range() {
        assert!(matches!(parse_score("150"), Err(AppError::OutOfRange(150))));
    }

    #[test]
    fn question_mark_propagates() {
        fn failing() -> Result<i32, ParseIntError> {
            let _n: i32 = "not a number".parse()?;
            Ok(0)
        }
        assert!(failing().is_err());
    }
}
