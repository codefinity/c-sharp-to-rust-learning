// ============================================================
// CONCEPT: FFI — Calling C from Rust (and exposing Rust to C)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses P/Invoke (DllImport attribute) to call native code:
//   [DllImport("libc.so.6")]
//   static extern int abs(int n);
//
// Rust uses extern "C" blocks:
//   extern "C" { fn abs(n: libc::c_int) -> libc::c_int; }
//
// Rust can ALSO expose functions for C to call using #[no_mangle]
// and extern "C", making Rust an alternative to writing C libraries.
//
// RUN: cargo run --bin ffi_basics
// ============================================================

use libc::{c_char, c_double, c_int};
use std::ffi::{CStr, CString};

fn main() {
    println!("=== FFI and Interop ===\n");

    calling_c_functions();
    c_strings();
    c_types_mapping();
    exposing_rust_to_c();
    safe_wrappers();
}

// ---- 1. Calling C standard library functions -----------------------

// Declare C functions in an extern "C" block.
// Edition 2024: extern blocks must be marked unsafe.
// The linker resolves these at link time.
unsafe extern "C" {
    fn abs(n: c_int) -> c_int;
    fn sqrt(n: c_double) -> c_double;
    fn strlen(s: *const c_char) -> libc::size_t;
    fn puts(s: *const c_char) -> c_int;
}

fn calling_c_functions() {
    println!("--- Calling C Standard Library Functions ---");

    // SAFETY: abs is a pure C math function with no side effects or aliasing
    let result = unsafe { abs(-42) };
    println!("C abs(-42) = {result}");

    let root = unsafe { sqrt(144.0) };
    println!("C sqrt(144.0) = {root}");

    // strlen with a CString:
    let cstr = CString::new("hello").unwrap();
    let len = unsafe { strlen(cstr.as_ptr()) };
    println!("C strlen(\"hello\") = {len}");

    // puts — C puts() adds a newline:
    let greeting = CString::new("  Hello from C's puts()!").unwrap();
    unsafe { puts(greeting.as_ptr()); }
}

// ---- 2. CString and CStr — the string bridge -----------------------

fn c_strings() {
    println!("\n--- CString and CStr ---");

    // CString — Rust-owned, NUL-terminated, heap-allocated
    // C# analogy: Marshal.StringToHGlobalAnsi / Marshal.StringToCoTaskMemAnsi

    let rust_str: &str = "Hello, C world!";

    // Rust &str → CString (allocates, appends NUL):
    let cstring: CString = CString::new(rust_str)
        .expect("CString::new failed: interior NUL byte");

    println!("CString ptr: {:p}", cstring.as_ptr());
    println!("CString len (excl NUL): {}", cstring.as_bytes().len());

    // &CStr — borrowed view into a NUL-terminated C string
    // C# analogy: ReadOnlySpan<byte> of an ANSI string

    // From a CString:
    let cstr: &CStr = cstring.as_c_str();
    println!("CStr → &str: {:?}", cstr.to_str().unwrap());

    // From a raw pointer (returned by a C function):
    let raw: *const c_char = cstring.as_ptr();
    // SAFETY: raw points to our own CString, which is still alive
    let borrowed: &CStr = unsafe { CStr::from_ptr(raw) };
    println!("from_ptr → to_string_lossy: {}", borrowed.to_string_lossy());

    // Strings with NUL bytes panic in CString::new — handle gracefully:
    match CString::new("has\0nul") {
        Ok(_)  => println!("no NUL"),
        Err(e) => println!("CString::new error: {e}"),
    }
}

// ---- 3. C ↔ Rust type mapping --------------------------------------

fn c_types_mapping() {
    println!("\n--- C ↔ Rust Type Mapping (via libc) ---");

    // Use libc crate types for cross-platform correctness.
    // C# analogy: using System.Runtime.InteropServices marshalling types.

    let _i: libc::c_int      = 42;           // int          → c_int (usually i32)
    let _l: libc::c_long     = 42;           // long         → c_long (platform-dependent)
    let _d: libc::c_double   = 3.14;         // double       → c_double (f64)
    let _f: libc::c_float    = 3.14;         // float        → c_float (f32)
    let _s: libc::size_t     = 64;           // size_t       → usize
    let _p: libc::ssize_t    = -1;           // ssize_t      → isize
    let _c: libc::c_char     = b'A' as i8;   // char         → c_char (i8 on most platforms)
    let _uc: libc::c_uchar   = b'A';         // unsigned char

    println!(r#"
C type          | Rust type (libc)     | C# equivalent
----------------|----------------------|------------------
int             | c_int     (i32)      | int
long            | c_long    (i32/i64)  | long (platform)
unsigned int    | c_uint    (u32)      | uint
size_t          | size_t    (usize)    | UIntPtr
double          | c_double  (f64)      | double
float           | c_float   (f32)      | float
char*           | *const c_char        | string (Marshal)
void*           | *mut c_void          | IntPtr / void*
bool (C99)      | c_int (0/1)          | bool (marshal as int)
"#);
}

// ---- 4. Exposing Rust functions to C / P/Invoke -------------------

// #[no_mangle]   — keep the function name as-is (no Rust name mangling)
// extern "C"     — use the C calling convention
// These two together make the function callable from C, C++, C#, Python, etc.

#[unsafe(no_mangle)]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_greet(name: *const c_char) -> c_int {
    // SAFETY: caller guarantees name is a valid NUL-terminated C string
    if name.is_null() { return -1; }
    let name_str = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    println!("Hello from Rust, {name_str}!");
    0
}

// For a C# P/Invoke:
//   [DllImport("myrustlib.dll")]
//   static extern int rust_add(int a, int b);
//
//   [DllImport("myrustlib.dll")]
//   static extern int rust_greet([MarshalAs(UnmanagedType.LPStr)] string name);

fn exposing_rust_to_c() {
    println!("\n--- Exposing Rust to C / C# P/Invoke ---");

    // We can call our own #[no_mangle] functions from within Rust too:
    let sum = rust_add(10, 32);
    println!("rust_add(10, 32) = {sum}");

    let name = CString::new("Developer").unwrap();
    rust_greet(name.as_ptr());

    println!(r#"
To use from C#:
  [DllImport("myrustlib")]
  static extern int rust_add(int a, int b);

To build as a cdylib, in Cargo.toml:
  [lib]
  crate-type = ["cdylib"]
"#);
}

// ---- 5. Safe wrappers around FFI -----------------------------------

// The idiomatic pattern: thin unsafe FFI declaration + safe public API.

mod safe_libc {
    use libc::{c_double, c_int};

    unsafe extern "C" {
        fn abs(n: c_int) -> c_int;
        fn pow(base: c_double, exp: c_double) -> c_double;
    }

    pub fn safe_abs(n: i32) -> i32 {
        // SAFETY: abs is pure, always defined, no aliasing
        unsafe { abs(n) }
    }

    pub fn safe_pow(base: f64, exp: f64) -> f64 {
        // SAFETY: pow is pure, defined for all finite inputs
        unsafe { pow(base, exp) }
    }
}

fn safe_wrappers() {
    println!("\n--- Safe Wrappers Over FFI ---");

    println!("safe_abs(-99) = {}", safe_libc::safe_abs(-99));
    println!("safe_pow(2.0, 10.0) = {}", safe_libc::safe_pow(2.0, 10.0));

    println!(r#"
Best practices for FFI:
  1. Keep extern "C" blocks private (not pub)
  2. Write a public safe wrapper that validates inputs
  3. Add "// SAFETY:" comments explaining invariants
  4. Use CString/CStr for strings — never &str directly
  5. Use libc types (c_int, size_t) for cross-platform safety
  6. Mark raw pointer parameters as *const (immutable) where possible
  7. Return error codes or Option/Result from safe wrappers
"#);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::safe_libc::*;

    #[test]
    fn c_abs() {
        assert_eq!(safe_abs(-5), 5);
        assert_eq!(safe_abs(0), 0);
    }

    #[test]
    fn rust_add_ffi() {
        assert_eq!(rust_add(3, 4), 7);
    }

    #[test]
    fn cstring_roundtrip() {
        let s = "hello";
        let cs = CString::new(s).unwrap();
        let back = cs.to_str().unwrap();
        assert_eq!(back, s);
    }
}
