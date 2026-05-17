// ============================================================
// CONCEPT: Builder Pattern
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# builders: fluent interfaces, object initialisers, record `with`.
// Rust builders: same fluent style but leveraging ownership/move semantics.
//
// The Rust builder idiom consumes `self` (moves) on each setter call
// and returns the modified builder. This eliminates aliasing issues.
//
// RUN: cargo run --bin builder_pattern
// ============================================================

use std::time::Duration;

fn main() {
    println!("=== Builder Pattern ===\n");

    basic_builder();
    validated_builder();
    generic_builder();
}

// ---- 1. Basic fluent builder ---------------------------------------

#[derive(Debug)]
struct HttpClient {
    base_url: String,
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
    bearer_token: Option<String>,
}

// C# analogy: HttpClientBuilder or HttpClient + HttpClientHandler
struct HttpClientBuilder {
    base_url: String,
    timeout: Duration,
    max_retries: u32,
    user_agent: String,
    bearer_token: Option<String>,
}

impl HttpClientBuilder {
    fn new(base_url: impl Into<String>) -> Self {
        HttpClientBuilder {
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            user_agent: "rust-client/1.0".to_string(),
            bearer_token: None,
        }
    }

    // Each setter consumes self and returns Self (move-based fluent API):
    fn timeout(mut self, t: Duration) -> Self {
        self.timeout = t;
        self
    }

    fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.bearer_token = Some(token.into());
        self
    }

    fn build(self) -> HttpClient {
        HttpClient {
            base_url: self.base_url,
            timeout: self.timeout,
            max_retries: self.max_retries,
            user_agent: self.user_agent,
            bearer_token: self.bearer_token,
        }
    }
}

fn basic_builder() {
    println!("--- Basic Fluent Builder ---");

    // C# analogy:
    //   new HttpClientBuilder("https://api.example.com")
    //       .WithTimeout(TimeSpan.FromSeconds(10))
    //       .WithBearerToken("secret")
    //       .Build();
    let client = HttpClientBuilder::new("https://api.example.com")
        .timeout(Duration::from_secs(10))
        .max_retries(5)
        .user_agent("my-app/2.0")
        .bearer_token("supersecret")
        .build();

    println!("{client:#?}");
}

// ---- 2. Builder with validation ------------------------------------

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
    tls: bool,
}

#[derive(Debug)]
struct ServerConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    workers: Option<usize>,
    tls: bool,
}

#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error("host is required")]
    MissingHost,
    #[error("port must be between 1 and 65535")]
    InvalidPort,
    #[error("workers must be >= 1")]
    InvalidWorkers,
}

impl ServerConfigBuilder {
    fn new() -> Self {
        ServerConfigBuilder {
            host: None,
            port: None,
            workers: None,
            tls: false,
        }
    }

    fn host(mut self, h: impl Into<String>) -> Self {
        self.host = Some(h.into());
        self
    }

    fn port(mut self, p: u16) -> Self {
        self.port = Some(p);
        self
    }

    fn workers(mut self, n: usize) -> Self {
        self.workers = Some(n);
        self
    }

    fn tls(mut self, enabled: bool) -> Self {
        self.tls = enabled;
        self
    }

    fn build(self) -> Result<ServerConfig, ConfigError> {
        let host = self.host.ok_or(ConfigError::MissingHost)?;
        let port = self.port.unwrap_or(8080);
        if port == 0 { return Err(ConfigError::InvalidPort); }
        let workers = self.workers.unwrap_or_else(num_cpus);
        if workers == 0 { return Err(ConfigError::InvalidWorkers); }

        Ok(ServerConfig { host, port, workers, tls: self.tls })
    }
}

fn num_cpus() -> usize {
    // Simplified — real code uses the `num_cpus` crate
    4
}

fn validated_builder() {
    println!("\n--- Validated Builder (Result<T, Error>) ---");

    let cfg = ServerConfigBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .tls(true)
        .build();
    println!("valid config: {:?}", cfg);

    let bad = ServerConfigBuilder::new().build();  // missing host
    println!("missing host: {:?}", bad);
}

// ---- 3. Generic builder (phantom-typed) ----------------------------

// A query builder that uses generic state to prevent building
// without setting required fields at the TYPE level:

use std::marker::PhantomData;

struct Unset;
struct Set<T>(PhantomData<T>);

struct QueryBuilder<HasTable, HasSelect> {
    table: String,
    select: Vec<String>,
    where_clause: Option<String>,
    limit: Option<usize>,
    _table: PhantomData<HasTable>,
    _select: PhantomData<HasSelect>,
}

impl QueryBuilder<Unset, Unset> {
    fn new() -> Self {
        QueryBuilder {
            table: String::new(),
            select: vec!["*".to_string()],
            where_clause: None,
            limit: None,
            _table: PhantomData,
            _select: PhantomData,
        }
    }
}

impl<S> QueryBuilder<Unset, S> {
    fn from(mut self, table: impl Into<String>) -> QueryBuilder<Set<String>, S> {
        self.table = table.into();
        QueryBuilder {
            table: self.table,
            select: self.select,
            where_clause: self.where_clause,
            limit: self.limit,
            _table: PhantomData,
            _select: PhantomData,
        }
    }
}

impl<T> QueryBuilder<T, Unset> {
    fn select(mut self, cols: &[&str]) -> QueryBuilder<T, Set<Vec<String>>> {
        self.select = cols.iter().map(|s| s.to_string()).collect();
        QueryBuilder {
            table: self.table,
            select: self.select,
            where_clause: self.where_clause,
            limit: self.limit,
            _table: PhantomData,
            _select: PhantomData,
        }
    }
}

// Only callable when BOTH table and select have been set:
impl QueryBuilder<Set<String>, Set<Vec<String>>> {
    fn where_clause(mut self, cond: impl Into<String>) -> Self {
        self.where_clause = Some(cond.into());
        self
    }

    fn limit(mut self, n: usize) -> Self {
        self.limit = Some(n);
        self
    }

    fn build(self) -> String {
        let cols = self.select.join(", ");
        let mut sql = format!("SELECT {} FROM {}", cols, self.table);
        if let Some(w) = self.where_clause {
            sql.push_str(&format!(" WHERE {w}"));
        }
        if let Some(l) = self.limit {
            sql.push_str(&format!(" LIMIT {l}"));
        }
        sql
    }
}

fn generic_builder() {
    println!("\n--- Phantom-Typed Builder (compile-time required fields) ---");

    let sql = QueryBuilder::new()
        .from("users")
        .select(&["id", "name", "email"])
        .where_clause("active = true")
        .limit(10)
        .build();
    println!("SQL: {sql}");

    // This would be a COMPILE ERROR — .build() not available without .from():
    // QueryBuilder::new().select(&["*"]).build();

    println!("(Calling .build() without .from() is a compile error!)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_builder() {
        let c = HttpClientBuilder::new("http://localhost")
            .timeout(Duration::from_secs(5))
            .build();
        assert_eq!(c.base_url, "http://localhost");
        assert_eq!(c.timeout, Duration::from_secs(5));
    }

    #[test]
    fn server_config_missing_host() {
        let r = ServerConfigBuilder::new().build();
        assert!(r.is_err());
    }

    #[test]
    fn query_builder() {
        let sql = QueryBuilder::new()
            .from("orders")
            .select(&["id", "total"])
            .build();
        assert!(sql.contains("FROM orders"));
        assert!(sql.contains("id, total"));
    }
}
