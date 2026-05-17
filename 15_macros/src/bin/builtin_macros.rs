// ============================================================
// CONCEPT: Built-in Macros and std Macro Patterns
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust's standard library ships many macros that look like function
// calls but operate on the AST. Every C# developer reaches for
// Console.WriteLine; in Rust the equivalent is a macro: println!
//
// RUN: cargo run --bin builtin_macros
// ============================================================

use std::collections::HashMap;

fn main() {
    println!("=== Built-in Macros ===\n");

    formatting_macros();
    assertion_macros();
    collection_macros();
    compile_time_macros();
    todo_unimplemented();
    include_macros();
}

// ---- 1. Formatting macros ------------------------------------------

fn formatting_macros() {
    println!("--- Formatting Macros ---");

    // println! / print! — C# Console.WriteLine / Console.Write
    println!("hello, world");
    print!("no newline ");
    println!("← that was print!");

    // Named arguments (Rust 1.58+):
    let name = "Rust";
    println!("Hello, {name}!");          // variable capture (Edition 2021+)
    println!("Hello, {name:>10}!");      // right-align in 10 chars
    println!("pi ≈ {:.4}", std::f64::consts::PI);

    // format! returns a String — C# string.Format / $"..."
    let s = format!("{:05}", 42);        // "00042"
    println!("format!: '{s}'");

    // eprintln! / eprint! write to stderr:
    eprintln!("this goes to stderr");

    // write! / writeln! write to any impl Write:
    use std::io::Write;
    let mut buf = Vec::<u8>::new();
    writeln!(buf, "into a buffer: {}", 99).unwrap();
    println!("write! result: {:?}", String::from_utf8(buf).unwrap().trim());

    // Format specifiers quick-ref:
    println!("{:?}",  (1, "two", 3.0));  // Debug
    println!("{:#?}", vec![1, 2, 3]);    // Pretty Debug
    println!("{:b}",  42_u8);            // binary: 101010
    println!("{:o}",  42_u8);            // octal:  52
    println!("{:x}",  255_u8);           // hex lower: ff
    println!("{:X}",  255_u8);           // hex upper: FF
    println!("{:e}",  1_000_000.0_f64);  // scientific: 1e6
    println!("{:+}",  42_i32);           // with sign: +42
}

// ---- 2. Assertion macros -------------------------------------------

fn assertion_macros() {
    println!("\n--- Assertion Macros ---");

    // assert! — C# Debug.Assert / throw if false
    assert!(1 + 1 == 2);
    assert!(1 + 1 == 2, "math is broken: {} != {}", 1 + 1, 2);

    // assert_eq! / assert_ne! — C# Assert.AreEqual / Assert.AreNotEqual
    assert_eq!(2 + 2, 4);
    assert_ne!(2 + 2, 5);
    assert_eq!(2 + 2, 4, "custom message with context: {}", "here");

    // debug_assert! — only in debug builds (like C# Debug.Assert)
    debug_assert!(true, "only checked in debug mode");

    // matches! — C# pattern matching in boolean expression
    let x: Option<i32> = Some(42);
    assert!(matches!(x, Some(n) if n > 0));
    assert!(!matches!(x, None));

    println!("all assertions passed");
}

// ---- 3. Collection construction macros -----------------------------

fn collection_macros() {
    println!("\n--- Collection Macros ---");

    // vec! — C# new List<T> { 1, 2, 3 }
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0_i32; 5]; // repeat: [0, 0, 0, 0, 0]
    println!("vec!: {v1:?}");
    println!("vec![0;5]: {v2:?}");

    // No built-in map! or set! in std — use HashMap::from / BTreeMap::from:
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("HashMap::from: {} entries", map.len());
}

// ---- 4. Compile-time macros ----------------------------------------

fn compile_time_macros() {
    println!("\n--- Compile-Time Macros ---");

    // env! — read environment variable at compile time (C# compile-time const):
    let cargo_pkg = env!("CARGO_PKG_NAME");
    println!("CARGO_PKG_NAME = {cargo_pkg}");

    let cargo_version = env!("CARGO_PKG_VERSION");
    println!("CARGO_PKG_VERSION = {cargo_version}");

    // option_env! — like env! but returns Option<&str> (won't fail at compile):
    let ci: Option<&str> = option_env!("CI");
    println!("CI env: {:?}", ci);

    // file! / line! / column! — C# [CallerFilePath] / [CallerLineNumber]
    println!("source: {}:{}", file!(), line!());

    // stringify! — turns a token tree into a string literal at compile time:
    let name = stringify!(hello_world_function);
    println!("stringify!: '{name}'");

    // concat! — concatenates string literals at compile time:
    const GREETING: &str = concat!("Hello", ", ", "World", "!");
    println!("concat!: '{GREETING}'");

    // cfg! — evaluate cfg condition at runtime as a bool:
    let is_debug = cfg!(debug_assertions);
    println!("debug build: {is_debug}");

    // include_str! — embed a text file as &str at compile time:
    // (Commented out since we don't have a file to embed)
    // const README: &str = include_str!("../../README.md");
    println!("include_str!/include_bytes! embed files at compile time");
}

// ---- 5. todo! / unimplemented! / unreachable! ----------------------

fn todo_unimplemented() {
    println!("\n--- todo! / unimplemented! / unreachable! ---");

    // todo! — marks code as not yet written (panics with helpful message)
    //         C# analogy: throw new NotImplementedException()
    fn work_in_progress() -> i32 {
        // todo!("implement the real logic here")
        42 // placeholder
    }
    println!("todo! placeholder: {}", work_in_progress());

    // unimplemented! — same as todo! but semantically "will never implement"
    // unreachable! — assert a code path is never reached

    let value: i32 = 3;
    let _ = match value {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => unreachable!("we only pass 1-3"),
    };
    println!("unreachable! not triggered");
}

// ---- 6. include macros demo ----------------------------------------

fn include_macros() {
    println!("\n--- include! family ---");

    // include_bytes! — embed binary file as &[u8] at compile time
    // include_str!   — embed text file as &'static str at compile time
    // include!       — include a Rust source file (e.g., generated code)

    println!("include_str!(\"path\") → &'static str  (text file)");
    println!("include_bytes!(\"path\") → &'static [u8] (binary)");
    println!("These run at compile time — no file I/O at runtime.");

    // Compile-time array of built-in macro names for documentation:
    const BUILTIN: &[&str] = &[
        "println!", "print!", "eprintln!", "eprint!", "format!", "write!", "writeln!",
        "vec!", "assert!", "assert_eq!", "assert_ne!", "debug_assert!",
        "panic!", "todo!", "unimplemented!", "unreachable!",
        "matches!", "dbg!", "env!", "option_env!", "concat!", "stringify!",
        "file!", "line!", "column!", "include_str!", "include_bytes!", "include!",
        "cfg!", "compile_error!",
    ];
    println!("\nRust built-in macros ({} total):", BUILTIN.len());
    for chunk in BUILTIN.chunks(6) {
        println!("  {}", chunk.join("  "));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_macro() {
        assert_eq!(format!("{:05}", 42), "00042");
        assert_eq!(format!("{:.2}", 3.14159), "3.14");
    }

    #[test]
    fn vec_repeat() {
        let v = vec![0_i32; 5];
        assert_eq!(v.len(), 5);
        assert!(v.iter().all(|&x| x == 0));
    }

    #[test]
    fn matches_macro() {
        let x: Option<i32> = Some(42);
        assert!(matches!(x, Some(_)));
        assert!(!matches!(x, None));
        assert!(matches!(x, Some(n) if n > 0));
    }
}
