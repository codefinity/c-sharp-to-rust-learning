// ============================================================
// CONCEPT: HashMap and HashSet
// ============================================================
// RUN: cargo run --bin hashmap_hashset
// ============================================================

use std::collections::{HashMap, HashSet};

fn main() {
    hashmap_basics();
    hashmap_operations();
    entry_api();
    hashset_demo();
    grouping_pattern();
}

fn hashmap_basics() {
    println!("=== HashMap<K,V> ===");

    // C# Dictionary<string, int>
    let mut scores: HashMap<String, i32> = HashMap::new();

    // insert — returns the OLD value if key existed
    scores.insert("Alice".into(), 10);
    scores.insert("Bob".into(), 20);
    let old = scores.insert("Alice".into(), 50); // update Alice
    println!("old value for Alice: {old:?}"); // Some(10)

    // get returns Option<&V>
    println!("Alice: {:?}", scores.get("Alice"));
    println!("Carol: {:?}", scores.get("Carol"));

    // Direct access (panics if missing):
    println!("Bob: {}", scores["Bob"]);

    // contains_key
    println!("has Alice: {}", scores.contains_key("Alice"));

    // Iteration — unordered (use BTreeMap for sorted)
    for (name, score) in &scores {
        println!("  {name} = {score}");
    }

    // len, is_empty
    println!("len={} empty={}", scores.len(), scores.is_empty());
}

fn hashmap_operations() {
    println!("\n=== HashMap Operations ===");

    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();

    // Remove — returns Option<V>
    map.insert("a", vec![1, 2]);
    let removed = map.remove("a");
    println!("removed: {removed:?}");

    // Collect from iterator of tuples:
    let pairs = [("one", 1), ("two", 2), ("three", 3)];
    let map2: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("from iter: {map2:?}");

    // From array literal (Rust 1.56+):
    let map3: HashMap<_, _> = [("x", 10), ("y", 20), ("z", 30)].into_iter().collect();
    println!("map3: {map3:?}");

    // Keys, values, and entries iterators:
    let keys: Vec<_> = map3.keys().collect();
    let vals: Vec<_> = map3.values().collect();
    println!("keys: {keys:?}");
    println!("vals: {vals:?}");

    // Retain: remove entries matching predicate
    let mut m: HashMap<&str, i32> = map3.clone().into_iter().collect();
    m.retain(|_, v| *v > 15);
    println!("after retain (>15): {m:?}");

    // Merge two maps (second overwrites first for duplicate keys):
    let mut merged: HashMap<&str, i32> = [("a",1), ("b",2)].into_iter().collect();
    let extra: HashMap<&str, i32> = [("b",20), ("c",3)].into_iter().collect();
    merged.extend(extra);
    println!("merged: {merged:?}");
}

fn entry_api() {
    println!("\n=== Entry API (insert-or-update) ===");

    // C# pattern: if (dict.ContainsKey(k)) dict[k]++; else dict[k] = 1;
    // Rust idiomatic:

    let text = "hello world foo bar hello foo foo";
    let mut word_count: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        // entry().or_insert() — like C# GetOrAdd
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut counts: Vec<_> = word_count.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1)); // sort by count descending
    for (word, count) in &counts {
        println!("  {word}: {count}");
    }

    // entry().or_insert_with() — lazy computation (closure called only if missing)
    let mut cache: HashMap<i32, Vec<i32>> = HashMap::new();
    let factors = cache.entry(12).or_insert_with(|| {
        println!("  computing factors of 12...");
        (1..=12).filter(|&x| 12 % x == 0).collect()
    });
    println!("factors of 12: {factors:?}");
    // Second call — closure NOT called again:
    let factors2 = cache.entry(12).or_insert_with(|| {
        println!("  this should NOT print");
        vec![]
    });
    println!("cached: {factors2:?}");

    // entry().and_modify() — only modify if key exists
    let mut m: HashMap<&str, i32> = [("a", 1)].into_iter().collect();
    m.entry("a").and_modify(|v| *v *= 10).or_insert(0);
    m.entry("b").and_modify(|v| *v *= 10).or_insert(99);
    println!("and_modify: {m:?}"); // a=10, b=99
}

fn hashset_demo() {
    println!("\n=== HashSet<T> ===");

    // C# HashSet<T> — exactly the same concept
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1); set.insert(2); set.insert(3);
    set.insert(2); // duplicate — ignored
    println!("set: {set:?}");
    println!("len: {}", set.len());
    println!("contains 2: {}", set.contains(&2));
    println!("contains 5: {}", set.contains(&5));

    // Set operations:
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5, 6].into_iter().collect();

    let union:        HashSet<i32> = a.union(&b).copied().collect();
    let intersection: HashSet<i32> = a.intersection(&b).copied().collect();
    let difference:   HashSet<i32> = a.difference(&b).copied().collect();     // a - b
    let sym_diff:     HashSet<i32> = a.symmetric_difference(&b).copied().collect();

    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("union: {union:?}");
    println!("intersection: {intersection:?}");
    println!("difference: {difference:?}");
    println!("symmetric_difference: {sym_diff:?}");

    println!("a.is_subset(union): {}", a.is_subset(&union));
    println!("a.is_superset(intersection): {}", a.is_superset(&intersection));
}

fn grouping_pattern() {
    println!("\n=== Grouping Pattern (like C# LINQ GroupBy) ===");

    let people = vec![
        ("Alice", 30), ("Bob", 25), ("Carol", 30),
        ("Dave", 25), ("Eve", 35),
    ];

    // Group by age:
    let mut by_age: HashMap<i32, Vec<&str>> = HashMap::new();
    for (name, age) in &people {
        by_age.entry(*age).or_insert_with(Vec::new).push(name);
    }

    let mut ages: Vec<i32> = by_age.keys().copied().collect();
    ages.sort();
    for age in ages {
        println!("  age {age}: {:?}", by_age[&age]);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn hashmap_word_count() {
        let mut m: HashMap<&str, u32> = HashMap::new();
        for w in ["a", "b", "a", "c", "b", "a"] {
            *m.entry(w).or_insert(0) += 1;
        }
        assert_eq!(m["a"], 3);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 1);
    }

    #[test]
    fn hashset_deduplication() {
        let v = vec![1, 2, 3, 2, 1, 4];
        let set: HashSet<i32> = v.into_iter().collect();
        assert_eq!(set.len(), 4);
    }

    #[test]
    fn set_intersection() {
        let a: HashSet<i32> = [1,2,3].into_iter().collect();
        let b: HashSet<i32> = [2,3,4].into_iter().collect();
        let inter: HashSet<_> = a.intersection(&b).collect();
        assert_eq!(inter.len(), 2);
    }
}
