// ============================================================
// CONCEPT: Result Patterns — comprehensive guide
// ============================================================
// RUN: cargo run --bin result_patterns
// ============================================================
use std::num::ParseIntError;
use std::fs;
use std::io;

fn main() {
    early_return_patterns();
    chaining_results();
    collecting_results();
    multi_error_type();
}

fn early_return_patterns() {
    println!("=== Early Return with ? ===");

    // Without ? (verbose):
    fn parse_add_verbose(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let x = match a.parse::<i32>() { Ok(v) => v, Err(e) => return Err(e) };
        let y = match b.parse::<i32>() { Ok(v) => v, Err(e) => return Err(e) };
        Ok(x + y)
    }

    // With ? (idiomatic):
    fn parse_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
        Ok(a.parse::<i32>()? + b.parse::<i32>()?)
    }

    println!("{:?}", parse_add("3", "4"));
    println!("{:?}", parse_add("3", "x"));
    println!("{:?}", parse_add_verbose("5", "6"));

    // Nested ? in complex pipelines:
    fn process_csv_line(line: &str) -> Result<Vec<i32>, ParseIntError> {
        line.split(',')
            .map(str::trim)
            .map(|s| s.parse::<i32>())
            .collect() // collects into Result<Vec<i32>, ParseIntError>
    }

    println!("{:?}", process_csv_line("1, 2, 3, 4"));
    println!("{:?}", process_csv_line("1, two, 3"));
}

fn chaining_results() {
    println!("\n=== Chaining Results ===");

    // Map/and_then chain — like LINQ in C#
    let result = "42"
        .parse::<i32>()
        .map(|n| n * 2)
        .and_then(|n| if n > 0 { Ok(n) } else { Err("negative".parse::<i32>().unwrap_err()) });
    println!("chain result: {result:?}");

    // Flat chain across different error types using Box<dyn Error>:
    type BoxError = Box<dyn std::error::Error + Send + Sync>;

    fn fetch_and_parse(path: &str) -> Result<i32, BoxError> {
        let content = fs::read_to_string(path)?; // io::Error
        let n: i32  = content.trim().parse()?;   // ParseIntError
        Ok(n)
    }

    match fetch_and_parse("number.txt") {
        Ok(n)  => println!("from file: {n}"),
        Err(e) => println!("expected error: {e}"),
    }
}

fn collecting_results() {
    println!("\n=== Collecting Results ===");

    // Vec<Result<T,E>> → Result<Vec<T>,E>  (fails on first error)
    let strings = vec!["1", "2", "3"];
    let nums: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("all ok: {nums:?}");

    let mixed = vec!["1", "bad", "3"];
    let mixed_result: Result<Vec<i32>, _> = mixed.iter().map(|s| s.parse::<i32>()).collect();
    println!("with error: {:?}", mixed_result.map_err(|e| e.to_string()));

    // Partition: separate Ok from Err values
    let inputs = vec!["1", "bad", "3", "also_bad", "5"];
    let (oks, errs): (Vec<_>, Vec<_>) = inputs.iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<i32> = oks.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();
    println!("numbers: {numbers:?}");
    println!("errors: {}", errors.len());

    // filter_map — skip errors, keep successes
    let numbers_only: Vec<i32> = inputs.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("numbers only (filter_map): {numbers_only:?}");
}

#[derive(Debug)]
enum MultiError {
    Parse(ParseIntError),
    Io(io::Error),
    OutOfRange(i32),
}

impl std::fmt::Display for MultiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiError::Parse(e)    => write!(f, "parse: {e}"),
            MultiError::Io(e)       => write!(f, "io: {e}"),
            MultiError::OutOfRange(n) => write!(f, "out of range: {n}"),
        }
    }
}
impl std::error::Error for MultiError {}

impl From<ParseIntError> for MultiError {
    fn from(e: ParseIntError) -> Self { MultiError::Parse(e) }
}
impl From<io::Error> for MultiError {
    fn from(e: io::Error) -> Self { MultiError::Io(e) }
}

fn multi_error_type() {
    println!("\n=== Multiple Error Types in One Function ===");

    fn process(input: &str) -> Result<i32, MultiError> {
        let n: i32 = input.trim().parse()?; // ParseIntError → MultiError
        if !(0..=100).contains(&n) {
            return Err(MultiError::OutOfRange(n));
        }
        Ok(n)
    }

    for input in &["42", "abc", "150"] {
        match process(input) {
            Ok(n)  => println!("  '{input}' → {n}"),
            Err(e) => println!("  '{input}' → {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_ok() {
        let r: Result<Vec<i32>, _> = vec!["1","2","3"].iter()
            .map(|s| s.parse::<i32>())
            .collect();
        assert_eq!(r.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn collect_fails_on_first_error() {
        let r: Result<Vec<i32>, _> = vec!["1","bad","3"].iter()
            .map(|s| s.parse::<i32>())
            .collect();
        assert!(r.is_err());
    }
}
