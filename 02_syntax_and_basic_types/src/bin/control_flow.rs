// ============================================================
// CONCEPT: Control Flow — if, loop, while, for, match
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust control flow is mostly familiar, with key differences:
//   • `if` is an EXPRESSION — returns a value (like C# ternary `? :`)
//   • `loop` is an infinite loop that can `break` with a value
//   • `match` replaces switch/case and is exhaustive + expression
//   • `for` iterates over iterators (no index-based C-style for loop)
//   • `while let` is a loop driven by pattern matching
//
// RUN: cargo run --bin control_flow
// ============================================================

fn main() {
    if_expressions();
    loop_expressions();
    while_loops();
    for_loops();
    match_expressions();
    if_let_while_let();
    let_else();
}

fn if_expressions() {
    println!("=== if Expressions ===");

    let x = 7;

    // Basic if — same as C#
    if x > 5 {
        println!("{x} is greater than 5");
    } else if x == 5 {
        println!("{x} equals 5");
    } else {
        println!("{x} is less than 5");
    }

    // if as an expression (like C# ternary `x > 5 ? "big" : "small"`)
    let description = if x > 5 { "big" } else { "small" };
    println!("{x} is {description}");

    // Both arms must produce the SAME type:
    let abs_val = if x >= 0 { x } else { -x };
    println!("abs({x}) = {abs_val}");

    // Used directly in a function call:
    println!("max = {}", if x > 10 { x } else { 10 });
}

fn loop_expressions() {
    println!("\n=== loop (infinite loop with optional break value) ===");

    // Basic infinite loop — break to exit
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 { break; }
    }
    println!("counter = {counter}");

    // loop can RETURN a value via `break value`:
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // this is the value of the `loop` expression
        }
    };
    println!("loop result = {result}"); // 20

    // Nested loops with labels — like C#'s labeled break (goto in disguise)
    let mut found = false;
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 7 {
                println!("Found: i={i} j={j}");
                found = true;
                break 'outer; // break the outer loop
            }
        }
    }
    println!("found = {found}");
}

fn while_loops() {
    println!("\n=== while Loops ===");

    // Standard while — same as C#
    let mut n = 1;
    while n < 100 {
        n *= 2;
    }
    println!("first power of 2 >= 100: {n}");

    // Counting down:
    let mut countdown = 5;
    while countdown > 0 {
        print!("{countdown}... ");
        countdown -= 1;
    }
    println!("Go!");
}

fn for_loops() {
    println!("\n=== for Loops (iterator-based) ===");

    // Rust `for` iterates over anything implementing IntoIterator.
    // There is NO C-style `for (int i = 0; i < n; i++)` loop.

    // Range — exclusive end
    for i in 0..5 {
        print!("{i} ");
    }
    println!();

    // Range — inclusive end
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();

    // Iterate over array (borrows elements):
    let arr = [10, 20, 30, 40, 50];
    for val in &arr {
        print!("{val} ");
    }
    println!();

    // Enumerate — C# equivalent: foreach (var (i, v) in arr.Select((v,i) => (i,v)))
    for (i, val) in arr.iter().enumerate() {
        println!("[{i}] = {val}");
    }

    // Iterating with step — use step_by():
    for i in (0..10).step_by(2) {
        print!("{i} ");
    }
    println!();

    // Iterating in reverse:
    for i in (0..5).rev() {
        print!("{i} ");
    }
    println!();

    // Collect results of a loop using map/filter (more idiomatic than for+push):
    let doubled: Vec<i32> = (1..=5).map(|x| x * 2).collect();
    println!("doubled: {doubled:?}");
}

fn match_expressions() {
    println!("\n=== match Expressions ===");

    // match is like switch/case but:
    // 1. It's an expression (returns a value)
    // 2. It is exhaustive — all cases must be covered
    // 3. Arms can use patterns, not just literals

    let n = 7;
    let description = match n {
        1         => "one",
        2 | 3     => "two or three",        // OR patterns
        4..=6     => "four through six",    // range pattern
        7         => "lucky seven",
        _         => "something else",      // wildcard (default)
    };
    println!("{n} is {description}");

    // match with binding (@):
    let val = 15;
    match val {
        n @ 1..=12  => println!("{n} is a month number"),
        n @ 13..=19 => println!("{n} is a teen"),
        n           => println!("{n} is something else"),
    }

    // match on tuples:
    let point = (0_i32, -5_i32);
    match point {
        (0, 0)   => println!("origin"),
        (x, 0)   => println!("on x-axis at {x}"),
        (0, y)   => println!("on y-axis at {y}"),
        (x, y)   => println!("at ({x}, {y})"),
    }

    // match guards — additional conditions on patterns:
    let pair = (2_i32, -2_i32);
    match pair {
        (x, y) if x == y        => println!("equal"),
        (x, y) if x + y == 0   => println!("sum to zero"),
        (x, _)                  => println!("x is {x}"),
    }
}

fn if_let_while_let() {
    println!("\n=== if let and while let ===");

    // `if let` is syntactic sugar for a match with one arm.
    // Useful when you only care about ONE pattern.
    let maybe: Option<i32> = Some(42);

    // Verbose match:
    match maybe {
        Some(v) => println!("got {v}"),
        None    => println!("nothing"),
    }

    // Equivalent if let:
    if let Some(v) = maybe {
        println!("if let got {v}");
    }

    // with else:
    if let Some(v) = maybe {
        println!("value: {v}");
    } else {
        println!("no value");
    }

    // while let — loop as long as pattern matches:
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped: {top}");
    }
    println!("stack empty: {}", stack.is_empty());
}

fn let_else() {
    println!("\n=== let-else (Rust 1.65+) ===");

    // let-else is like `if let` but diverges when the pattern doesn't match.
    // Useful for early-return validation — like C# pattern matching with `is`
    // combined with a guard return.

    fn process(input: &str) -> Option<i32> {
        // If `parse()` fails, execute the `else` block (must diverge: return/break/panic)
        let Ok(n) = input.trim().parse::<i32>() else {
            println!("  '{input}' is not a valid integer");
            return None;
        };
        println!("  parsed: {n}");
        Some(n * 2)
    }

    process("42");
    process("hello");
    process("  7  ");

    // C# equivalent:
    // if (!int.TryParse(input, out var n)) { return null; }
    // return n * 2;
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. `if` and `match` are expressions — they return values.
// 2. `match` is exhaustive — missing a case is a compile error.
// 3. No C-style for loop — use ranges, iterators, or `while`.
// 4. `loop` can break with a value.
// 5. Loop labels ('outer:) enable breaking outer loops.
// 6. `let-else` provides concise early-return on pattern mismatch.

#[cfg(test)]
mod tests {
    #[test]
    fn if_expression_returns_value() {
        let x = 10;
        let s = if x > 5 { "big" } else { "small" };
        assert_eq!(s, "big");
    }

    #[test]
    fn loop_breaks_with_value() {
        let mut i = 0;
        let v = loop {
            i += 1;
            if i == 3 { break i * 10; }
        };
        assert_eq!(v, 30);
    }

    #[test]
    fn match_is_exhaustive() {
        let x = 99;
        let s = match x {
            0        => "zero",
            1..=10   => "small",
            _        => "other",
        };
        assert_eq!(s, "other");
    }

    #[test]
    fn for_range_sum() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
    }
}
