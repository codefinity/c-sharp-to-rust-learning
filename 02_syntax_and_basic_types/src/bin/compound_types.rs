// ============================================================
// CONCEPT: Compound Types — Tuples and Arrays
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# has ValueTuple<T1,T2,...> and fixed-size arrays. Rust has:
//   • Tuples:  (T1, T2, ...) — heterogeneous, fixed size, stack-allocated
//   • Arrays:  [T; N]        — homogeneous, fixed size, stack-allocated
//   • Slices:  [T]           — dynamically-sized view (like Span<T> in C#)
//
// Vec<T> is covered in module 08 (Collections).
//
// RUN: cargo run --bin compound_types
// ============================================================

fn main() {
    tuples_demo();
    arrays_demo();
    slices_demo();
    patterns_in_compounds();
}

fn tuples_demo() {
    println!("=== Tuples ===");

    // C#: (int x, string name) t = (42, "Alice");
    // Rust:
    let t: (i32, &str, f64) = (42, "Alice", 3.14);

    // Access by index using dot notation:
    println!("t.0 = {}  t.1 = {}  t.2 = {}", t.0, t.1, t.2);

    // Destructure:
    let (id, name, score) = t;
    println!("id={id} name={name} score={score}");

    // Unit type `()` — the empty tuple. It's the return type of functions
    // that don't return a value (like `void` in C#, but it IS a value).
    let unit: () = ();
    println!("unit = {unit:?}"); // ()

    // Nested tuples:
    let nested = ((1_i32, 2_i32), (3_i32, 4_i32));
    println!("nested.0.0 = {}", nested.0.0);

    // Returning multiple values from a function (like C# out params or ValueTuple):
    let (min, max) = min_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
    println!("min={min} max={max}");
}

fn min_max(data: &[i32]) -> (i32, i32) {
    let min = *data.iter().min().expect("empty slice");
    let max = *data.iter().max().expect("empty slice");
    (min, max)
}

fn arrays_demo() {
    println!("\n=== Arrays ===");

    // Arrays have a FIXED size known at compile time: [T; N]
    // C#: int[] arr = {1, 2, 3}; — heap-allocated, length stored separately
    // Rust: let arr = [1, 2, 3]; — stack-allocated when inside a function

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr    = {arr:?}");
    println!("len    = {}", arr.len());
    println!("arr[2] = {}", arr[2]);

    // Repeat syntax — like new int[5] { 0, 0, 0, 0, 0 }:
    let zeros: [i32; 5] = [0; 5];
    println!("zeros  = {zeros:?}");

    // 2D array:
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    for row in &matrix {
        println!("{row:?}");
    }

    // Out-of-bounds access panics at runtime (debug + release):
    // let _ = arr[10]; // thread 'main' panicked: index out of bounds

    // Safe bounds check:
    match arr.get(10) {
        Some(v) => println!("got {v}"),
        None    => println!("index out of bounds — handled safely"),
    }

    // Arrays implement Copy if T: Copy
    let copy_arr = arr; // arr is still usable because i32: Copy
    println!("copy_arr = {copy_arr:?}");
}

fn slices_demo() {
    println!("\n=== Slices ===");

    // A slice [T] is a *view* into a contiguous sequence — like Span<T> in C#.
    // You almost always use them behind a reference: &[T] or &mut [T].

    let arr = [10, 20, 30, 40, 50];

    // Borrow the whole array as a slice:
    let slice: &[i32] = &arr;
    println!("slice = {slice:?}");

    // Sub-slice (like arr[1..3] in C#):
    let middle = &arr[1..4]; // indices 1, 2, 3
    println!("middle = {middle:?}");

    // Functions that accept &[T] work on arrays, Vecs, and other contiguous types:
    println!("sum = {}", sum(slice));
    println!("sum(middle) = {}", sum(middle));

    // Mutable slice:
    let mut data = [3, 1, 4, 1, 5];
    let ms: &mut [i32] = &mut data;
    ms.sort();
    println!("sorted = {ms:?}");

    // String slices (&str) are slices of UTF-8 bytes — covered in strings.rs.
}

fn sum(slice: &[i32]) -> i32 {
    // &[T] is a fat pointer: pointer + length — like a (T*, int) pair in C.
    // This function accepts arrays, Vecs, or any contiguous [i32].
    slice.iter().sum()
}

fn patterns_in_compounds() {
    println!("\n=== Patterns in Compound Types ===");

    // Pattern matching on arrays/slices:
    let arr = [1, 2, 3];
    match arr {
        [1, ..]        => println!("starts with 1"),
        [_, 2, _]      => println!("middle is 2"),
        _              => println!("something else"),
    }

    // Rest patterns in slices:
    let data = [10, 20, 30, 40, 50];
    if let [first, .., last] = data {
        println!("first={first} last={last}");
    }

    // Tuple struct-like patterns:
    let point = (0_i32, 5_i32);
    match point {
        (0, y) => println!("on Y axis at y={y}"),
        (x, 0) => println!("on X axis at x={x}"),
        (x, y) => println!("at ({x}, {y})"),
    }
}

// ─── KEY DIFFERENCES FROM C# ─────────────────────────────────
// 1. Arrays [T; N] are stack-allocated value types (like C# stackalloc).
// 2. Slices &[T] are fat pointers (ptr + len) — like Span<T> but zero-cost.
// 3. Out-of-bounds panics; use `.get(i)` for safe access returning Option<&T>.
// 4. No jagged vs rectangular distinction — use [[T; N]; M] for 2D.
// 5. Tuples use `.0`, `.1` for access, not named fields.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_works() {
        let (min, max) = min_max(&[5, 3, 8, 1]);
        assert_eq!(min, 1);
        assert_eq!(max, 8);
    }

    #[test]
    fn sum_of_slice() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn array_get_returns_none() {
        let arr = [1, 2, 3];
        assert_eq!(arr.get(10), None);
    }

    #[test]
    fn sub_slice() {
        let arr = [10, 20, 30, 40, 50];
        assert_eq!(&arr[1..4], &[20, 30, 40]);
    }
}
