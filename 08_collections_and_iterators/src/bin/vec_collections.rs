// ============================================================
// CONCEPT: Vec, VecDeque, BTreeMap, BTreeSet and other collections
// ============================================================
//
// C# → Rust collection mapping:
//   List<T>            → Vec<T>
//   Queue<T>/Stack<T>  → VecDeque<T>  (double-ended queue)
//   Dictionary<K,V>    → HashMap<K,V> (see hashmap_hashset.rs)
//   SortedDictionary   → BTreeMap<K,V>
//   HashSet<T>         → HashSet<T>
//   SortedSet<T>       → BTreeSet<T>
//   LinkedList<T>      → std::collections::LinkedList<T>
//
// RUN: cargo run --bin vec_collections
// ============================================================

use std::collections::{VecDeque, BTreeMap, BTreeSet, LinkedList};

fn main() {
    vec_basics();
    vec_operations();
    vecdeque_demo();
    btreemap_demo();
    btreeset_demo();
    linkedlist_demo();
}

fn vec_basics() {
    println!("=== Vec<T> ===");

    // Creation
    let empty: Vec<i32> = Vec::new();
    let from_macro = vec![1, 2, 3, 4, 5];
    let with_capacity: Vec<i32> = Vec::with_capacity(10); // pre-allocate
    let repeated = vec![0_i32; 5]; // [0, 0, 0, 0, 0]

    println!("empty len={} capacity={}", empty.len(), empty.capacity());
    println!("from_macro: {from_macro:?}");
    println!("with_capacity: len={} cap={}", with_capacity.len(), with_capacity.capacity());
    println!("repeated: {repeated:?}");

    // Reading
    let v = vec![10, 20, 30, 40, 50];
    println!("v[2] = {}", v[2]);                     // panics on OOB
    println!("v.get(2) = {:?}", v.get(2));            // safe: Option<&T>
    println!("v.get(99) = {:?}", v.get(99));          // None
    println!("first = {:?}", v.first());
    println!("last  = {:?}", v.last());
    println!("len = {}  is_empty = {}", v.len(), v.is_empty());
}

fn vec_operations() {
    println!("\n=== Vec Operations ===");

    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    // Push / pop
    v.push(7);
    println!("after push(7): {:?}", &v[..]);
    println!("pop: {:?}", v.pop());

    // Insert / remove
    v.insert(0, 0);  // insert at index 0
    v.remove(5);     // remove at index 5 (shifts elements)
    println!("after insert/remove: {:?}", v);

    // Sort
    let mut sorted = v.clone();
    sorted.sort();
    println!("sorted: {sorted:?}");

    // Sort by custom key:
    let mut words = vec!["banana", "apple", "cherry", "date"];
    words.sort_by_key(|w| w.len());
    println!("sorted by len: {words:?}");

    // Binary search (requires sorted):
    sorted.dedup(); // remove consecutive duplicates first
    println!("dedup: {sorted:?}");
    println!("binary_search(5): {:?}", sorted.binary_search(&5));

    // Slice operations on Vec:
    let slice = &sorted[1..4];
    println!("slice [1..4]: {slice:?}");

    // Retain (in-place filter — like C# RemoveAll):
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    nums.retain(|&x| x % 2 == 0);
    println!("evens only: {nums:?}");

    // Drain: remove a range and iterate removed elements
    let drained: Vec<_> = nums.drain(1..).collect();
    println!("drained: {drained:?}  remaining: {nums:?}");

    // Extend, append, concat
    let mut a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    a.extend(&b);   // b is borrowed — a copy of elements
    println!("extended: {a:?}");

    let combined: Vec<_> = [vec![1,2], vec![3,4], vec![5,6]].concat();
    println!("concat: {combined:?}");

    // Contains, position
    println!("contains 3: {}", a.contains(&3));
    println!("position of 4: {:?}", a.iter().position(|&x| x == 4));

    // Chunks and windows:
    let data = vec![1, 2, 3, 4, 5, 6];
    let chunks: Vec<_> = data.chunks(2).collect();
    println!("chunks(2): {chunks:?}");
    let windows: Vec<_> = data.windows(3).collect();
    println!("windows(3): {windows:?}");
}

fn vecdeque_demo() {
    println!("\n=== VecDeque<T> (double-ended queue) ===");

    // Like C# Queue<T> / Stack<T> but more flexible
    let mut deque: VecDeque<i32> = VecDeque::new();

    // Push to front or back:
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    deque.push_front(-1);
    println!("deque: {deque:?}");

    // Pop from front or back:
    println!("front: {:?}", deque.front());
    println!("back:  {:?}", deque.back());
    println!("pop_front: {:?}", deque.pop_front());
    println!("pop_back:  {:?}", deque.pop_back());
    println!("after: {deque:?}");

    // VecDeque as Queue (FIFO):
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("first");
    queue.push_back("second");
    queue.push_back("third");
    while let Some(item) = queue.pop_front() {
        println!("dequeued: {item}");
    }
}

fn btreemap_demo() {
    println!("\n=== BTreeMap<K,V> (sorted dictionary) ===");

    // BTreeMap keeps keys SORTED — like C# SortedDictionary<K,V>
    // Slower than HashMap but iterates in sorted order.
    let mut scores: BTreeMap<String, i32> = BTreeMap::new();
    scores.insert("Alice".into(), 95);
    scores.insert("Charlie".into(), 78);
    scores.insert("Bob".into(), 88);

    // Iteration is in KEY ORDER:
    for (name, score) in &scores {
        println!("  {name}: {score}");
    }

    // Range queries — a feature HashMap cannot do:
    for (name, score) in scores.range(String::from("B")..=String::from("C")) {
        println!("  range query: {name} = {score}");
    }

    println!("first entry: {:?}", scores.iter().next());
    println!("last entry: {:?}", scores.iter().next_back());
}

fn btreeset_demo() {
    println!("\n=== BTreeSet<T> (sorted set) ===");

    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.extend([5, 3, 8, 1, 9, 2, 7, 4, 6]);

    // Always sorted:
    println!("set: {set:?}");
    println!("min: {:?}", set.iter().next());
    println!("max: {:?}", set.iter().next_back());

    // Range queries:
    let range: Vec<_> = set.range(3..=7).collect();
    println!("3..=7: {range:?}");

    // Set operations:
    let a: BTreeSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: BTreeSet<i32> = [3, 4, 5, 6].into_iter().collect();

    let union: BTreeSet<_>        = a.union(&b).collect();
    let intersection: BTreeSet<_> = a.intersection(&b).collect();
    let difference: BTreeSet<_>   = a.difference(&b).collect();

    println!("union: {union:?}");
    println!("intersection: {intersection:?}");
    println!("difference (a-b): {difference:?}");
}

fn linkedlist_demo() {
    println!("\n=== LinkedList<T> ===");

    // LinkedList is rarely used in Rust — Vec is usually better.
    // Use LinkedList only when you need O(1) split and prepend.
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_front(0);

    println!("list: {list:?}");
    list.pop_front();
    println!("after pop_front: {list:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_push_pop() {
        let mut v: Vec<i32> = Vec::new();
        v.push(1); v.push(2);
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn vec_sort() {
        let mut v = vec![3, 1, 2];
        v.sort();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn btreemap_sorted() {
        let mut m: BTreeMap<i32, &str> = BTreeMap::new();
        m.insert(3, "c"); m.insert(1, "a"); m.insert(2, "b");
        let keys: Vec<i32> = m.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }
}
