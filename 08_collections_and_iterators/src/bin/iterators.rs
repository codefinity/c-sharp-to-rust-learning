// ============================================================
// CONCEPT: Iterators and Iterator Adapters
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// Rust iterators are like C# LINQ (IEnumerable<T>) but:
//   • LAZY by default — nothing executes until you consume
//   • Zero-cost — the compiler often optimises to a tight loop
//   • Chainable — each adapter transforms the iterator
//   • Type-safe — the full adapter chain is a single concrete type
//
// C# LINQ → Rust Iterator analogues:
//   .Select(x => x*2)          → .map(|x| x*2)
//   .Where(x => x > 5)         → .filter(|&x| x > 5)
//   .SelectMany(x => x)        → .flat_map(|x| x)
//   .Aggregate(0, (a,b) => a+b)→ .fold(0, |a,b| a+b)
//   .Sum()                     → .sum()
//   .Count()                   → .count()
//   .First()                   → .next() / .first()
//   .Any(pred)                 → .any(pred)
//   .All(pred)                 → .all(pred)
//   .Take(n)                   → .take(n)
//   .Skip(n)                   → .skip(n)
//   .Zip(other, ...)           → .zip(other)
//   .OrderBy(key)              → .sorted_by_key(key) (itertools crate)
//   .ToList()                  → .collect::<Vec<_>>()
//   .ToDictionary(k,v)         → .collect::<HashMap<_,_>>()
//
// RUN: cargo run --bin iterators
// ============================================================

fn main() {
    iterator_creation();
    transforming_adapters();
    consuming_adapters();
    chaining_adapters();
    iterator_patterns();
}

fn iterator_creation() {
    println!("=== Creating Iterators ===");

    // iter() — borrows each element (&T)
    let v = vec![1, 2, 3, 4, 5];
    let refs: Vec<&i32> = v.iter().collect();
    println!("iter: {refs:?}");

    // into_iter() — consumes, yields owned T
    let owned: Vec<i32> = vec![1, 2, 3].into_iter().collect();
    println!("into_iter: {owned:?}");

    // iter_mut() — yields &mut T
    let mut mutable = vec![1, 2, 3];
    mutable.iter_mut().for_each(|x| *x *= 2);
    println!("iter_mut doubled: {mutable:?}");

    // Range iterators
    let range: Vec<i32>  = (1..=5).collect();
    let stepped: Vec<i32> = (0..10).step_by(2).collect();
    println!("range: {range:?}");
    println!("step_by(2): {stepped:?}");

    // std::iter::once, repeat, empty
    let once_vals: Vec<i32>   = std::iter::once(42).collect();
    let repeated: Vec<i32>    = std::iter::repeat(7).take(3).collect();
    let empty: Vec<i32>       = std::iter::empty::<i32>().collect();
    println!("once: {once_vals:?}");
    println!("repeat(7).take(3): {repeated:?}");
    println!("empty: {empty:?}");

    // successors — iterate a function of the previous value
    let powers_of_2: Vec<u32> = std::iter::successors(Some(1_u32), |&n| {
        n.checked_mul(2) // None when overflow
    }).take(10).collect();
    println!("powers of 2: {powers_of_2:?}");
}

fn transforming_adapters() {
    println!("\n=== Transforming Adapters ===");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — transform each element (Select in LINQ)
    let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    println!("map (double): {doubled:?}");

    // filter — keep elements matching predicate (Where in LINQ)
    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter (even): {evens:?}");

    // filter_map — filter AND transform (combines Where+Select+null-filter)
    let parsed: Vec<i32> = vec!["1", "two", "3", "four", "5"]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("filter_map: {parsed:?}");

    // flat_map — transform and flatten (SelectMany in LINQ)
    let words = vec!["hello world", "foo bar baz"];
    let all_words: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map: {all_words:?}");

    // flatten — flatten an iterator of iterables
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat: Vec<i32> = nested.into_iter().flatten().collect();
    println!("flatten: {flat:?}");

    // take / skip
    let first3: Vec<&i32>    = v.iter().take(3).collect();
    let after3: Vec<&i32>    = v.iter().skip(3).collect();
    let middle: Vec<&i32>    = v.iter().skip(2).take(4).collect();
    println!("take(3): {first3:?}");
    println!("skip(3): {after3:?}");
    println!("skip(2).take(4): {middle:?}");

    // enumerate — add index (like Select with index in LINQ)
    for (i, &val) in v.iter().enumerate().take(3) {
        println!("  [{i}] = {val}");
    }

    // zip — pair elements from two iterators
    let letters = vec!['a', 'b', 'c'];
    let pairs: Vec<_> = (1..=3).zip(letters.iter()).collect();
    println!("zip: {pairs:?}");

    // chain — concatenate iterators
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("chain: {chained:?}");

    // rev — reverse
    let reversed: Vec<&i32> = v.iter().rev().take(3).collect();
    println!("rev.take(3): {reversed:?}");

    // windows / chunks as iterators (already in vec_collections but useful here):
    let windows: Vec<&[i32]> = v.windows(3).collect();
    println!("windows(3): first={:?}", windows[0]);

    // peekable — look ahead without consuming
    let mut peekable = v.iter().peekable();
    while let Some(&&next) = peekable.peek() {
        if next > 5 { break; }
        print!("{} ", peekable.next().unwrap());
    }
    println!("(stopped before {}", peekable.peek().copied().unwrap());
}

fn consuming_adapters() {
    println!("\n=== Consuming Adapters (Terminators) ===");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // collect — materialise into a collection
    let vec: Vec<i32>        = (1..=5).collect();
    let set: std::collections::HashSet<i32> = (1..=5).collect();
    println!("collect vec: {vec:?}");
    println!("collect set len: {}", set.len());

    // sum / product
    let sum: i32     = v.iter().sum();
    let product: u64 = (1_u64..=5).product();
    println!("sum: {sum}  product 1..=5: {product}");

    // count
    let count = v.iter().filter(|&&x| x % 2 == 0).count();
    println!("even count: {count}");

    // fold (reduce with accumulator) — like Aggregate in LINQ
    let total = v.iter().fold(0_i32, |acc, &x| acc + x);
    println!("fold sum: {total}");

    // reduce — fold without initial value
    let max = v.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("reduce max: {max:?}");

    // any / all
    println!("any >8: {}", v.iter().any(|&x| x > 8));
    println!("all >0: {}", v.iter().all(|&x| x > 0));

    // find / position / find_map
    println!("find >5: {:?}", v.iter().find(|&&x| x > 5));
    println!("position >5: {:?}", v.iter().position(|&x| x > 5));
    let found = v.iter().find_map(|&x| if x > 5 { Some(x * 10) } else { None });
    println!("find_map (first >5 * 10): {found:?}");

    // min / max
    println!("min: {:?}  max: {:?}", v.iter().min(), v.iter().max());

    // min_by_key / max_by_key
    let words = vec!["apple", "fig", "banana", "kiwi"];
    println!("shortest: {:?}", words.iter().min_by_key(|w| w.len()));
    println!("longest: {:?}", words.iter().max_by_key(|w| w.len()));

    // for_each — like foreach in C#
    v.iter().take(3).for_each(|x| print!("{x} "));
    println!();

    // last
    println!("last: {:?}", v.iter().last());

    // nth — O(n)
    println!("nth(4): {:?}", v.iter().nth(4)); // 5th element
}

fn chaining_adapters() {
    println!("\n=== Chaining Adapters (LINQ-style pipelines) ===");

    // C# LINQ equivalent:
    // var result = numbers
    //     .Where(x => x % 2 == 0)
    //     .Select(x => x * x)
    //     .Where(x => x > 10)
    //     .Sum();

    let result: i32 = (1..=20)
        .filter(|&x| x % 2 == 0)   // keep evens
        .map(|x| x * x)             // square them
        .filter(|&x| x > 10)        // keep > 10
        .sum();                     // total
    println!("pipeline result: {result}");

    // Complex grouping without external crates:
    let words = "the quick brown fox jumps over the lazy dog";
    let mut word_lengths: Vec<(&str, usize)> = words.split_whitespace()
        .map(|w| (w, w.len()))
        .collect();
    word_lengths.sort_by_key(|&(_, len)| std::cmp::Reverse(len));
    for (word, len) in word_lengths.iter().take(3) {
        println!("  longest: {word} ({len})");
    }

    // Unzip — split pairs iterator into two collections
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (nums, chars): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
    println!("unzip nums: {nums:?}  chars: {chars:?}");
}

fn iterator_patterns() {
    println!("\n=== Iterator Patterns ===");

    // scan — like fold but yields intermediate values (like observable Scan in Rx)
    let running_sum: Vec<i32> = (1..=5).scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    }).collect();
    println!("running sum: {running_sum:?}");

    // inspect — non-consuming side effects for debugging
    let result: Vec<i32> = (1..=5)
        .inspect(|x| print!("before:{x} "))
        .map(|x| x * 2)
        .inspect(|x| print!("after:{x} "))
        .collect();
    println!("\ninspect: {result:?}");

    // take_while / skip_while
    let taken: Vec<i32> = (1..).take_while(|&x| x < 5).collect();
    let skipped: Vec<i32> = vec![1, 3, 5, 7, 2, 4].into_iter().skip_while(|&x| x % 2 != 0).collect();
    println!("take_while (<5): {taken:?}");
    println!("skip_while (odd): {skipped:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn map_filter_sum() {
        let result: i32 = (1..=10)
            .filter(|&x| x % 2 == 0)
            .map(|x| x * x)
            .sum();
        assert_eq!(result, 4 + 16 + 36 + 64 + 100);
    }

    #[test]
    fn collect_to_vec() {
        let v: Vec<i32> = (1..=5).collect();
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn flat_map() {
        let nested = vec![vec![1, 2], vec![3, 4]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }

    #[test]
    fn find_and_position() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(v.iter().find(|&&x| x > 25), Some(&30));
        assert_eq!(v.iter().position(|&x| x > 25), Some(2));
    }
}
