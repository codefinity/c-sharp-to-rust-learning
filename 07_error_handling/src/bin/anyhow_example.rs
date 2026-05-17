// ============================================================
// CONCEPT: anyhow — Ergonomic Error Handling (Application Code)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// `thiserror` is for LIBRARY code (typed, inspectable errors).
// `anyhow` is for APPLICATION code (top-level error handling, scripts, CLIs).
//
// `anyhow::Error` is like C# `Exception` base class:
//   • Any error that implements std::error::Error can be wrapped
//   • You can add context with .context() (like Exception.Message chaining)
//   • `?` automatically wraps any compatible error
//   • anyhow::Result<T> = Result<T, anyhow::Error>
//
// C# analogy:
//   throw new Exception("failed to load config", innerException);
// Rust anyhow:
//   .context("failed to load config")?
//
// USE anyhow WHEN:
//   • Writing application (binary) code
//   • You don't need callers to inspect specific error types
//   • Quick prototyping / scripts
//   • The entry point of your error handling chain
//
// RUN: cargo run --bin anyhow_example
// ============================================================

use anyhow::{anyhow, bail, ensure, Context, Result};

fn main() {
    basic_anyhow();
    context_and_chains();
    anyhow_patterns();
    mixing_thiserror_and_anyhow();
}

fn basic_anyhow() {
    println!("=== Basic anyhow ===");

    // anyhow::Result<T> = Result<T, anyhow::Error>
    // Any error implementing std::error::Error converts with ?

    fn parse_number(s: &str) -> Result<i32> {
        // `?` wraps ParseIntError into anyhow::Error
        let n: i32 = s.trim().parse()?;
        Ok(n)
    }

    println!("parse '42': {:?}", parse_number("42"));
    println!("parse 'x':  {:?}", parse_number("x").map_err(|e| e.to_string()));

    // anyhow! macro creates an ad-hoc error:
    fn check_positive(n: i32) -> Result<i32> {
        if n < 0 {
            return Err(anyhow!("expected positive number, got {n}"));
        }
        Ok(n)
    }
    println!("check -1: {:?}", check_positive(-1).map_err(|e| e.to_string()));

    // bail! macro is shorthand for return Err(anyhow!(...)):
    fn check_range(n: i32, lo: i32, hi: i32) -> Result<i32> {
        if n < lo || n > hi {
            bail!("{n} is not in [{lo}, {hi}]");
        }
        Ok(n)
    }
    println!("check_range 150: {:?}", check_range(150, 0, 100).map_err(|e| e.to_string()));

    // ensure! is like assert! but returns Err instead of panicking:
    fn divide(a: f64, b: f64) -> Result<f64> {
        ensure!(b != 0.0, "division by zero: {a} / {b}");
        Ok(a / b)
    }
    println!("10/0: {:?}", divide(10.0, 0.0).map_err(|e| e.to_string()));
    println!("10/2: {:?}", divide(10.0, 2.0));
}

fn context_and_chains() {
    println!("\n=== Context and Error Chains ===");

    // .context("msg") adds a layer of context — like wrapping an exception:
    fn read_config_file(path: &str) -> Result<String> {
        std::fs::read_to_string(path)
            .context(format!("failed to read config file '{path}'"))
    }

    // Chain multiple contexts as errors propagate:
    fn load_app() -> Result<()> {
        let _config = read_config_file("app.toml")
            .context("application startup failed")?;
        Ok(())
    }

    match load_app() {
        Ok(()) => println!("app loaded"),
        Err(e) => {
            println!("Error: {e}");
            // Print the full chain (like InnerException.ToString() in C#):
            println!("Chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("  {}: {}", i + 1, cause);
            }
        }
    }

    // with_context — lazy version (closure only called on error path):
    fn read_score(path: &str) -> Result<i32> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading score from '{path}'"))?;
        content.trim().parse::<i32>()
            .with_context(|| format!("parsing score in '{path}'"))
    }
    let _ = read_score("scores.txt"); // will error — that's expected
}

fn anyhow_patterns() {
    println!("\n=== Common anyhow Patterns ===");

    // Pattern 1: main returns Result — anyhow prints the error chain:
    // fn main() -> anyhow::Result<()> { ... }
    // If main returns Err, Rust prints "Error: ..." and exits with code 1.

    // Pattern 2: Downcasting — get the original error type back:
    fn failing_op() -> Result<()> {
        let _: i32 = "not a number".parse()?;
        Ok(())
    }

    let err = failing_op().unwrap_err();
    println!("type-erased error: {err}");

    // Downcast to the original error type:
    if let Some(parse_err) = err.downcast_ref::<std::num::ParseIntError>() {
        println!("  original type: ParseIntError = {parse_err}");
    }

    // Pattern 3: Collecting multiple errors:
    let inputs = ["1", "two", "3", "four"];
    let results: Vec<Result<i32>> = inputs.iter()
        .map(|s| s.parse::<i32>().map_err(anyhow::Error::from))
        .collect();

    let (successes, failures): (Vec<_>, Vec<_>) = results.into_iter()
        .partition(Result::is_ok);
    println!("successes: {}", successes.len());
    println!("failures: {}", failures.len());
    for f in failures {
        println!("  failed: {}", f.unwrap_err());
    }
}

fn mixing_thiserror_and_anyhow() {
    println!("\n=== Mixing thiserror + anyhow ===");

    println!(
        r#"
The recommended pattern:

  LIBRARIES (thiserror):
    • Define typed, inspectable error enums
    • Callers can match on variants
    • Implement Display and Error manually or via thiserror

  APPLICATIONS (anyhow):
    • Use anyhow::Result in main and top-level functions
    • Add context with .context() as errors propagate
    • thiserror errors are automatically wrapped by anyhow via ?

  Example flow:
    library fn parse_config() -> Result<Config, ConfigError>
         ↓  ?
    app    fn load_app()     -> anyhow::Result<App>
         ↓  ? (ConfigError wrapped into anyhow::Error)
    app    fn main()         -> anyhow::Result<()>
"#
    );

    // Demonstration: thiserror error wrapped into anyhow context
    use thiserror::Error;

    #[derive(Debug, Error)]
    enum LibraryError {
        #[error("bad input: {0}")]
        BadInput(String),
    }

    fn library_function(input: &str) -> std::result::Result<i32, LibraryError> {
        input.parse::<i32>().map_err(|_| LibraryError::BadInput(input.into()))
    }

    fn app_function(input: &str) -> Result<i32> {
        // thiserror error → anyhow::Error via ? (From impl is auto-generated
        // because LibraryError: std::error::Error)
        let n = library_function(input)
            .context(format!("app_function: parsing '{input}'"))?;
        Ok(n * 2)
    }

    match app_function("hello") {
        Ok(n)  => println!("result: {n}"),
        Err(e) => {
            println!("app error: {e}");
            for cause in e.chain().skip(1) {
                println!("  caused by: {cause}");
            }
        }
    }
}

// ─── RESULT PATTERNS CHEAT SHEET ─────────────────────────────
//
// ? operator:      propagate error (with From conversion)
// unwrap():        panic on Err/None (test code only)
// expect("msg"):   panic with message (test / impossible code paths)
// map():           transform Ok value
// map_err():       transform Err value
// and_then():      chain operations
// or_else():       provide fallback
// context():       add context layer (anyhow)
// bail!():         early return with Err (anyhow)
// ensure!():       conditional early return (anyhow)
// ok_or():         Option → Result
// ok():            Result → Option

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anyhow_wraps_error() {
        fn failing() -> Result<i32> {
            let _: i32 = "x".parse()?;
            Ok(0)
        }
        assert!(failing().is_err());
    }

    #[test]
    fn bail_returns_err() {
        fn bailing(x: i32) -> Result<i32> {
            if x < 0 { bail!("negative: {x}"); }
            Ok(x)
        }
        assert!(bailing(-1).is_err());
        assert_eq!(bailing(5).unwrap(), 5);
    }

    #[test]
    fn ensure_on_false() {
        fn checked(b: bool) -> Result<()> {
            ensure!(b, "b was false");
            Ok(())
        }
        assert!(checked(false).is_err());
        assert!(checked(true).is_ok());
    }
}
