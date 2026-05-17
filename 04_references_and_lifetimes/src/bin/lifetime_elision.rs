// ============================================================
// CONCEPT: Lifetime Elision
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Explicit lifetime annotations look intimidating. The good news: the
// compiler can INFER lifetime annotations for most common patterns.
// This is called "lifetime elision."
//
// Three elision rules (applied in order):
//   Rule 1: Each reference parameter gets its own lifetime.
//   Rule 2: If there is exactly ONE input lifetime, it's assigned to all outputs.
//   Rule 3: If one of the parameters is &self or &mut self, its lifetime is
//           assigned to all output lifetimes.
//
// If, after applying these rules, all output lifetimes are resolved,
// you DON'T need to write lifetime annotations.
//
// RUN: cargo run --bin lifetime_elision
// ============================================================

fn main() {
    rule_one_demo();
    rule_two_demo();
    rule_three_demo();
    when_you_must_annotate();
}

fn rule_one_demo() {
    println!("=== Rule 1: Each input ref gets its own lifetime ===");

    // Written with explicit lifetimes:
    // fn first_word<'a>(s: &'a str) -> &'a str { ... }  ← one input, so rule 2 applies

    // Elided version (the compiler fills in 'a automatically):
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }

    let s = String::from("hello world");
    println!("first word: '{}'", first_word(&s));
}

fn rule_two_demo() {
    println!("\n=== Rule 2: One input lifetime → assigned to output ===");

    // When there's exactly ONE input reference lifetime, the output
    // implicitly gets the same lifetime.

    // This compiles without annotations:
    fn trim_and_borrow(s: &str) -> &str {
        s.trim()
    }

    // Equivalent explicit form:
    fn trim_explicit<'a>(s: &'a str) -> &'a str {
        s.trim()
    }

    let s = String::from("  hello  ");
    println!("trimmed: '{}'", trim_and_borrow(&s));
    println!("explicit: '{}'", trim_explicit(&s));

    // This ALSO works — two inputs but only one is a reference:
    fn skip_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
        s.strip_prefix(prefix).unwrap_or(s)
    }
    // ↑ 'a applies to s's output; prefix's lifetime is independent
    let result = skip_prefix("hello world", "hello ");
    println!("after skip: '{result}'");
}

fn rule_three_demo() {
    println!("\n=== Rule 3: &self lifetime → assigned to output ===");

    struct StrWrapper {
        content: String,
    }

    impl StrWrapper {
        // Elided — returns reference with lifetime of &self:
        fn as_str(&self) -> &str {
            &self.content
        }

        // Explicit equivalent:
        fn as_str_explicit<'a>(&'a self) -> &'a str {
            &self.content
        }

        // Two string refs + &self — rule 3 means output borrows from self:
        fn combine_with<'a>(&'a self, other: &str) -> String {
            // returns owned String, so lifetime isn't relevant here
            format!("{} {}", self.content, other)
        }
    }

    let w = StrWrapper { content: String::from("Rust") };
    println!("as_str: '{}'", w.as_str());
    println!("explicit: '{}'", w.as_str_explicit());
    println!("combined: '{}'", w.combine_with("rocks"));
}

fn when_you_must_annotate() {
    println!("\n=== When You MUST Annotate ===");

    println!(
        r#"
Elision CANNOT resolve — you must annotate when:
  1. Two or more input refs exist but return borrows from a specific one:
       fn longer<'a>(x: &'a str, y: &str) -> &'a str {{ x }}

  2. A struct holds references:
       struct Excerpt<'a> {{ text: &'a str }}

  3. Generic types with lifetime bounds:
       fn print<'a, T: Display + 'a>(val: &'a T) {{ ... }}

  4. Trait objects with lifetimes:
       Box<dyn Trait + 'static>
       Box<dyn Trait + '_>  ← inferred from context
"#
    );

    // Example where elision fails — needs explicit annotation:
    fn longer<'a>(x: &'a str, y: &str) -> &'a str {
        // clearly returns x — the lifetime of y is irrelevant
        let _ = y;
        x
    }

    let s1 = String::from("longer string");
    let result;
    {
        let s2 = String::from("short"); // different scope
        result = longer(&s1, &s2); // result borrows from s1, not s2
        println!("result = '{result}'");
    }
    println!("result still valid after s2 dropped: '{result}'");

    // Edition 2024: use<'a> syntax for precise RPIT capturing
    // (return-position impl Trait — captures only specified lifetimes)
    fn make_adder(x: i32) -> impl Fn(i32) -> i32 + use<> {
        // use<> = capture no lifetimes from the environment
        move |y| x + y
    }
    let add5 = make_adder(5);
    println!("5 + 3 = {}", add5(3));
}

// ─── SUMMARY OF ELISION RULES ────────────────────────────────
// Rule 1: fn foo(x: &T)       →  fn foo<'a>(x: &'a T)
// Rule 2: fn foo(x: &T) -> &U →  fn foo<'a>(x: &'a T) -> &'a U
// Rule 3: fn foo(&self) -> &U →  fn foo<'a>(&'a self) -> &'a U

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. C# never requires lifetime annotations — GC manages object lifetime.
// 2. Rust elision means MOST methods (especially &self ones) need no annotation.
// 3. When you see <'a> in library code, it means "borrowed data with this duration."
// 4. The 'static lifetime means the reference can live forever (e.g., string literals).

#[cfg(test)]
mod tests {
    fn first_word(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }

    #[test]
    fn elided_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn longer_only_borrows_first() {
        fn longer<'a>(x: &'a str, _y: &str) -> &'a str { x }
        let a = String::from("hello world");
        let result;
        {
            let b = String::from("hi");
            result = longer(&a, &b);
        }
        assert_eq!(result, "hello world");
    }
}
