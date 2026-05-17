// ============================================================
// CONCEPT: thiserror — Ergonomic Custom Error Types (Library Code)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// In C#, custom exceptions inherit from Exception and override Message.
// In Rust, you implement std::error::Error + Display for custom errors.
// `thiserror` is a derive macro that generates the boilerplate.
//
// C# custom exception:
//   class DatabaseException : Exception {
//       public DatabaseException(string msg) : base(msg) {}
//       public DatabaseException(string msg, Exception inner) : base(msg, inner) {}
//   }
//
// Rust with thiserror:
//   #[derive(Debug, thiserror::Error)]
//   enum DatabaseError {
//       #[error("connection failed: {0}")]
//       Connection(#[from] io::Error),
//   }
//
// USE thiserror WHEN:
//   • Writing a library crate
//   • You need typed, inspectable errors
//   • Callers need to match on error variants
//
// RUN: cargo run --bin thiserror_example
// ============================================================

use thiserror::Error;
use std::num::ParseIntError;
use std::io;

fn main() {
    basic_thiserror();
    error_chains();
    error_hierarchy();
    converting_errors();
}

// ─── BASIC THISERROR USAGE ───────────────────────────────────

#[derive(Debug, Error)]
enum ConfigError {
    #[error("missing field: {field}")]
    MissingField { field: String },

    #[error("invalid value '{value}' for field '{field}': {reason}")]
    InvalidValue {
        field:  String,
        value:  String,
        reason: String,
    },

    #[error("parse error in field '{field}'")]
    ParseError {
        field: String,
        #[source] // marks this as the cause (like InnerException in C#)
        source: ParseIntError,
    },

    #[error("I/O error reading config")]
    Io(#[from] io::Error), // #[from] generates From<io::Error> for ConfigError
}

fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let port = s.parse::<u16>().map_err(|e| ConfigError::ParseError {
        field: "port".into(),
        source: e,
    })?;
    if port < 1024 {
        return Err(ConfigError::InvalidValue {
            field: "port".into(),
            value: s.into(),
            reason: "privileged ports (<1024) are not allowed".into(),
        });
    }
    Ok(port)
}

fn basic_thiserror() {
    println!("=== Basic thiserror Usage ===");

    let inputs = ["8080", "80", "abc", ""];
    for input in inputs {
        match parse_port(input) {
            Ok(port) => println!("  '{input}' → port: {port}"),
            Err(e)   => println!("  '{input}' → error: {e}"),
        }
    }

    // Access the source (inner error) — like InnerException in C#:
    let err = parse_port("abc").unwrap_err();
    if let ConfigError::ParseError { field, source } = &err {
        println!("\nError source chain:");
        println!("  ConfigError: {err}");
        println!("  field: {field}");
        println!("  source: {source}");
        // Walk the error chain:
        use std::error::Error;
        let mut cause = err.source();
        while let Some(e) = cause {
            println!("  caused by: {e}");
            cause = e.source();
        }
    }
}

// ─── ERROR CHAINS ────────────────────────────────────────────

#[derive(Debug, Error)]
enum AppError {
    #[error("configuration error")]
    Config(#[from] ConfigError),

    #[error("database error: {message}")]
    Database { message: String },

    #[error("network timeout after {seconds}s")]
    Timeout { seconds: u64 },
}

fn load_app_config(port_str: &str) -> Result<u16, AppError> {
    let port = parse_port(port_str)?; // ConfigError → AppError via From
    Ok(port)
}

fn error_chains() {
    println!("\n=== Error Chains (wrapping errors) ===");

    match load_app_config("not-a-port") {
        Ok(port) => println!("port: {port}"),
        Err(e) => {
            println!("AppError: {e}");
            // Walk the chain:
            use std::error::Error;
            let mut source = e.source();
            let mut depth = 1;
            while let Some(cause) = source {
                println!("  {:>depth$} caused by: {cause}", "", depth = depth * 2);
                source = cause.source();
                depth += 1;
            }
        }
    }
}

// ─── HIERARCHICAL ERRORS ─────────────────────────────────────

#[derive(Debug, Error)]
enum ValidationError {
    #[error("field '{0}' is required")]
    Required(String),

    #[error("field '{field}' must be between {min} and {max}, got {actual}")]
    OutOfRange { field: String, min: i64, max: i64, actual: i64 },

    #[error("field '{field}' has invalid format: {message}")]
    InvalidFormat { field: String, message: String },
}

#[derive(Debug, Error)]
enum UserRegistrationError {
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationError),

    #[error("username '{0}' is already taken")]
    DuplicateUsername(String),

    #[error("email '{0}' is already registered")]
    DuplicateEmail(String),
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.is_empty() {
        return Err(ValidationError::Required("username".into()));
    }
    if username.len() < 3 || username.len() > 20 {
        return Err(ValidationError::OutOfRange {
            field: "username".into(),
            min: 3, max: 20,
            actual: username.len() as i64,
        });
    }
    Ok(())
}

fn register_user(username: &str, email: &str) -> Result<(), UserRegistrationError> {
    validate_username(username)?; // ValidationError → UserRegistrationError via From
    if username == "admin" {
        return Err(UserRegistrationError::DuplicateUsername(username.into()));
    }
    if email == "taken@example.com" {
        return Err(UserRegistrationError::DuplicateEmail(email.into()));
    }
    println!("  Registered: {username} <{email}>");
    Ok(())
}

fn error_hierarchy() {
    println!("\n=== Error Hierarchy ===");

    let test_cases = [
        ("alice", "alice@example.com"),
        ("", "x@y.com"),
        ("ab", "x@y.com"),
        ("admin", "admin@example.com"),
        ("bob", "taken@example.com"),
    ];

    for (username, email) in test_cases {
        match register_user(username, email) {
            Ok(())  => {},
            Err(e)  => println!("  Registration failed for '{username}': {e}"),
        }
    }
}

fn converting_errors() {
    println!("\n=== Pattern: from() vs map_err() ===");

    // #[from] generates: impl From<IoError> for MyError { ... }
    // This lets `?` convert automatically.
    //
    // Without #[from], use .map_err():

    fn read_config() -> Result<String, ConfigError> {
        // Simulate reading from a file — if it fails, map the io::Error:
        let content = std::fs::read_to_string("nonexistent.toml")
            .map_err(|e| {
                println!("  mapping io::Error to ConfigError::Io");
                ConfigError::Io(e)
            })?;
        Ok(content)
    }

    // With #[from], this is equivalent:
    fn read_config_with_from() -> Result<String, ConfigError> {
        let content = std::fs::read_to_string("nonexistent.toml")?;
        // ^ ?  auto-converts io::Error → ConfigError via From impl from #[from]
        Ok(content)
    }

    match read_config() {
        Ok(s) => println!("config: {s}"),
        Err(e) => println!("  read_config: {e}"),
    }
    match read_config_with_from() {
        Ok(s) => println!("config: {s}"),
        Err(e) => println!("  read_config_with_from: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_port() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
    }

    #[test]
    fn privileged_port_rejected() {
        assert!(matches!(parse_port("80"), Err(ConfigError::InvalidValue { .. })));
    }

    #[test]
    fn invalid_port_string() {
        assert!(matches!(parse_port("abc"), Err(ConfigError::ParseError { .. })));
    }

    #[test]
    fn validate_username_empty() {
        assert!(matches!(validate_username(""), Err(ValidationError::Required(_))));
    }

    #[test]
    fn validate_username_too_short() {
        assert!(matches!(validate_username("ab"), Err(ValidationError::OutOfRange { .. })));
    }
}
