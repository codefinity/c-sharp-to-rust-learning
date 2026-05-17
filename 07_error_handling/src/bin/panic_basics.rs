// ============================================================
// CONCEPT: panic! — Unrecoverable Errors
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has two categories of "fail-fast" situations:
//   1. Environment.FailFast() / StackOverflowException — process dies
//   2. Debug.Assert() — checked in debug builds only
//
// Rust has `panic!` — terminates the current thread (or process in
// single-threaded contexts). It is for PROGRAMMING BUGS, not user errors.
//
// C# analogy: throw new InvalidOperationException() for logic bugs,
// but Rust panics unwind the stack (or abort — configurable) rather than
// propagating as catchable exceptions.
//
// Use panic! for: violated invariants, index out of bounds, unwrap on None.
// Use Result: for recoverable errors (file not found, bad input, network).
//
// RUN: cargo run --bin panic_basics
// ============================================================

fn main() {
    panic_in_practice();
    unwrap_and_expect();
    panic_hooks();
    catch_unwind_demo();
}

fn panic_in_practice() {
    println!("=== panic! Use Cases ===");
    println!(
        r#"
When to use panic!:
  • Violating a contract/precondition that SHOULD never happen
  • Index out of bounds (automatic — arrays/Vec)
  • Integer division by zero
  • Stack overflow (recursive functions without base case)
  • Custom: assert!(), assert_eq!(), assert_ne!(), unreachable!(), todo!()

When NOT to use panic!:
  • File not found (use Result<_, io::Error>)
  • Network failure (use Result<_, ...>)
  • Bad user input (use Result<_, ParseError>)
  • Any error the caller should handle

Cargo profile settings control panic behaviour:
  [profile.release]
  panic = "abort"    # smaller binary, no unwinding
  # or
  panic = "unwind"   # default: can catch_unwind
"#
    );

    // Assertions — like C# Debug.Assert / Contract.Assert
    let x = 5;
    assert!(x > 0, "x must be positive, got {x}");
    assert_eq!(x, 5, "expected 5");
    assert_ne!(x, 0, "x must not be zero");
    println!("assertions passed");

    // unreachable! — marks code that should never be reached:
    let val = 2;
    let _result = match val {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => unreachable!("val should be 1-3, got {val}"),
    };

    // todo!() — marks unimplemented code that will be filled in later:
    // fn not_yet_done() -> i32 { todo!("implement this") }
    // Calling it panics: "not yet implemented: implement this"
}

fn unwrap_and_expect() {
    println!("\n=== unwrap() and expect() ===");

    // .unwrap() panics if the value is None or Err
    // Use only when you KNOW the value is present (or in tests):
    let x: Option<i32> = Some(42);
    let val = x.unwrap(); // safe here — we know it's Some
    println!("unwrapped: {val}");

    // .expect("message") — like unwrap but with a custom message:
    let y: Option<i32> = Some(99);
    let val2 = y.expect("y should always be Some in this context");
    println!("expected: {val2}");

    // In production code, prefer proper error handling:
    let text = "42";
    let n: i32 = text.parse().expect("text should always be a valid integer");
    println!("parsed: {n}");

    // Common idiom for "this error means a bug, not user error":
    let result: Result<i32, &str> = Ok(100);
    let value = result.unwrap_or_else(|e| panic!("unexpected error state: {e}"));
    println!("value: {value}");

    println!("\n⚠️  Never use unwrap() on user input — always handle the error.");
}

fn panic_hooks() {
    println!("\n=== Panic Hooks ===");

    // You can install a custom panic hook for logging/telemetry:
    // (Before the hook fires, the stack is unwound and printed.)
    let original = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        println!("[CUSTOM HOOK] panic occurred!");
        if let Some(location) = info.location() {
            println!("  at {}:{}:{}", location.file(), location.line(), location.column());
        }
        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            println!("  message: {msg}");
        }
        // Call the original hook too:
        original(info);
    }));

    // Demonstrate the hook fires (caught by catch_unwind below):
    let _ = std::panic::catch_unwind(|| {
        panic!("demo panic from hook demo");
    });

    // Restore default hook:
    std::panic::set_hook(Box::new(|_| {})); // silent hook for cleanup
}

fn catch_unwind_demo() {
    println!("\n=== catch_unwind (like try/catch for panics) ===");

    // std::panic::catch_unwind() is the closest thing to a try/catch.
    // Use it sparingly — mainly for FFI boundaries and test frameworks.
    // DOES NOT catch panics when panic = "abort".

    let result = std::panic::catch_unwind(|| {
        let v: Vec<i32> = vec![];
        v[0] // panic: index out of bounds
    });

    match result {
        Ok(val)  => println!("succeeded: {val}"),
        Err(_)   => println!("caught a panic (out-of-bounds)"),
    }

    let result2 = std::panic::catch_unwind(|| {
        let x: i32 = 5;
        x * 2 // no panic
    });
    println!("no panic: {:?}", result2);
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. panic! is for programming bugs; Result is for expected failures.
// 2. No try/catch for normal errors — use Result + ? operator.
// 3. catch_unwind exists but is rare — mostly for FFI safety.
// 4. Panics can abort or unwind based on build profile.
// 5. Debug assertions (debug_assert!) are stripped in release builds.

#[cfg(test)]
mod tests {
    #[test]
    fn assert_passes() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds_panics() {
        let v: Vec<i32> = vec![1, 2, 3];
        let _ = v[10]; // panics
    }

    #[test]
    fn catch_unwind_catches_panic() {
        let r = std::panic::catch_unwind(|| panic!("test panic"));
        assert!(r.is_err());
    }
}
