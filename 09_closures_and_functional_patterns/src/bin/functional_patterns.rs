// ============================================================
// CONCEPT: Functional Patterns — map, filter, fold, monadic chains
// ============================================================
// RUN: cargo run --bin functional_patterns
// ============================================================

fn main() {
    map_filter_reduce();
    monadic_option_result();
    currying_partial();
    memoization();
    pipeline_builder();
}

fn map_filter_reduce() {
    println!("=== Map / Filter / Fold ===");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // C# LINQ pipeline equivalent:
    let result: Vec<String> = data.iter()
        .filter(|&&x| x % 2 == 0)      // Where
        .map(|&x| x * x)               // Select
        .filter(|&x| x > 10)           // Where again
        .map(|x| format!("{x:>3}"))    // Select to string
        .collect();

    println!("pipeline: {result:?}");

    // Fold (reduce with accumulator):
    let sentence = vec!["Hello", "from", "Rust"];
    let joined = sentence.iter().fold(String::new(), |mut acc, &word| {
        if !acc.is_empty() { acc.push(' '); }
        acc.push_str(word);
        acc
    });
    println!("fold join: {joined}");

    // scan — running totals
    let running: Vec<i32> = (1..=5).scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    }).collect();
    println!("running sums: {running:?}");
}

fn monadic_option_result() {
    println!("\n=== Monadic Option/Result Chains ===");

    // Rust Option and Result behave like monads:
    // map   = fmap
    // and_then = bind (>>=)

    fn safe_div(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 { None } else { Some(a / b) }
    }

    fn safe_sqrt(x: f64) -> Option<f64> {
        if x < 0.0 { None } else { Some(x.sqrt()) }
    }

    // Monadic chain: divide then sqrt
    let result = safe_div(100.0, 4.0).and_then(safe_sqrt);
    println!("sqrt(100/4) = {result:?}"); // Some(5.0)

    let result2 = safe_div(100.0, 0.0).and_then(safe_sqrt);
    println!("sqrt(100/0) = {result2:?}"); // None (short-circuits)

    let result3 = safe_div(100.0, -1.0).and_then(safe_sqrt);
    println!("sqrt(100/-1) = {result3:?}"); // None (negative sqrt)

    // Result chain:
    let parse_and_sqrt: Result<f64, String> = "9.0"
        .parse::<f64>().map_err(|e| e.to_string())
        .and_then(|n| if n >= 0.0 { Ok(n.sqrt()) } else { Err("negative".into()) });
    println!("parse_and_sqrt: {parse_and_sqrt:?}");
}

fn currying_partial() {
    println!("\n=== Currying and Partial Application ===");

    // Rust doesn't have built-in currying, but we can simulate it:
    fn add(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }

    let add5 = add(5);
    let add10 = add(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // Partial application via closures:
    fn multiply(a: i32, b: i32) -> i32 { a * b }

    let double   = |x| multiply(2, x);
    let triple   = |x| multiply(3, x);

    let results: Vec<i32> = (1..=5).map(double).collect();
    println!("doubled: {results:?}");
    let results: Vec<i32> = (1..=5).map(triple).collect();
    println!("tripled: {results:?}");

    // Function composition:
    fn compose<A, B, C>(f: impl Fn(A) -> B + 'static, g: impl Fn(B) -> C + 'static)
        -> impl Fn(A) -> C
    {
        move |x| g(f(x))
    }

    let double_then_stringify = compose(|x: i32| x * 2, |x: i32| format!("{x}"));
    let results: Vec<String> = (1..=5).map(double_then_stringify).collect();
    println!("compose: {results:?}");
}

fn memoization() {
    println!("\n=== Memoization ===");

    use std::collections::HashMap;

    struct Memoized<A, B> {
        cache: HashMap<A, B>,
        compute: Box<dyn Fn(A) -> B>,
    }

    impl<A: std::hash::Hash + Eq + Clone, B: Clone> Memoized<A, B> {
        fn new(f: impl Fn(A) -> B + 'static) -> Self {
            Self { cache: HashMap::new(), compute: Box::new(f) }
        }

        fn call(&mut self, arg: A) -> &B {
            if !self.cache.contains_key(&arg) {
                let result = (self.compute)(arg.clone());
                self.cache.insert(arg.clone(), result);
            }
            &self.cache[&arg]
        }
    }

    let mut fib_memo = Memoized::new(|n: u64| {
        // Simple (non-recursive) fib for demo:
        let (mut a, mut b) = (0_u64, 1_u64);
        for _ in 0..n { let t = a + b; a = b; b = t; }
        a
    });

    for n in [10, 20, 10, 30, 20] {
        println!("  fib({n}) = {}", fib_memo.call(n));
    }
    println!("  cache size: {}", fib_memo.cache.len()); // 3 unique keys
}

fn pipeline_builder() {
    println!("\n=== Pipeline Builder (fluent API) ===");

    // Builder pattern with a processing pipeline — like LINQ's deferred execution
    struct Pipeline<T> {
        data: Vec<T>,
    }

    impl<T: Clone + std::fmt::Debug> Pipeline<T> {
        fn new(data: Vec<T>) -> Self { Self { data } }

        fn filter(mut self, pred: impl Fn(&T) -> bool) -> Self {
            self.data.retain(|x| pred(x));
            self
        }

        fn inspect(self, f: impl Fn(&T)) -> Self {
            self.data.iter().for_each(f);
            self
        }

        fn collect(self) -> Vec<T> { self.data }
    }

    impl<T: Clone + std::fmt::Debug + std::ops::Mul<Output = T> + Copy + From<i32>> Pipeline<T> {
        fn map_mul(mut self, factor: T) -> Self {
            self.data = self.data.iter().map(|&x| x * factor).collect();
            self
        }
    }

    let result = Pipeline::new(vec![1_i32, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        .filter(|&x| x % 2 == 0)
        .inspect(|x| print!("filtered: {x}  "))
        .collect();
    println!("\nresult: {result:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn fold_sum() {
        let sum: i32 = (1..=10).fold(0, |acc, x| acc + x);
        assert_eq!(sum, 55);
    }

    #[test]
    fn monadic_chain_short_circuits() {
        fn safe_div(a: f64, b: f64) -> Option<f64> {
            if b == 0.0 { None } else { Some(a / b) }
        }
        assert_eq!(safe_div(10.0, 0.0).and_then(|x| Some(x * 2.0)), None);
    }

    #[test]
    fn partial_application() {
        let add = |a: i32| move |b: i32| a + b;
        let add5 = add(5);
        assert_eq!(add5(3), 8);
    }
}
