// ============================================================
// CONCEPT: Hello World — Your First Rust Program
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C# you write:
//   using System;
//   class Program { static void Main() { Console.WriteLine("Hello"); } }
//
// Rust has no classes, no runtime, no GC, and no JIT. The entry point is a
// free function called `main`. Compilation produces a native binary.
//
// RUN: cargo run --bin hello_world
// ============================================================

// ─── C# VERSION ─────────────────────────────────────────────
// using System;
// class Program {
//     static void Main(string[] args) {
//         Console.WriteLine("Hello, World!");
//         Console.WriteLine($"Arguments: {args.Length}");
//     }
// }

// ─── RUST VERSION ────────────────────────────────────────────
fn main() {
    // println! is a macro (note the !), not a function.
    // It checks format string correctness at compile time — no runtime format errors.
    println!("Hello, World!");

    // std::env::args() returns an iterator over command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    println!("Program name: {}", args[0]);
    println!("Argument count (excluding program name): {}", args.len() - 1);

    // ── SIMPLE EXAMPLE ──────────────────────────────────────
    greet("C# Developer");

    // ── ADVANCED EXAMPLE ────────────────────────────────────
    advanced_formatting();
}

fn greet(name: &str) {
    // &str is an immutable string slice — roughly analogous to a C# `string`
    // that you cannot mutate (all C# strings are immutable anyway, but the
    // type system here enforces it through borrowing).
    println!("Hello, {}! Welcome to Rust.", name);
}

fn advanced_formatting() {
    // Rust's format macros support named arguments, debug printing,
    // padding, precision, and more — all resolved at compile time.
    let name = "Rust";
    let version = 1.95_f64;

    println!("Language : {name}");           // named argument (Rust 1.58+)
    println!("Version  : {version:.2}");     // two decimal places
    println!("Debug    : {:?}", (name, version)); // Debug trait output
    println!("Pretty   : {:#?}", vec![1, 2, 3]);  // pretty-printed debug

    // Padding and alignment
    println!("{:<10} | {:>10} | {:^10}", "left", "right", "center");
    println!("{:0>5}", 42);  // zero-pad to width 5 → "00042"

    // Binary / hex / octal
    println!("dec={0} hex={0:#x} oct={0:#o} bin={0:#b}", 255);
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. `println!` is a macro (!) — format string is compile-time checked.
// 2. No `using` / `namespace` boilerplate at the top for basics.
// 3. The binary is native — no CLR, no JIT warmup.
// 4. `fn main()` instead of `static void Main()`.
// 5. String literals are UTF-8 `&str` slices, not UTF-16 System.String.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Print your name and today's date using named arguments.
// 2. Print the numbers 1-10 using a loop inside main.
// 3. Modify `greet` to accept an age (u32) and include it in the message.
// 4. Print all command-line arguments passed to the program.

// ─── SOLUTIONS ───────────────────────────────────────────────
#[allow(dead_code)]
fn exercise_solutions() {
    // Exercise 1
    let name = "Alice";
    let date = "2026-05-17";
    println!("Name: {name}, Date: {date}");

    // Exercise 2
    for i in 1..=10 {
        print!("{i} ");
    }
    println!();

    // Exercise 3
    fn greet_with_age(name: &str, age: u32) {
        println!("Hello, {name}! You are {age} years old.");
    }
    greet_with_age("Bob", 30);

    // Exercise 4
    for (i, arg) in std::env::args().enumerate() {
        println!("arg[{i}] = {arg}");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_string_works() {
        let s = format!("{:0>5}", 42);
        assert_eq!(s, "00042");
    }

    #[test]
    fn named_arg_works() {
        let x = 7;
        let s = format!("{x}");
        assert_eq!(s, "7");
    }
}
