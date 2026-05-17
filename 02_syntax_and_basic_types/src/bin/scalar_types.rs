// ============================================================
// CONCEPT: Scalar Types (integers, floats, bool, char)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust's scalar types map closely to C# primitives but with key differences:
//   • Integer overflow is a compile-time/runtime distinction (not silent)
//   • char is a Unicode scalar (4 bytes), not UTF-16 (2 bytes)
//   • No implicit numeric conversions — you must use `as` or `From`/`Into`
//   • isize/usize depend on pointer width (like IntPtr/UIntPtr in C#)
//
// C# → Rust type mapping:
//   sbyte   → i8       byte   → u8
//   short   → i16      ushort → u16
//   int     → i32      uint   → u32
//   long    → i64      ulong  → u64
//   Int128  → i128     UInt128→ u128
//   nint    → isize    nuint  → usize
//   float   → f32      double → f64
//   bool    → bool
//   char    → NO direct equivalent (Rust char = Unicode scalar, 4 bytes)
//
// RUN: cargo run --bin scalar_types
// ============================================================

fn main() {
    integer_types();
    float_types();
    boolean_type();
    char_type();
    overflow_behaviour();
    numeric_conversions();
}

fn integer_types() {
    println!("=== Integer Types ===");

    // Signed: i8, i16, i32, i64, i128, isize
    // Unsigned: u8, u16, u32, u64, u128, usize
    let a: i8   = -128_i8;
    let b: u8   = 255_u8;
    let c: i32  = 2_147_483_647;
    let d: u64  = 18_446_744_073_709_551_615_u64;
    let e: i128 = i128::MAX;
    let f: usize = usize::MAX; // pointer-sized (8 bytes on 64-bit)

    println!("i8::MIN  = {}", i8::MIN);
    println!("i8::MAX  = {}", i8::MAX);
    println!("u8::MAX  = {}", u8::MAX);
    println!("i32::MAX = {}", i32::MAX);
    println!("a={a} b={b} c={c}");
    println!("d={d} e={e} f={f}");

    // Literal suffixes and bases
    let decimal     = 98_222;       // visual separator
    let hex         = 0xff;
    let octal       = 0o77;
    let binary      = 0b1111_0000;
    let byte: u8    = b'A';         // byte literal — ASCII value of 'A' (65)
    println!("dec={decimal} hex={hex} oct={octal} bin={binary} byte={byte}");
}

fn float_types() {
    println!("\n=== Float Types ===");

    // f32 (C# float) and f64 (C# double)
    // Default float literal is f64.
    let x: f64 = 2.0;
    let y: f32 = 3.0_f32;
    println!("f64: {x}  f32: {y}");

    // Special values
    println!("f64 NaN:  {}", f64::NAN);
    println!("f64 Inf:  {}", f64::INFINITY);
    println!("f64 -Inf: {}", f64::NEG_INFINITY);
    println!("NaN == NaN: {}", f64::NAN == f64::NAN); // false! same as C#

    // Standard math ops
    let sqrt2 = 2.0_f64.sqrt();
    let pi    = std::f64::consts::PI;
    println!("sqrt(2) = {sqrt2:.6}");
    println!("pi      = {pi:.10}");
    println!("sin(pi) = {:.6}", pi.sin());

    // Precision gotcha (same as C#):
    println!("0.1 + 0.2 = {}", 0.1_f64 + 0.2_f64); // ~0.30000000000000004
}

fn boolean_type() {
    println!("\n=== Boolean Type ===");

    let t: bool = true;
    let f: bool = false;
    println!("AND: {} OR: {} NOT: {}", t && f, t || f, !t);

    // Booleans are NOT integers in Rust (unlike C where true==1).
    // `if 1 { }` is a compile error.
    // But you can convert: `true as u8` → 1_u8
    println!("true as u8 = {}", true as u8);
    println!("false as u8 = {}", false as u8);

    // Short-circuit evaluation works the same as C#:
    let mut n = 0;
    let _ = false && { n += 1; true }; // right side never evaluated
    println!("n after short-circuit = {n}"); // 0
}

fn char_type() {
    println!("\n=== Char Type ===");

    // Rust `char` is a Unicode scalar value (U+0000 – U+D7FF, U+E000 – U+10FFFF).
    // It is ALWAYS 4 bytes on the stack.
    // C# `char` is a UTF-16 code unit (2 bytes) — cannot represent all codepoints.

    let letter  = 'A';
    let emoji   = '🦀'; // Rust mascot — valid char!
    let chinese = '中';
    let null    = '\0';

    println!("letter  = {letter}  ({}  bytes)", std::mem::size_of::<char>());
    println!("emoji   = {emoji}");
    println!("chinese = {chinese}");
    println!("null    = {:?}", null);

    // Char methods
    println!("is alphabetic: {}", letter.is_alphabetic());
    println!("is digit:      {}", '9'.is_ascii_digit());
    println!("to uppercase:  {}", 'a'.to_ascii_uppercase());
    println!("unicode value: U+{:04X}", emoji as u32);

    // ⚠️ C# char vs Rust char
    // C# can store only BMP chars in a single char (U+0000–U+FFFF).
    // Rust char covers all of Unicode (U+0000–U+10FFFF excl. surrogates).
    // Rust strings are UTF-8, not UTF-16.
}

fn overflow_behaviour() {
    println!("\n=== Integer Overflow ===");

    // In debug builds:  overflow PANICS at runtime
    // In release builds: overflow WRAPS (two's complement)
    // This is configurable via `overflow-checks = true/false` in Cargo.toml

    // Safe alternatives (always explicit):
    let x: u8 = 255;
    println!("wrapping_add: {}", x.wrapping_add(1)); // 0
    println!("saturating_add: {}", x.saturating_add(1)); // 255 (clamped)
    println!("checked_add: {:?}", x.checked_add(1));     // None (overflow)
    println!("overflowing_add: {:?}", x.overflowing_add(1)); // (0, true)

    // C# equivalent: unchecked { x + 1 } / checked { x + 1 }
    // Rust makes the semantics explicit per call site, not per block.
}

fn numeric_conversions() {
    println!("\n=== Numeric Conversions ===");

    // NO implicit numeric conversion in Rust — unlike C# which widens i32→i64 etc.
    let x: i32 = 100;

    // `as` is an explicit cast — can truncate or change sign.
    let y = x as i64;   // widening — safe
    let z = x as i8;    // truncating — may lose data (100 fits in i8, ok here)
    println!("i32({x}) as i64 = {y}  as i8 = {z}");

    // `From`/`Into` are infallible, type-checked widening:
    let a: i64 = i64::from(x);  // always safe: i32 → i64
    let b: i64 = x.into();      // same via Into
    println!("From: {a}  Into: {b}");

    // `TryFrom`/`TryInto` for fallible narrowing:
    let big: i64 = 300;
    match i8::try_from(big) {
        Ok(v)  => println!("fits in i8: {v}"),
        Err(e) => println!("overflow: {e}"),
    }
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. No implicit widening — `i32 + i64` is a compile error.
// 2. `char` is 4 bytes (Unicode scalar), C# char is 2 bytes (UTF-16).
// 3. Integer overflow panics in debug, wraps in release by default.
// 4. Use `from`/`into` for safe conversions, `as` for explicit casts.
// 5. Default integer type is `i32`, default float is `f64`.

#[cfg(test)]
mod tests {
    #[test]
    fn wrapping_add_wraps_u8() {
        assert_eq!(255_u8.wrapping_add(1), 0);
    }

    #[test]
    fn checked_add_returns_none_on_overflow() {
        assert_eq!(255_u8.checked_add(1), None);
    }

    #[test]
    fn char_is_four_bytes() {
        assert_eq!(std::mem::size_of::<char>(), 4);
    }

    #[test]
    fn from_conversion_widens() {
        let x: i32 = 1_000;
        let y: i64 = i64::from(x);
        assert_eq!(y, 1_000_i64);
    }

    #[test]
    fn try_from_fails_for_overflow() {
        assert!(i8::try_from(200_i32).is_err());
    }
}
