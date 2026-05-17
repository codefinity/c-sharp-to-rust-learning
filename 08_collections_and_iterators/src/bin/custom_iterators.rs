// ============================================================
// CONCEPT: Custom Iterators
// ============================================================
// RUN: cargo run --bin custom_iterators
// ============================================================

fn main() {
    basic_custom_iterator();
    fibonacci_iterator();
    tree_iterator();
    generator_style();
}

struct Range2D {
    rows: usize, cols: usize,
    current_row: usize, current_col: usize,
}

impl Range2D {
    fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols, current_row: 0, current_col: 0 }
    }
}

impl Iterator for Range2D {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.current_row >= self.rows { return None; }
        let result = (self.current_row, self.current_col);
        self.current_col += 1;
        if self.current_col >= self.cols {
            self.current_col = 0;
            self.current_row += 1;
        }
        Some(result)
    }
}

fn basic_custom_iterator() {
    println!("=== Custom 2D Range Iterator ===");

    let r2d = Range2D::new(2, 3);
    let points: Vec<_> = r2d.collect();
    println!("2×3 grid: {points:?}");

    // All Iterator adapters are available after implementing Iterator:
    let count = Range2D::new(3, 3).filter(|(r, c)| r == c).count(); // diagonal
    println!("diagonal count in 3×3: {count}");
}

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Self { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a.checked_add(self.b)?; // None on overflow
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}

fn fibonacci_iterator() {
    println!("\n=== Fibonacci Iterator ===");

    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("first 10 fibonacci: {fibs:?}");

    let sum: u64 = Fibonacci::new().take_while(|&n| n < 100).sum();
    println!("sum of fibonacci < 100: {sum}");

    // Pair consecutive fibonacci numbers:
    let pairs: Vec<_> = Fibonacci::new()
        .take(5)
        .zip(Fibonacci::new().skip(1).take(5))
        .collect();
    println!("consecutive pairs: {pairs:?}");
}

// Simple binary tree for iterator demonstration
#[derive(Debug)]
enum Tree<T> {
    Leaf,
    Node { value: T, left: Box<Tree<T>>, right: Box<Tree<T>> },
}

impl<T: Ord> Tree<T> {
    fn insert(self, val: T) -> Self {
        match self {
            Tree::Leaf => Tree::Node {
                value: val,
                left: Box::new(Tree::Leaf),
                right: Box::new(Tree::Leaf),
            },
            Tree::Node { value, left, right } => {
                if val < value {
                    Tree::Node { value, left: Box::new(left.insert(val)), right }
                } else if val > value {
                    Tree::Node { value, left, right: Box::new(right.insert(val)) }
                } else {
                    Tree::Node { value, left, right } // duplicate ignored
                }
            }
        }
    }

    // In-order traversal → sorted output
    fn in_order(&self) -> Vec<&T> {
        match self {
            Tree::Leaf => vec![],
            Tree::Node { value, left, right } => {
                let mut v = left.in_order();
                v.push(value);
                v.extend(right.in_order());
                v
            }
        }
    }
}

fn tree_iterator() {
    println!("\n=== BST In-Order Traversal ===");

    let tree = [5, 3, 7, 1, 4, 6, 8]
        .into_iter()
        .fold(Tree::Leaf, |t, v| t.insert(v));

    let sorted = tree.in_order();
    println!("in-order: {sorted:?}");
}

fn generator_style() {
    println!("\n=== Generator-Style with gen blocks (Rust 2024 Edition) ===");
    // Note: `gen` blocks are a Rust 2024 feature for writing iterator-like
    // generators using a natural sequential style.
    // Due to compiler feature stabilisation, we show the pattern:

    println!("gen block example (Rust edition 2024):");
    println!("  fn even_fibonacci() -> impl Iterator<Item=u64> {{");
    println!("      gen {{");
    println!("          let mut a = 0_u64; let mut b = 1_u64;");
    println!("          loop {{");
    println!("              if a % 2 == 0 {{ yield a; }}");
    println!("              (a, b) = (b, a.saturating_add(b));");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Equivalent using a struct iterator (works on all stable editions):
    let even_fibs: Vec<u64> = Fibonacci::new()
        .filter(|&n| n % 2 == 0)
        .take(6)
        .collect();
    println!("even fibonacci (filter-based): {even_fibs:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range2d_count() {
        assert_eq!(Range2D::new(3, 4).count(), 12);
    }

    #[test]
    fn fibonacci_first_seven() {
        let v: Vec<u64> = Fibonacci::new().take(7).collect();
        assert_eq!(v, vec![1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn tree_sorted() {
        let tree = [3, 1, 4, 1, 5, 9, 2, 6]
            .into_iter()
            .fold(Tree::Leaf, |t, v| t.insert(v));
        let sorted = tree.in_order();
        let values: Vec<i32> = sorted.iter().map(|&&v| v).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6, 9]);
    }
}
