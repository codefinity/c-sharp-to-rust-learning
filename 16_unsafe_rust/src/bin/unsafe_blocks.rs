// ============================================================
// CONCEPT: Unsafe Rust — What It Is and When to Use It
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has `unsafe` blocks for pointer arithmetic.
// Rust's `unsafe` is similar but covers a wider set of operations.
//
// Unsafe Rust does NOT turn off the borrow checker or type system.
// It only unlocks FIVE specific superpowers:
//   1. Dereference a raw pointer (*const T / *mut T)
//   2. Call an unsafe function or extern "C" function
//   3. Access or modify a mutable static variable
//   4. Implement an unsafe trait (Send, Sync)
//   5. Access fields of a union
//
// Key principle: unsafe is a promise to the compiler that YOU have
// verified the invariants. The invariant burden shifts to you.
//
// RUN: cargo run --bin unsafe_blocks
// ============================================================

fn main() {
    println!("=== Unsafe Rust ===\n");

    unsafe_functions();
    mutable_statics();
    unsafe_traits();
    union_demo();
    common_patterns();
}

// ---- 1. unsafe functions and blocks --------------------------------

// An unsafe function is one that requires the caller to uphold invariants.
// C# analogy: a method that accepts IntPtr and docs say "must be valid"
unsafe fn dangerous_divide(a: i32, b: i32) -> i32 {
    // The caller must guarantee b != 0
    a / b
}

// A safe wrapper — the idiomatic pattern:
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        // SAFETY: we just checked b != 0
        Some(unsafe { dangerous_divide(a, b) })
    }
}

fn unsafe_functions() {
    println!("--- Unsafe Functions and Safe Wrappers ---");

    println!("safe_divide(10, 2) = {:?}", safe_divide(10, 2));
    println!("safe_divide(10, 0) = {:?}", safe_divide(10, 0));

    // from_utf8_unchecked — standard library example of unsafe function:
    let bytes: &[u8] = b"valid utf-8";
    // SAFETY: we know this is valid UTF-8
    let s = unsafe { std::str::from_utf8_unchecked(bytes) };
    println!("from_utf8_unchecked: '{s}'");

    // slice::from_raw_parts — building a slice from pointer + length:
    let array: [i32; 5] = [10, 20, 30, 40, 50];
    let ptr = array.as_ptr();
    // SAFETY: ptr is valid, len=3 is in bounds, lifetime tied to array
    let slice = unsafe { std::slice::from_raw_parts(ptr, 3) };
    println!("from_raw_parts slice: {slice:?}");
}

// ---- 2. Mutable static variables -----------------------------------

// Static mut is unsafe because multiple threads could race on it.
// Use Mutex<T> / AtomicXxx in production code instead.
static mut CALL_COUNT: u32 = 0;

fn track_call() {
    // SAFETY: single-threaded program; no concurrent access
    unsafe {
        CALL_COUNT += 1;
    }
}

fn get_call_count() -> u32 {
    // SAFETY: single-threaded; only reading
    unsafe { CALL_COUNT }
}

fn mutable_statics() {
    println!("\n--- Mutable Static Variables ---");

    for _ in 0..5 {
        track_call();
    }
    println!("call count: {}", get_call_count());
    println!("NOTE: prefer AtomicU32 for real concurrent code");
}

// ---- 3. Unsafe traits: Send and Sync --------------------------------

// Send  — the type can be *transferred* to another thread
// Sync  — the type can be *accessed* from multiple threads concurrently
//         (i.e., &T is Send)
//
// Most types are automatically Send/Sync if their fields are.
// Raw pointers (*const T, *mut T) are NOT Send/Sync — you must manually
// implement them if you know your type is thread-safe.

struct MyRawBuf {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: MyRawBuf owns the allocation exclusively (like Box<[u8]>)
unsafe impl Send for MyRawBuf {}
unsafe impl Sync for MyRawBuf {}

impl MyRawBuf {
    fn new(len: usize) -> Self {
        let layout = std::alloc::Layout::array::<u8>(len).unwrap();
        // SAFETY: layout has non-zero size
        let ptr = unsafe { std::alloc::alloc(layout) };
        MyRawBuf { ptr, len }
    }
}

impl Drop for MyRawBuf {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<u8>(self.len).unwrap();
        // SAFETY: ptr was allocated with the same allocator and layout
        unsafe { std::alloc::dealloc(self.ptr, layout) };
    }
}

fn unsafe_traits() {
    println!("\n--- Unsafe Traits (Send + Sync) ---");

    let buf = MyRawBuf::new(64);
    println!("MyRawBuf allocated {} bytes at {:p}", buf.len, buf.ptr);

    let buf = std::sync::Arc::new(buf); // works because we impl Sync
    let buf2 = buf.clone();
    let h = std::thread::spawn(move || {
        println!("  accessed from thread: len={}", buf2.len);
    });
    h.join().unwrap();
    println!("  main thread: len={}", buf.len);
}

// ---- 4. Unions -------------------------------------------------------

// Rust unions are like C unions — all fields share the same memory.
// Accessing a union field is unsafe because you must know which variant
// was last written.
// C# analogy: [StructLayout(LayoutKind.Explicit)] with [FieldOffset(0)]

#[repr(C)]
union FloatBits {
    f: f32,
    bits: u32,
}

fn union_demo() {
    println!("\n--- Unions ---");

    let fb = FloatBits { f: 1.0_f32 };
    // SAFETY: we just wrote the `f` field
    let bits = unsafe { fb.bits };
    println!("1.0_f32 bits: {bits:#010x}  (IEEE 754: sign=0, exp=127, mantissa=0)");

    // Parsing f32 bits: sign | exponent | mantissa
    let sign     = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xFF;
    let mantissa =  bits        & 0x7F_FFFF;
    println!("  sign={sign}, exponent={exponent} (biased), mantissa={mantissa:#x}");
}

// ---- 5. Common safe abstractions over unsafe code ------------------

fn common_patterns() {
    println!("\n--- Patterns: Safe Abstractions Over Unsafe ---");

    println!(r#"
1. The "safe wrapper" pattern
   unsafe fn raw(ptr, len) → ...
   fn safe(slice: &[T]) → ...  // validates, then calls unsafe

2. The "SAFETY:" comment convention
   Always document WHY the unsafe block is sound:
   // SAFETY: ptr is non-null because we got it from Box::into_raw

3. Minimize unsafe scope
   BAD:  unsafe {{ let x = ...; let y = ...; ... 50 lines ... }}
   GOOD: let valid_ptr = unsafe {{ raw_ptr.as_ref().unwrap() }};
         // rest of logic is safe

4. Invariants in documentation
   Document every precondition on unsafe fn using /// # Safety

5. Prefer safe alternatives
   *mut T     → Box<T>, Vec<T>, &mut T
   static mut → Mutex<T>, AtomicUsize, OnceLock<T>
   raw divide → checked_div(), Option<T>
   from_raw   → TryFrom/TryInto with error handling
"#);

    // Demonstrate unsafe for performance: unchecked array access
    let data: Vec<i32> = (0..1000).collect();

    // Safe: bounds-checked every iteration
    let sum_safe: i32 = data.iter().sum();

    // Unsafe: skip bounds check (valid when index is guaranteed in-range)
    let mut sum_unsafe: i32 = 0;
    for i in 0..data.len() {
        // SAFETY: i is in 0..data.len()
        sum_unsafe += unsafe { *data.get_unchecked(i) };
    }

    assert_eq!(sum_safe, sum_unsafe);
    println!("safe sum == unsafe sum: {sum_safe} (get_unchecked skips bounds check)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_divide_works() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn float_bits() {
        let fb = FloatBits { f: 0.0_f32 };
        let bits = unsafe { fb.bits };
        assert_eq!(bits, 0);
    }
}
