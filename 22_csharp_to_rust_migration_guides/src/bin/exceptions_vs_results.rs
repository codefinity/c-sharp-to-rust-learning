// ============================================================
// MIGRATION GUIDE: C# Exceptions vs Rust Result<T, E>
// ============================================================
//
// C# uses exceptions (throw/catch) for error handling.
// Rust uses Result<T, E> and Option<T> — errors are values.
//
// This is one of the biggest mental shifts. Rust forces you to
// think about what can fail and handle it explicitly.
//
// RUN: cargo run --bin exceptions_vs_results
// ============================================================

use std::num::ParseIntError;
use std::fmt;

fn main() {
    println!("=== Exceptions vs Result<T, E> ===\n");

    basic_comparison();
    error_propagation();
    error_types();
    option_vs_null();
    best_practices();
}

// ---- 1. Basic comparison ------------------------------------------

fn basic_comparison() {
    println!("--- Basic Comparison ---");

    println!(r#"
C# exception throwing and catching:
  int Parse(string s) {{
      if (!int.TryParse(s, out int n)) throw new FormatException($"bad: {{s}}");
      return n;
  }}
  try {{ int n = Parse("42"); }}
  catch (FormatException e) {{ Console.Error.WriteLine(e.Message); }}

Rust — errors are return values:
  fn parse(s: &str) -> Result<i32, ParseIntError> {{
      s.parse::<i32>()
  }}
  match parse("42") {{
      Ok(n)  => println!("{{n}}"),
      Err(e) => eprintln!("error: {{e}}"),
  }}

Key differences:
  • Rust errors are part of the function signature (visible to callers)
  • Unchecked exceptions → hard to know what can throw
  • Result<T,E> → impossible to forget to handle an error (compiler warns)
  • panic! ≈ RuntimeException — only for truly unrecoverable situations
"#);

    let ok: Result<i32, &str>  = Ok(42);
    let err: Result<i32, &str> = Err("something failed");

    println!("ok:  {ok:?}");
    println!("err: {err:?}");
    println!("ok.is_ok():   {}", ok.is_ok());
    println!("err.is_err(): {}", err.is_err());
    println!("ok.unwrap_or(0): {}", ok.unwrap_or(0));
    println!("err.unwrap_or(0): {}", err.unwrap_or(0));
}

// ---- 2. Error propagation with ? ----------------------------------

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    TooBig(i32),
    Io(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Parse(e)  => write!(f, "parse error: {e}"),
            AppError::TooBig(n) => write!(f, "value too big: {n}"),
            AppError::Io(e)     => write!(f, "I/O error: {e}"),
        }
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

fn parse_positive(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.trim().parse()?;   // ? converts ParseIntError → AppError
    if n > 1000 { return Err(AppError::TooBig(n)); }
    Ok(n)
}

fn load_and_parse(s: &str) -> Result<i32, AppError> {
    // Multiple ?s chain naturally:
    let n = parse_positive(s)?;
    Ok(n * 2)
}

fn error_propagation() {
    println!("\n--- Error Propagation with ? ---");

    println!(r#"
C# rethrow pattern:
  int Process(string s) {{
      try {{ return int.Parse(s) * 2; }}
      catch (FormatException e) {{
          throw new AppException("bad input", e);  // wrap and rethrow
      }}
  }}

Rust ? operator:
  fn process(s: &str) -> Result<i32, AppError> {{
      let n: i32 = s.parse()?;  // ? = Ok(v) → v, Err(e) → return Err(From::from(e))
      Ok(n * 2)
  }}
"#);

    for input in &["42", "1001", "abc", "  7  "] {
        match load_and_parse(input) {
            Ok(n)  => println!("  '{input}' → {n}"),
            Err(e) => println!("  '{input}' → Error: {e}"),
        }
    }
}

// ---- 3. Error types: thiserror vs anyhow vs manual ----------------

fn error_types() {
    println!("\n--- Error Type Patterns ---");

    println!(r#"
1. Manual (educational, max control):
   enum MyError {{ IoError(std::io::Error), ParseError(ParseIntError) }}
   impl fmt::Display for MyError {{ ... }}
   impl From<std::io::Error> for MyError {{ ... }}

2. thiserror (library code — typed errors for callers):
   #[derive(thiserror::Error, Debug)]
   enum MyError {{
       #[error("I/O error: {{0}}")]
       Io(#[from] std::io::Error),
       #[error("parse error: {{0}}")]
       Parse(#[from] ParseIntError),
   }}

3. anyhow (application code — ergonomic, boxed errors):
   fn run() -> anyhow::Result<()> {{
       let s = std::fs::read_to_string("file.txt")?;
       let n: i32 = s.trim().parse().context("not a number")?;
       Ok(())
   }}

Rule of thumb:
  • Library crate    → thiserror (callers can match on specific errors)
  • Application main → anyhow (just need to display and exit)
  • Performance-critical path → manual enum (no heap alloc)
"#);

    // Collecting Results — common C# pattern: catch inside Select:
    let strings = ["1", "2", "bad", "4", "5"];

    // Collect only successes (like C#'s Where + int.TryParse):
    let good: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("  filter_map parse: {good:?}");

    // Collect all or fail on first error:
    let all: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("  collect all or fail: {}", all.is_err());

    // Partition into (Ok, Err):
    let (oks, errs): (Vec<_>, Vec<_>) = strings.iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("  partition: {} ok, {} err", oks.len(), errs.len());
}

// ---- 4. Option<T> vs null ----------------------------------------

fn option_vs_null() {
    println!("\n--- Option<T> vs null ---");

    println!(r#"
C# nullable reference types (C# 8+):
  string? name = null;
  if (name is not null) {{ Console.Write(name.Length); }}

Rust Option<T>:
  let name: Option<String> = None;
  if let Some(ref n) = name {{ println!("{{}}",  n.len()); }}

C# null-coalescing:
  var n = name ?? "default";   → let n = name.unwrap_or("default");
  var n = name ?? GetDefault(); → let n = name.unwrap_or_else(GetDefault);

C# null-conditional:
  name?.Length                 → name.as_ref().map(|n| n.len())
  name?.ToUpper() ?? "NONE"    → name.as_deref().map(str::to_uppercase).unwrap_or("NONE".to_string())

C# NullReferenceException:
  At runtime if null is dereferenced.
  In Rust: impossible — Option forces you to check before access.
"#);

    let some: Option<String> = Some("hello".to_string());
    let none: Option<String> = None;

    // unwrap_or, map, and_then:
    println!("  some.map len: {:?}", some.as_ref().map(|s| s.len()));
    println!("  none.unwrap_or: {}", none.as_deref().unwrap_or("default"));

    // Option as iterator — filter_map:
    let items: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let values: Vec<i32> = items.into_iter().flatten().collect();
    println!("  flatten Options: {values:?}");

    // ? on Option inside fn returning Option:
    fn first_char(s: Option<&str>) -> Option<char> {
        s?.chars().next()
    }
    println!("  first_char(Some(\"hi\")) = {:?}", first_char(Some("hi")));
    println!("  first_char(None)       = {:?}", first_char(None));
}

// ---- 5. Best practices summary ------------------------------------

fn best_practices() {
    println!("\n--- Best Practices ---");

    println!(r#"
Coming from C# to Rust error handling:

DO:
  ✓ Return Result<T, E> for all fallible operations
  ✓ Use ? to propagate errors without ceremony
  ✓ Use thiserror for library errors (typed, matchable)
  ✓ Use anyhow for binary/application errors (ergonomic)
  ✓ Use Option<T> for values that may not exist (not null)
  ✓ Use .map() / .and_then() to transform values without unwrapping
  ✓ Use .unwrap_or() / .unwrap_or_else() for defaults
  ✓ Match on Err variant when you need to handle specific cases

DON'T:
  ✗ .unwrap() in production code (panic on Err/None)
  ✗ .expect("msg") on inputs you don't control (use ? instead)
  ✗ panic! for expected error conditions (use Result)
  ✗ Box<dyn Error> for library APIs (use a typed enum)
  ✗ Return Option when the error reason matters (use Result)
  ✗ Use unwrap() where the caller needs to handle the failure
"#);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid() {
        assert_eq!(parse_positive("42").unwrap(), 42);
    }

    #[test]
    fn parse_invalid() {
        assert!(parse_positive("abc").is_err());
    }

    #[test]
    fn parse_too_big() {
        assert!(matches!(parse_positive("9999"), Err(AppError::TooBig(9999))));
    }
}
