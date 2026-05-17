// ============================================================
// CONCEPT: Box<T> — Heap Allocation
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C#, all reference types are heap-allocated by default.
// In Rust, values are stack-allocated by default.
// Box<T> explicitly moves a value to the heap.
//
// Use Box<T> when:
//   1. Type is too large for the stack
//   2. You need a known-size representation of a recursive type
//   3. You want to use trait objects (Box<dyn Trait>)
//   4. You're transferring ownership of a large value cheaply (pointer copy)
//
// C# analogy: every `new MyClass()` is like `Box::new(MyStruct { ... })`
//
// RUN: cargo run --bin box_pointer
// ============================================================

fn main() {
    basic_box();
    recursive_types();
    box_trait_objects();
    box_vs_stack();
}

fn basic_box() {
    println!("=== Box<T> Basics ===");

    // Simple heap allocation
    let boxed: Box<i32> = Box::new(5);
    println!("boxed = {}", *boxed);       // deref to get i32
    println!("boxed = {boxed}");          // auto-deref for Display

    let boxed_str: Box<str> = "hello".into();
    println!("boxed_str = {boxed_str}");

    // Box<T> is a smart pointer:
    println!("size of Box<i32>: {} bytes", std::mem::size_of::<Box<i32>>()); // 8 (pointer)
    println!("size of i32:      {} bytes", std::mem::size_of::<i32>());       // 4

    // Deref coercion: &Box<T> → &T
    fn print_value(x: &i32) { println!("value: {x}"); }
    print_value(&boxed); // &Box<i32> coerces to &i32

    // Box is dropped (memory freed) when it goes out of scope
    {
        let _large = Box::new([0_u8; 1024 * 1024]); // 1MB on heap
        println!("1MB allocated on heap");
    } // freed here — no GC needed
    println!("1MB freed");
}

fn recursive_types() {
    println!("\n=== Recursive Types (need Box) ===");

    // C# classes can have self-referential fields naturally.
    // In Rust, a recursive struct would have INFINITE size at compile time.
    // Box<T> breaks the recursion by making the size = pointer size.

    // WITHOUT Box — does NOT compile:
    // enum List { Cons(i32, List), Nil }  // ← error: infinite size

    // WITH Box — works:
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> Self { List::Nil }

        fn prepend(self, val: i32) -> Self {
            List::Cons(val, Box::new(self))
        }

        fn len(&self) -> usize {
            match self {
                List::Nil        => 0,
                List::Cons(_, t) => 1 + t.len(),
            }
        }

        fn sum(&self) -> i32 {
            match self {
                List::Nil        => 0,
                List::Cons(v, t) => v + t.sum(),
            }
        }
    }

    let list = List::new()
        .prepend(5)
        .prepend(3)
        .prepend(1);

    println!("list len: {}", list.len());
    println!("list sum: {}", list.sum());

    // Binary tree:
    #[derive(Debug)]
    enum BTree {
        Leaf,
        Node { val: i32, left: Box<BTree>, right: Box<BTree> },
    }

    let tree = BTree::Node {
        val: 4,
        left: Box::new(BTree::Node {
            val: 2,
            left: Box::new(BTree::Leaf),
            right: Box::new(BTree::Leaf),
        }),
        right: Box::new(BTree::Node {
            val: 6,
            left: Box::new(BTree::Leaf),
            right: Box::new(BTree::Leaf),
        }),
    };
    println!("tree: {tree:?}");
}

fn box_trait_objects() {
    println!("\n=== Box<dyn Trait> — Heap-Allocated Trait Objects ===");

    trait Drawable {
        fn draw(&self) -> String;
    }

    struct Circle   { radius: f64 }
    struct Square   { side: f64   }

    impl Drawable for Circle { fn draw(&self) -> String { format!("○ r={:.1}", self.radius) } }
    impl Drawable for Square { fn draw(&self) -> String { format!("□ s={:.1}", self.side) } }

    // Heterogeneous Vec — requires Box<dyn Trait> for heap storage:
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Square { side: 2.0 }),
        Box::new(Circle { radius: 1.5 }),
    ];

    for shape in &shapes {
        println!("  {}", shape.draw());
    }
}

fn box_vs_stack() {
    println!("\n=== Box vs Stack Performance ===");
    println!(
        r#"
Stack allocation: O(1), zero overhead
  let x = [0_u8; 64]; // always stack if it fits

Box (heap allocation): involves allocator, small overhead
  let x = Box::new([0_u8; 64]); // heap — pointer deref on access

When to use Box:
  ✓ Recursive types (List, Tree)
  ✓ Trait objects (Box<dyn Fn() -> ()>)
  ✓ Large values to avoid expensive stack copies
  ✓ FFI with C (heap ptr is easier to pass)

C# comparison:
  All class instances → C# Box::new() equivalent
  Value types (struct) → Rust default stack allocation
  Boxing value types → Box<T> in Rust
"#
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn box_deref() {
        let b = Box::new(42_i32);
        assert_eq!(*b, 42);
    }

    #[test]
    fn recursive_list_len() {
        #[derive(Debug)]
        enum List { Cons(i32, Box<List>), Nil }
        impl List {
            fn len(&self) -> usize {
                match self { List::Nil => 0, List::Cons(_, t) => 1 + t.len() }
            }
        }
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        assert_eq!(list.len(), 2);
    }
}
