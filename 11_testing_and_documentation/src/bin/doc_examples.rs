// ============================================================
// CONCEPT: Documentation Comments and Doc Tests
// ============================================================
// RUN: cargo run --bin doc_examples
// RUN TESTS: cargo test --doc
// GENERATE DOCS: cargo doc --open
// ============================================================

fn main() {
    doc_comments_demo();
    test_organisation_demo();
}

fn doc_comments_demo() {
    println!("=== Documentation Comment Styles ===");
    println!(
        r#"
/// Outer doc comment — documents the item BELOW it
/// Use Markdown: **bold**, `code`, # Headings
///
/// # Examples
/// ```
/// let x = 42;  // this runs as a test!
/// ```
///
/// # Panics    — document panic conditions
/// # Errors    — document error conditions (for Result-returning fns)
/// # Safety    — document unsafe preconditions

//! Inner doc comment — documents the item it's INSIDE
//! Typically used at the top of a module/crate file.

// Regular comments (NOT documentation — not visible in docs)

Sections you can include:
  # Examples        — runnable code examples (doc tests)
  # Panics          — when the function panics
  # Errors          — what errors can be returned
  # Safety          — unsafe preconditions
  # Arguments       — parameter descriptions
  # Returns         — return value description
"#
    );

    use testing_and_documentation::factorial;
    println!("factorial(10) = {}", factorial(10));
}

fn test_organisation_demo() {
    println!("\n=== Test Organisation ===");
    println!(
        r#"
Rust test types:
  1. Unit tests (#[test] in src/ files)
     • In the same file as the code
     • #[cfg(test)] module — only compiled during tests
     • Can access private items
     • Like C# [TestClass] / [TestMethod] in the production project

  2. Integration tests (tests/ directory)
     • Each file is a separate test crate
     • Can only access public API (like a real consumer)
     • Like C# separate test project referencing your library

  3. Doc tests (/// ``` ... ``` in documentation)
     • Code in doc comments is compiled and run
     • Ensures docs stay in sync with code
     • Unique to Rust — C# has no equivalent

Test annotations:
  #[test]                    — marks a test function
  #[should_panic]            — test passes if it panics
  #[should_panic(expected = "msg")]  — panics with this message
  #[ignore]                  — skip this test (run with --include-ignored)
  #[test]
  #[cfg(feature = "expensive")] — conditional test

Running tests:
  cargo test               — all tests
  cargo test fn_name       — tests matching "fn_name"
  cargo test -- --nocapture — show println! output
  cargo test -- --ignored  — run only #[ignore] tests
  cargo test -- --test-threads=1  — single-threaded (for shared state)
"#
    );
}
