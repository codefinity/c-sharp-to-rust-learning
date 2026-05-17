// ============================================================
// CONCEPT: Strings — String, &str, slices, UTF-8
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has ONE string type (System.String — immutable, UTF-16, heap-allocated).
// Rust has TWO main string types:
//   • &str   — immutable string SLICE (borrowed view into UTF-8 bytes)
//              ≈ ReadOnlySpan<char> but for UTF-8 bytes
//   • String — owned, growable, heap-allocated UTF-8 string
//              ≈ System.Text.StringBuilder or string you own
//
// Both types store UTF-8, not UTF-16. This means a char in Rust is 1-4 bytes.
// Indexing by byte position is allowed; indexing by character is NOT (O(n)).
//
// RUN: cargo run --bin strings
// ============================================================

fn main() {
    str_slices();
    owned_strings();
    string_slicing();
    string_operations();
    utf8_handling();
    string_conversions();
}

fn str_slices() {
    println!("=== &str (string slices) ===");

    // String literals are &'static str — a reference into the binary's
    // read-only data section. Their lifetime is the entire program.
    let greeting: &str = "Hello, Rust!";
    println!("{greeting}");
    println!("len (bytes) = {}", greeting.len()); // 12 bytes

    // Functions naturally take &str — works for both literals and String refs:
    print_length(greeting);
    print_length(&String::from("dynamic")); // &String coerces to &str

    // &str is a fat pointer: pointer + byte-length
    println!("size_of::<&str> = {}", std::mem::size_of::<&str>()); // 16 on 64-bit
}

fn print_length(s: &str) {
    println!("'{s}' has {} bytes", s.len());
}

fn owned_strings() {
    println!("\n=== String (owned) ===");

    // String::new() — empty, growable
    let mut s = String::new();
    s.push_str("Hello");    // append string slice
    s.push(',');             // append a single char
    s.push_str(" world!");
    println!("{s}");

    // String::from() / .to_string() / format!()
    let a = String::from("foo");
    let b = "bar".to_string();
    let c = format!("{a} {b}");  // like C# $"{a} {b}" — returns new String
    println!("{c}");

    // + operator takes ownership of left, borrows right:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is MOVED here — no longer usable
    println!("{s3}");
    // println!("{s1}"); // ← compile error: s1 was moved

    // Prefer format! when you need multiple strings without moving:
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let game = format!("{a}-{b}-{c}"); // a, b, c all still valid
    println!("{game}");
}

fn string_slicing() {
    println!("\n=== String Slicing ===");

    let s = String::from("hello world");

    // Slicing returns &str — byte-based ranges
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    // Words iterator (splits on whitespace):
    for word in s.split_whitespace() {
        println!("word: {word}");
    }

    // Characters iterator (Unicode-correct, O(n)):
    let chars: Vec<char> = "café".chars().collect();
    println!("chars: {chars:?}");
    println!("char count: {}", "café".chars().count());
    println!("byte count: {}", "café".len()); // 5 bytes ('é' = 2 bytes UTF-8)

    // ⚠️ Cannot index directly: let ch = s[0]; // ← compile error
    // Use .chars().nth(n) for character access (O(n)):
    let nth = "hello".chars().nth(1);
    println!("2nd char: {nth:?}");
}

fn string_operations() {
    println!("\n=== Common String Operations ===");

    let s = "  Hello, World!  ".to_string();

    // Trimming
    println!("trim:  '{}'", s.trim());
    println!("trim_start: '{}'", s.trim_start());

    // Case
    println!("to_uppercase: {}", "hello".to_uppercase()); // Unicode-aware
    println!("to_lowercase: {}", "HELLO".to_lowercase());

    // Contains / starts_with / ends_with
    let haystack = "Rust programming";
    println!("contains 'Rust': {}", haystack.contains("Rust"));
    println!("starts_with 'Rust': {}", haystack.starts_with("Rust"));
    println!("ends_with 'ing': {}", haystack.ends_with("ing"));

    // Replace
    let replaced = "foo bar foo".replace("foo", "baz");
    println!("replaced: {replaced}");

    // Split and collect
    let csv = "a,b,c,d";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("split: {parts:?}");

    // Join
    let joined = parts.join(" | ");
    println!("joined: {joined}");

    // Parse (like int.Parse in C#)
    let n: i32 = "42".parse().expect("not a number");
    println!("parsed: {n}");

    // Repeat
    let repeated = "ab".repeat(3);
    println!("repeated: {repeated}"); // "ababab"
}

fn utf8_handling() {
    println!("\n=== UTF-8 Handling ===");

    let emoji_str = "Hello 🦀 World";
    println!("String: {emoji_str}");
    println!("Byte length: {}", emoji_str.len());              // bytes
    println!("Char count: {}", emoji_str.chars().count());     // unicode scalars

    // Bytes iterator:
    let first_5_bytes: Vec<u8> = emoji_str.bytes().take(5).collect();
    println!("First 5 bytes: {first_5_bytes:?}");

    // Slicing in the middle of a multibyte char PANICS — Rust prevents it:
    // let bad = &emoji_str[0..7]; // would panic if emoji boundary is not respected
    // Safe way: check char boundaries with is_char_boundary()
    let safe_end = emoji_str.char_indices()
        .nth(5)
        .map(|(i, _)| i)
        .unwrap_or(emoji_str.len());
    println!("First 5 chars as slice: '{}'", &emoji_str[..safe_end]);

    // Converting bytes to String:
    let bytes = b"hello";  // &[u8; 5]
    let from_bytes = std::str::from_utf8(bytes).expect("valid UTF-8");
    println!("from bytes: {from_bytes}");
}

fn string_conversions() {
    println!("\n=== String Conversions ===");

    // i32 → String
    let n = 42_i32;
    let s = n.to_string();
    println!("i32 to String: {s}");

    // String → i32 (fallible)
    let back: i32 = s.parse().unwrap();
    println!("String to i32: {back}");

    // &str → String (cloning / owning)
    let borrowed: &str = "hello";
    let owned: String = borrowed.to_string();         // or String::from(borrowed)
    let owned2: String = borrowed.to_owned();
    println!("owned: {owned} owned2: {owned2}");

    // String → &str (borrowing — zero cost)
    let s = String::from("world");
    let slice: &str = &s;          // Deref coercion: String → &String → &str
    let slice2: &str = s.as_str(); // explicit
    println!("slice: {slice} slice2: {slice2}");
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. UTF-8 (not UTF-16) — byte length ≠ character count for non-ASCII.
// 2. Two types: &str (borrowed view) vs String (owned value).
// 3. Cannot index by character position — use .chars().nth(n).
// 4. `+` for concatenation moves the left operand — use format! instead.
// 5. `String` is re-allocated on push when capacity is exceeded (like StringBuilder).

// ─── COMMON MISTAKES ─────────────────────────────────────────
// • Using s[0] expecting a char — you get a compile error; use s.chars().next()
// • Slicing in the middle of a multibyte character — causes runtime panic
// • Comparing &str and String — use == (it works via PartialEq impl)
// • Forgetting that .len() returns bytes, not characters

#[cfg(test)]
mod tests {
    #[test]
    fn str_len_is_bytes() {
        assert_eq!("café".len(), 5); // 'é' = 2 bytes
        assert_eq!("café".chars().count(), 4);
    }

    #[test]
    fn string_contains() {
        assert!("Hello, world!".contains("world"));
    }

    #[test]
    fn parse_roundtrip() {
        let n: i32 = "123".parse().unwrap();
        assert_eq!(n.to_string(), "123");
    }

    #[test]
    fn split_and_join() {
        let parts: Vec<&str> = "a,b,c".split(',').collect();
        assert_eq!(parts, vec!["a", "b", "c"]);
        assert_eq!(parts.join("-"), "a-b-c");
    }

    #[test]
    fn borrowed_equals_owned() {
        let owned = String::from("hello");
        let borrowed: &str = "hello";
        assert_eq!(owned, borrowed); // PartialEq<str> for String
    }
}
