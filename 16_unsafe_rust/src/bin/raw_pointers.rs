// ============================================================
// CONCEPT: Raw Pointers — *const T and *mut T
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# unsafe pointers (int*, void*) are similar in purpose.
// Key differences:
//   • Creating a raw pointer is SAFE — only dereferencing is unsafe
//   • Raw pointers don't have lifetimes — you must ensure validity
//   • Raw pointers are NOT automatically null-safe (no NullReferenceException)
//   • Multiple *mut T to the same memory can exist simultaneously
//     (unlike &mut T — the borrow checker doesn't track raw pointers)
//
// RUN: cargo run --bin raw_pointers
// ============================================================

fn main() {
    println!("=== Raw Pointers ===\n");

    pointer_basics();
    pointer_arithmetic();
    null_pointers();
    box_raw_round_trip();
    linked_list_raw();
}

// ---- 1. Pointer basics ---------------------------------------------

fn pointer_basics() {
    println!("--- Creating and Using Raw Pointers ---");

    let x: i32 = 42;

    // Coerce a reference to a raw pointer — this is SAFE:
    let r: &i32 = &x;
    let ptr: *const i32 = r as *const i32;     // or: &x as *const i32

    // Print the address:
    println!("x lives at: {ptr:p}");
    println!("x value via reference: {r}");

    // Dereference — must be inside unsafe:
    let val = unsafe { *ptr };
    println!("x value via raw pointer: {val}");

    // Mutable raw pointer:
    let mut y: i32 = 100;
    let mp: *mut i32 = &mut y as *mut i32;

    unsafe {
        *mp = 200;
    }
    println!("y after raw write: {y}");

    // Casting between pointer types (like C# (T*)ptr):
    let byte_ptr = ptr as *const u8;
    println!("first byte of x at {:p}", byte_ptr);
}

// ---- 2. Pointer arithmetic -----------------------------------------

fn pointer_arithmetic() {
    println!("\n--- Pointer Arithmetic ---");

    let array: [i32; 5] = [10, 20, 30, 40, 50];
    let base: *const i32 = array.as_ptr();

    // Advance by N elements using .add(n):
    // C# analogy: ptr + n (in unsafe context)
    for i in 0..5 {
        let val = unsafe { *base.add(i) };
        print!("{val} ");
    }
    println!();

    // offset — signed version (can go backward):
    let third: *const i32 = unsafe { base.add(2) };
    let first: *const i32 = unsafe { third.offset(-2) };
    println!("third element: {}", unsafe { *third });
    println!("first via offset(-2): {}", unsafe { *first });

    // wrapping_add — never overflows (wraps on overflow — useful in no-std):
    let _ = base.wrapping_add(1_000_000); // no UB even if out of bounds

    // Distance between pointers:
    // C# analogy: (ptr2 - ptr1) / sizeof(T)
    let last = unsafe { base.add(4) };
    let dist = unsafe { last.offset_from(base) };
    println!("distance base→last: {dist} elements");
}

// ---- 3. Null pointers ----------------------------------------------

fn null_pointers() {
    println!("\n--- Null Pointers ---");

    // Create null pointer (C# analogy: (int*)null):
    let null_ptr: *const i32 = std::ptr::null();
    let null_mut: *mut i32   = std::ptr::null_mut();

    println!("null *const i32: {:p}", null_ptr);
    println!("is_null: {}", null_ptr.is_null());

    // Safe null check before deref:
    unsafe {
        if !null_ptr.is_null() {
            println!("value: {}", *null_ptr);
        } else {
            println!("pointer is null — skipping dereference");
        }
    }

    // as_ref / as_mut — convert raw pointer to Option<&T>:
    let ptr: *const i32 = std::ptr::null();
    let opt: Option<&i32> = unsafe { ptr.as_ref() };
    println!("null.as_ref() = {:?}", opt);

    let val: i32 = 99;
    let non_null: *const i32 = &val;
    let opt2: Option<&i32> = unsafe { non_null.as_ref() };
    println!("non_null.as_ref() = {:?}", opt2);

    // NonNull<T> — a non-nullable raw pointer (thinner than Option<*mut T>):
    let nn = std::ptr::NonNull::new(null_mut);
    println!("NonNull::new(null_mut) = {:?}", nn);

    let val2: i32 = 7;
    let nn2 = std::ptr::NonNull::new(&val2 as *const i32 as *mut i32).unwrap();
    println!("NonNull from valid ptr: {:p}", nn2.as_ptr());
}

// ---- 4. Box::into_raw / Box::from_raw round-trip -------------------

fn box_raw_round_trip() {
    println!("\n--- Box ↔ Raw Pointer Round-Trip ---");

    // Box::into_raw — transfer ownership out of Box; caller is responsible
    // for eventually calling Box::from_raw to reclaim and drop the memory.
    // C# analogy: GCHandle.Alloc + GCHandle.AddrOfPinnedObject (but ownership-based)

    let boxed: Box<String> = Box::new("hello from the heap".to_string());
    let raw: *mut String = Box::into_raw(boxed);
    // `boxed` is gone; raw is a raw pointer we now own

    // We can pass raw to C FFI, store it in a C struct, etc.
    println!("raw ptr: {:p}", raw);
    unsafe { println!("deref: {}", *raw); }

    // Reconstruct Box to properly free memory:
    // SAFETY: raw came from Box::into_raw with the same type
    let recovered: Box<String> = unsafe { Box::from_raw(raw) };
    println!("recovered: {recovered}");
    // recovered is dropped here → memory freed
}

// ---- 5. Intrusive singly-linked list with raw pointers ------------

struct Node {
    value: i32,
    next: *mut Node,
}

struct RawList {
    head: *mut Node,
    len: usize,
}

impl RawList {
    fn new() -> Self {
        RawList { head: std::ptr::null_mut(), len: 0 }
    }

    fn push_front(&mut self, value: i32) {
        let node = Box::into_raw(Box::new(Node {
            value,
            next: self.head,
        }));
        self.head = node;
        self.len += 1;
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut cur = self.head;
        while !cur.is_null() {
            // SAFETY: cur was set from a valid Box allocation or null
            let node = unsafe { &*cur };
            result.push(node.value);
            cur = node.next;
        }
        result
    }
}

impl Drop for RawList {
    fn drop(&mut self) {
        let mut cur = self.head;
        while !cur.is_null() {
            // SAFETY: cur was allocated with Box::into_raw; we own it
            let node = unsafe { Box::from_raw(cur) };
            cur = node.next;
            // `node` (and therefore the Box) is dropped here
        }
    }
}

fn linked_list_raw() {
    println!("\n--- Raw Pointer Linked List ---");

    let mut list = RawList::new();
    for i in 1..=5 {
        list.push_front(i * 10);
    }
    println!("list (front→back): {:?}", list.to_vec());
    println!("len: {}", list.len);
    // list dropped here; all nodes freed via Drop
    println!("list dropped — all nodes freed");
}

// ---- Summary -------------------------------------------------------
//
// *const T  — immutable raw pointer (no aliasing guarantee)
// *mut T    — mutable raw pointer
//
// Safe to do (no unsafe needed):
//   &x as *const T        — create from reference
//   ptr.is_null()         — null check
//   ptr.add(n)            — advance (doesn't dereference)
//   ptr.cast::<U>()       — cast pointer type
//
// Requires unsafe:
//   *ptr                  — dereference
//   ptr.as_ref()          — to Option<&T>
//   ptr.offset_from(base) — signed distance
//   std::ptr::read(ptr)   — read without dropping
//   std::ptr::write(ptr, val) — write without reading
//   std::ptr::copy(src, dst, n) — memcpy
//
// C#/Rust analogy:
//   int* ptr = &x;        ←→ let ptr: *mut i32 = &mut x;
//   *ptr = 42;            ←→ unsafe { *ptr = 42; }
//   ptr++                 ←→ ptr = ptr.add(1);
//   (void*)ptr            ←→ ptr as *mut std::ffi::c_void

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_push_and_read() {
        let mut list = RawList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn box_round_trip() {
        let b = Box::new(42_i32);
        let raw = Box::into_raw(b);
        let val = unsafe { *raw };
        let _ = unsafe { Box::from_raw(raw) };
        assert_eq!(val, 42);
    }
}
