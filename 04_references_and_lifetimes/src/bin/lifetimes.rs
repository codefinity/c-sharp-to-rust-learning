// ============================================================
// CONCEPT: Lifetimes
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# references are kept alive by the GC as long as any live reference
// exists. You NEVER worry about a reference outliving its target.
//
// Rust has no GC — so references must NEVER outlive the value they point to.
// The BORROW CHECKER proves this, but sometimes it needs help: lifetime
// annotations tell the compiler how reference lifetimes relate to each other.
//
// Lifetime annotations are NOT runtime information — they exist only for the
// borrow checker and are erased from the compiled binary.
//
// SYNTAX:
//   fn foo<'a>(x: &'a str) -> &'a str { ... }
//   'a  — lifetime parameter (read: "lifetime a")
//   &'a — a reference that lives at least as long as 'a
//
// MOST CODE DOESN'T NEED EXPLICIT LIFETIMES — lifetime elision handles
// the common cases automatically (see lifetime_elision.rs).
//
// RUN: cargo run --bin lifetimes
// ============================================================

fn main() {
    lifetime_motivation();
    lifetime_in_functions();
    lifetime_in_structs();
    lifetime_in_impl();
    multiple_lifetimes();
    static_lifetime();
}

fn lifetime_motivation() {
    println!("=== Why Lifetimes Exist ===");

    // The borrow checker ensures that this CANNOT happen:
    //
    //   let r;
    //   {
    //       let x = 5;
    //       r = &x;  // ← borrow of x
    //   }            // ← x dropped here
    //   println!("{r}"); // ← r would be a dangling pointer!
    //
    // Rust catches this at compile time. C# can't have dangling refs
    // because the GC keeps objects alive. Without a GC, we need lifetimes.

    // Valid: r and x have the same scope
    let x = 5;
    let r = &x;
    println!("r = {r}"); // safe: x outlives r's use

    println!("\nLifetime annotation is the compiler asking:");
    println!("'How long does each reference need to live?'");
}

fn lifetime_in_functions() {
    println!("\n=== Lifetimes in Functions ===");

    // Problem: which reference does the return value borrow from?
    // The compiler cannot tell without an annotation.
    //
    // fn longer(x: &str, y: &str) -> &str { ... }
    // ↑ error: missing lifetime specifier

    // Solution: annotate that the output lives as long as BOTH inputs
    // (conservatively: as long as the SHORTER of the two):
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let s1 = String::from("long string");
    {
        let s2 = String::from("xyz");
        let result = longest(s1.as_str(), s2.as_str());
        println!("longest: '{result}'"); // fine: result used within s2's scope
    }

    // This would NOT compile:
    // let result;
    // {
    //     let s2 = String::from("xyz");
    //     result = longest(s1.as_str(), s2.as_str());
    // }
    // println!("{result}"); // s2 dropped, result might dangle

    // lifetime 'a is resolved to the SHORTER of the two input lifetimes
    println!("longest annotation resolved at call site, not definition");
}

fn lifetime_in_structs() {
    println!("\n=== Lifetimes in Structs ===");

    // A struct that holds a reference must declare a lifetime parameter.
    // This tells the compiler: "this struct cannot outlive the data it borrows."
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 { 3 }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            // Lifetime elision: return borrows from self, not announcement
            println!("Attention everyone: {announcement}!");
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("excerpt: '{}'", excerpt.part);
    println!("level: {}", excerpt.level());
    let part = excerpt.announce_and_return_part("important news");
    println!("returned part: '{part}'");

    // excerpt cannot outlive novel — compiler enforces this
}

fn lifetime_in_impl() {
    println!("\n=== Lifetimes in impl Blocks ===");

    // The 'a lifetime is declared on impl and repeated on the type.
    // When methods don't involve the struct's lifetime explicitly,
    // the lifetime is still there but often elided.

    struct Wrapper<'a> {
        value: &'a i32,
    }

    impl<'a> Wrapper<'a> {
        fn get(&self) -> &i32 {
            // Elided: self has lifetime 'a, so return borrows from 'a
            self.value
        }

        // Explicit version of the same:
        fn get_explicit(&self) -> &'a i32 {
            self.value
        }
    }

    let x = 42;
    let w = Wrapper { value: &x };
    println!("wrapped value: {}", w.get());
    println!("explicit: {}", w.get_explicit());
}

fn multiple_lifetimes() {
    println!("\n=== Multiple Lifetime Parameters ===");

    // Sometimes return borrows from only ONE of several inputs.
    // Annotate precisely to express the relationship.

    fn first_word<'a>(sentence: &'a str) -> &'a str {
        sentence.split_whitespace().next().unwrap_or("")
    }

    // A function with two independent lifetimes:
    fn print_pair<'a, 'b>(x: &'a str, y: &'b str) {
        // Neither return value — no output lifetime needed
        println!("'{x}' and '{y}'");
    }

    let s1 = String::from("hello world");
    let word = first_word(&s1);
    println!("first word: '{word}'");

    let s2 = String::from("goodbye");
    print_pair(&s1, &s2);

    // Lifetime subtyping ('a: 'b means 'a outlives 'b):
    fn longer_verbose<'a: 'b, 'b>(x: &'a str, y: &'b str) -> &'b str {
        // 'a must outlive 'b, so returning x as &'b str is safe
        if x.len() > y.len() { x } else { y }
    }
    println!("{}", longer_verbose(&s1, &s2));
}

fn static_lifetime() {
    println!("\n=== 'static Lifetime ===");

    // 'static means the reference is valid for the ENTIRE program lifetime.
    // String literals are 'static because they're in the binary's data section.

    let s: &'static str = "I live for the whole program";
    println!("{s}");

    // 'static in generic bounds means the type contains no non-'static refs:
    fn must_be_static<T: 'static>(val: T) -> T { val }

    let result = must_be_static(42_i32); // i32: 'static
    let result2 = must_be_static(String::from("owned")); // String: 'static (no refs)
    println!("static: {result} '{result2}'");

    // ⚠️ 'static in error messages often means the compiler wants an
    // OWNED type rather than a reference. "T: 'static" does NOT mean
    // you must leak memory — it means T owns all its data.
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. C# prevents dangling refs via GC; Rust via compile-time lifetime checks.
// 2. Lifetime annotations are purely compile-time — zero runtime cost.
// 3. 'static means "lives as long as the program" — string literals are 'static.
// 4. Most code doesn't need explicit lifetimes — elision rules handle it.
// 5. Lifetimes express RELATIONSHIPS between reference durations, not absolute durations.

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Adding 'static everywhere when the compiler asks for lifetimes —
//   this often means you should use an owned type (String) instead.
// • Confusing lifetimes with scopes — 'a is a minimum duration, not a scope.
// • Fighting the borrow checker: if it seems wrong, use .clone() temporarily
//   and think about the actual ownership structure.

#[cfg(test)]
mod tests {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    #[test]
    fn longest_picks_longer() {
        let s1 = "hello world";
        let s2 = "hi";
        assert_eq!(longest(s1, s2), "hello world");
    }

    #[test]
    fn longest_tie() {
        let s1 = "abc";
        let s2 = "xyz";
        // equal length — returns second (y wins when lengths equal)
        assert_eq!(longest(s1, s2), s2);
    }

    #[test]
    fn static_str_is_static() {
        let s: &'static str = "always alive";
        assert_eq!(s.len(), 12);
    }
}
