// ============================================================
// MIGRATION GUIDE: LINQ vs Rust Iterators
// ============================================================
//
// LINQ and Rust iterators are both lazy pipelines over sequences.
// The mental models are very similar — the main differences are
// syntax and the trait hierarchy.
//
// RUN: cargo run --bin linq_vs_iterators
// ============================================================

use std::collections::HashMap;

fn main() {
    println!("=== LINQ vs Rust Iterators ===\n");

    filtering_and_mapping();
    aggregation();
    grouping();
    joining();
    ordering();
    set_operations();
    chaining_comparison();
}

fn filtering_and_mapping() {
    println!("--- Filtering and Mapping ---");

    let numbers: Vec<i32> = (1..=10).collect();

    // C#: numbers.Where(x => x % 2 == 0).Select(x => x * x)
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("even squares: {result:?}");

    // C#: numbers.Select((x, i) => (i, x))
    let indexed: Vec<(usize, i32)> = numbers.iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .collect();
    println!("first 3 indexed: {:?}", &indexed[..3]);

    // C#: numbers.Where(x => x > 5).FirstOrDefault()
    let first_big = numbers.iter().find(|&&x| x > 5);
    println!("first > 5: {first_big:?}");

    // C#: numbers.TakeWhile(x => x < 5)
    let taken: Vec<i32> = numbers.iter()
        .copied()
        .take_while(|&x| x < 5)
        .collect();
    println!("take_while < 5: {taken:?}");
}

fn aggregation() {
    println!("\n--- Aggregation ---");

    let data: Vec<i32> = (1..=5).collect();

    // C#: data.Sum()
    let sum: i32 = data.iter().sum();
    println!("sum: {sum}");

    // C#: data.Aggregate((a, x) => a + x)
    let product: i32 = data.iter().copied().reduce(|a, x| a * x).unwrap();
    println!("product (reduce): {product}");

    // C#: data.Aggregate(0, (acc, x) => acc + x * x)
    let sum_sq: i32 = data.iter().fold(0, |acc, &x| acc + x * x);
    println!("fold sum-of-squares: {sum_sq}");

    // C#: data.Count()
    let count = data.iter().count();
    println!("count: {count}");

    // C#: data.Count(x => x % 2 == 0)
    let even_count = data.iter().filter(|&&x| x % 2 == 0).count();
    println!("even count: {even_count}");

    // C#: data.Min() / .Max()
    println!("min: {:?}", data.iter().min());
    println!("max: {:?}", data.iter().max());

    // C#: data.Any(x => x > 3) / .All(x => x > 0)
    println!("any > 3: {}", data.iter().any(|&x| x > 3));
    println!("all > 0: {}", data.iter().all(|&x| x > 0));
}

fn grouping() {
    println!("\n--- Grouping ---");

    let words = ["apple", "banana", "avocado", "blueberry", "cherry", "apricot"];

    // C#: words.GroupBy(w => w[0])
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
    for &w in &words {
        groups.entry(w.chars().next().unwrap())
            .or_default()
            .push(w);
    }

    let mut keys: Vec<char> = groups.keys().cloned().collect();
    keys.sort();
    for k in keys {
        println!("  '{k}': {:?}", groups[&k]);
    }

    // C#: words.GroupBy(w => w.Length).Select(g => (g.Key, g.Count()))
    let by_len: HashMap<usize, usize> = words.iter()
        .fold(HashMap::new(), |mut m, w| {
            *m.entry(w.len()).or_insert(0) += 1;
            m
        });
    let mut lens: Vec<(usize, usize)> = by_len.into_iter().collect();
    lens.sort();
    println!("  by length: {lens:?}");
}

fn joining() {
    println!("\n--- Joining (zip) ---");

    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![95, 87, 92];

    // C#: names.Zip(scores, (n, s) => $"{n}={s}")
    let paired: Vec<String> = names.iter()
        .zip(scores.iter())
        .map(|(&n, &s)| format!("{n}={s}"))
        .collect();
    println!("zipped: {paired:?}");

    // Inner join pattern (simulate a DB join):
    let users = vec![(1u32, "Alice"), (2, "Bob"), (3, "Carol")];
    let orders = vec![(1u32, "Laptop"), (1, "Phone"), (3, "Tablet")];

    // C#: users.Join(orders, u => u.Id, o => o.UserId, (u, o) => ...)
    let joined: Vec<String> = users.iter()
        .flat_map(|&(uid, name)| {
            orders.iter()
                .filter(move |&&(oid, _)| oid == uid)
                .map(move |&(_, item)| format!("{name} → {item}"))
        })
        .collect();
    println!("joined: {joined:?}");
}

fn ordering() {
    println!("\n--- Ordering ---");

    let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];

    // C#: data.OrderBy(x => x)
    let sorted: Vec<i32> = {
        let mut v = data.clone();
        v.sort();
        v
    };
    println!("sorted: {sorted:?}");

    // C#: data.OrderByDescending(x => x)
    let mut desc = data.clone();
    desc.sort_by(|a, b| b.cmp(a));
    println!("desc: {desc:?}");

    // C#: people.OrderBy(p => p.LastName).ThenBy(p => p.FirstName)
    let mut people = vec![("Alice", "Smith"), ("Bob", "Jones"), ("Carol", "Smith")];
    people.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
    println!("people by last then first: {people:?}");

    // C#: data.Distinct()
    data.sort();
    data.dedup();
    println!("deduplicated: {data:?}");
}

fn set_operations() {
    println!("\n--- Set Operations ---");

    use std::collections::HashSet;
    let a: HashSet<i32> = [1, 2, 3, 4, 5].into();
    let b: HashSet<i32> = [3, 4, 5, 6, 7].into();

    // C#: a.Intersect(b)
    let mut inter: Vec<i32> = a.intersection(&b).cloned().collect();
    inter.sort();
    println!("intersection: {inter:?}");

    // C#: a.Union(b)
    let mut union: Vec<i32> = a.union(&b).cloned().collect();
    union.sort();
    println!("union: {union:?}");

    // C#: a.Except(b)
    let mut except: Vec<i32> = a.difference(&b).cloned().collect();
    except.sort();
    println!("difference (a-b): {except:?}");
}

fn chaining_comparison() {
    println!("\n--- Side-by-Side Comparison ---");

    println!(r#"
LINQ                                 | Rust Iterator
-------------------------------------|--------------------------------------
.Where(x => pred)                    | .filter(|x| pred)
.Select(x => f(x))                   | .map(|x| f(x))
.SelectMany(x => coll(x))            | .flat_map(|x| coll(x))
.Take(n)                             | .take(n)
.Skip(n)                             | .skip(n)
.TakeWhile(pred)                     | .take_while(|x| pred)
.SkipWhile(pred)                     | .skip_while(|x| pred)
.First() / .FirstOrDefault()         | .next() after filter / .find()
.Last()                              | .last()
.Single()                            | (no direct equiv; use .next() + assert)
.OrderBy(key)                        | sort_by_key() (not lazy)
.OrderByDescending(key)              | sort_by(|a,b| b.cmp(a))
.ThenBy(key)                         | .then_with(|| ...)
.Distinct()                          | dedup() after sort / HashSet
.GroupBy(key)                        | fold into HashMap
.Zip(other, f)                       | .zip(other).map(|(a,b)| f(a,b))
.Concat(other)                       | .chain(other)
.Reverse()                           | .rev()
.Count()                             | .count()
.Sum() / .Min() / .Max()             | .sum() / .min() / .max()
.Any(pred) / .All(pred)              | .any(pred) / .all(pred)
.Aggregate(seed, f)                  | .fold(seed, f)
.Aggregate(f)                        | .reduce(f)
.ToList()                            | .collect::<Vec<_>>()
.ToArray()                           | .collect::<Vec<_>>() (no fixed array collect)
.ToDictionary(k, v)                  | .map(|x| (k(x), v(x))).collect::<HashMap<_,_>>()
.Contains(x)                         | .any(|y| y == x) or .contains(&x) on slice
.ElementAt(i)                        | .nth(i)
.Append(x)                           | .chain(std::iter::once(x))
.Prepend(x)                          | std::iter::once(x).chain(iter)
.Flatten() / SelectMany(x => x)      | .flatten()
"#);
}
