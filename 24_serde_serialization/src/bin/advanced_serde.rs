// ============================================================
// CONCEPT: Advanced Serde — Attributes, Enums, Custom Serializers
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# gives you [JsonPropertyName], [JsonIgnore], [JsonConverter].
// Serde attributes do the same, but more powerfully and at zero
// runtime cost (all logic generated at compile time).
//
// This file covers the attributes you'll use in real projects:
//   #[serde(rename)]          ← [JsonPropertyName]
//   #[serde(rename_all)]      ← JsonNamingPolicy.CamelCase
//   #[serde(skip)]            ← [JsonIgnore]
//   #[serde(skip_serializing_if)] ← conditional omission
//   #[serde(default)]         ← missing field gets default value
//   #[serde(flatten)]         ← merge a struct into parent
//   #[serde(tag)]             ← polymorphic JSON (discriminator field)
//   #[serde(alias)]           ← accept multiple field names
//
// RUN: cargo run --bin advanced_serde
// ============================================================

use serde::{Deserialize, Serialize};

fn main() {
    println!("=== Advanced Serde Attributes ===\n");

    demo_rename();
    demo_skip_fields();
    demo_default_values();
    demo_flatten();
    demo_enum_representations();
    demo_alias();
    demo_type_aliases();
}

// ─── 1. RENAMING FIELDS ─────────────────────────────────────────────────────
//
// C# APIs often use camelCase JSON while Rust uses snake_case internally.
// #[serde(rename_all = "camelCase")] converts all fields automatically.
// #[serde(rename = "specific_name")] overrides a single field.
//
// C#:
//   [JsonPropertyName("firstName")]
//   public string FirstName { get; set; }
//
//   var opts = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };

// rename_all on the container applies to every field:
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]   // Rust: first_name → JSON: firstName
struct ApiUser {
    first_name: String,
    last_name: String,
    email_address: String,

    #[serde(rename = "uid")]         // overrides rename_all for this field
    user_id: u64,
}

fn demo_rename() {
    println!("--- 1. Renaming Fields ---\n");

    let user = ApiUser {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email_address: "jane@doe.com".to_string(),
        user_id: 42,
    };

    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("Serialized (camelCase fields, 'uid' override):\n{json}\n");

    // Deserialize using the renamed keys:
    let json_in = r#"{"firstName":"John","lastName":"Smith","emailAddress":"j@s.com","uid":99}"#;
    let parsed: ApiUser = serde_json::from_str(json_in).unwrap();
    println!("Deserialized: {:?}\n", parsed);

    println!(r#"  rename_all options: "camelCase" | "PascalCase" | "snake_case"
              "SCREAMING_SNAKE_CASE" | "kebab-case" | "lowercase"

  C#                                    Rust
  ─────────────────────────────────────────────────────────────────
  [JsonPropertyName("firstName")]       #[serde(rename = "firstName")]
  JsonNamingPolicy.CamelCase (global)   #[serde(rename_all = "camelCase")] on struct
"#);
}

// ─── 2. SKIPPING FIELDS ─────────────────────────────────────────────────────
//
// C#:
//   [JsonIgnore]
//   public string Password { get; set; }
//
//   [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
//   public string? OptionalField { get; set; }

#[derive(Debug, Serialize, Deserialize)]
struct UserAccount {
    username: String,

    #[serde(skip)]                              // never serialized OR deserialized
    password_hash: String,

    #[serde(skip_serializing)]                 // deserialized but never written to JSON
    internal_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]  // omit from JSON when None
    display_name: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]    // omit from JSON when empty
    tags: Vec<String>,
}

fn demo_skip_fields() {
    println!("--- 2. Skipping Fields ---\n");

    let acc = UserAccount {
        username: "alice".to_string(),
        password_hash: "secret_hash".to_string(),
        internal_token: "tok_123".to_string(),
        display_name: None,      // will be omitted (skip_serializing_if)
        tags: vec![],            // will be omitted (skip_serializing_if)
    };

    let json = serde_json::to_string_pretty(&acc).unwrap();
    println!("Serialized (password_hash, display_name, tags all absent):\n{json}\n");

    let acc2 = UserAccount {
        username: "bob".to_string(),
        password_hash: "secret".to_string(),
        internal_token: "tok_456".to_string(),
        display_name: Some("Bobby".to_string()),
        tags: vec!["admin".to_string(), "beta".to_string()],
    };
    println!("Serialized (display_name and tags present):\n{}\n",
        serde_json::to_string_pretty(&acc2).unwrap());

    println!(r#"  C#                                          Rust
  ─────────────────────────────────────────────────────────────────────
  [JsonIgnore]                                #[serde(skip)]
  [JsonIgnore(WhenWritingNull)]               #[serde(skip_serializing_if = "Option::is_none")]
  [JsonIgnore(WhenWritingDefault)]            #[serde(skip_serializing_if = "...")]
  No read-only skip in STJ                    #[serde(skip_serializing)]
  No write-only skip in STJ                   #[serde(skip_deserializing)]
"#);
}

// ─── 3. DEFAULT VALUES ──────────────────────────────────────────────────────
//
// When a field is missing in the JSON, serde can fill it in with a default.
// C#:
//   public int Version { get; set; } = 1;  // initializer handles missing field

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,

    #[serde(default = "default_port")]   // calls default_port() when field absent
    port: u16,

    #[serde(default)]                    // uses Default::default() — false, 0, "", etc.
    debug: bool,

    #[serde(default = "default_timeout")]
    timeout_secs: u64,
}

fn default_port()    -> u16 { 8080 }
fn default_timeout() -> u64 { 30 }

fn demo_default_values() {
    println!("--- 3. Default Values for Missing Fields ---\n");

    // JSON with only required field:
    let minimal_json = r#"{"host": "localhost"}"#;
    let config: Config = serde_json::from_str(minimal_json).unwrap();
    println!("From minimal JSON: {config:?}");
    assert_eq!(config.port, 8080);
    assert!(!config.debug);
    assert_eq!(config.timeout_secs, 30);

    // JSON with everything specified:
    let full_json = r#"{"host":"prod.example.com","port":443,"debug":true,"timeout_secs":60}"#;
    let full: Config = serde_json::from_str(full_json).unwrap();
    println!("From full JSON:    {full:?}\n");

    println!(r#"  C#                                    Rust
  ─────────────────────────────────────────────────────────────────
  public int Port {{ get; set; }} = 8080  #[serde(default = "fn_name")]
  public bool Debug {{ get; set; }}       #[serde(default)]  // uses Default::default()
"#);
}

// ─── 4. FLATTEN ─────────────────────────────────────────────────────────────
//
// Merge the fields of a nested struct into the parent JSON object.
// Useful for splitting a large struct into logical sub-structs internally
// while presenting a flat JSON shape to the outside world.
//
// C# analogy: manually copying properties, or using a custom JsonConverter.

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    page: u32,
    per_page: u32,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    data: Vec<String>,

    #[serde(flatten)]        // Pagination fields appear at the same level as `data`
    pagination: Pagination,
}

fn demo_flatten() {
    println!("--- 4. Flattening Nested Structs ---\n");

    let resp = ApiResponse {
        data: vec!["item1".to_string(), "item2".to_string()],
        pagination: Pagination { page: 1, per_page: 10, total: 42 },
    };

    let json = serde_json::to_string_pretty(&resp).unwrap();
    println!("Flattened JSON (pagination fields at top level):\n{json}\n");

    // Deserialize from flat JSON back into nested struct:
    let flat = r#"{"data":["a","b"],"page":2,"per_page":10,"total":100}"#;
    let back: ApiResponse = serde_json::from_str(flat).unwrap();
    println!("Deserialized: page={}, total={}\n", back.pagination.page, back.pagination.total);
}

// ─── 5. ENUM REPRESENTATIONS ────────────────────────────────────────────────
//
// Serde supports 4 ways to represent enums in JSON. This is the most
// important thing to understand for API design.

// (A) EXTERNALLY TAGGED — default. {"VariantName": data}
//     Good when variants have different structures.
#[derive(Debug, Serialize, Deserialize)]
enum ExternallyTagged {
    Text(String),
    Number(i64),
    Point { x: f64, y: f64 },
}

// (B) INTERNALLY TAGGED — {"type": "VariantName", ...fields}
//     Good for polymorphic objects — matches C# [JsonPolymorphic] / discriminator.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]                  // adds "type": "VariantName" field
enum InternallyTagged {
    Circle  { radius: f64 },
    Rect    { width: f64, height: f64 },
}

// (C) ADJACENTLY TAGGED — {"t": "VariantName", "c": data}
//     Separates the tag from the content explicitly.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum AdjacentlyTagged {
    Int(i64),
    Str(String),
}

// (D) UNTAGGED — just the data, no discriminator
//     Serde tries each variant in order. Fragile — use sparingly.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Untagged {
    Int(i64),
    Str(String),
    Bool(bool),
}

fn demo_enum_representations() {
    println!("--- 5. Enum JSON Representations ---\n");

    // A) Externally tagged (default):
    let ext = vec![
        ExternallyTagged::Text("hello".to_string()),
        ExternallyTagged::Number(42),
        ExternallyTagged::Point { x: 1.0, y: 2.0 },
    ];
    println!("Externally tagged (default):");
    for e in &ext { println!("  {}", serde_json::to_string(e).unwrap()); }

    // B) Internally tagged — most useful for APIs:
    let int_tagged = vec![
        InternallyTagged::Circle { radius: 5.0 },
        InternallyTagged::Rect { width: 3.0, height: 4.0 },
    ];
    println!("\nInternally tagged (#[serde(tag = \"type\")]):");
    for s in &int_tagged { println!("  {}", serde_json::to_string(s).unwrap()); }

    // C) Adjacently tagged:
    let adj = vec![
        AdjacentlyTagged::Int(99),
        AdjacentlyTagged::Str("world".to_string()),
    ];
    println!("\nAdjacently tagged (#[serde(tag = \"t\", content = \"c\")]):");
    for a in &adj { println!("  {}", serde_json::to_string(a).unwrap()); }

    // D) Untagged:
    let unt = vec![
        Untagged::Int(7),
        Untagged::Str("rust".to_string()),
        Untagged::Bool(true),
    ];
    println!("\nUntagged (#[serde(untagged)]):");
    for u in &unt { println!("  {}", serde_json::to_string(u).unwrap()); }

    println!(r#"
  C#                                    Rust serde equivalent
  ─────────────────────────────────────────────────────────────────────
  [JsonPolymorphic]                     #[serde(tag = "type")]  (B)
  [JsonDerivedType(typeof(T), "tag")]   variant name becomes the discriminator
  JsonDocument / discriminator field    #[serde(tag="t",content="c")]  (C)
  No direct equivalent for (A)/(D)      (A) default / (D) #[serde(untagged)]
"#);
}

// ─── 6. ALIASES ─────────────────────────────────────────────────────────────
//
// Accept multiple JSON field names for the same Rust field.
// Useful when consuming APIs that changed field names between versions.

#[derive(Debug, Deserialize)]
struct LegacyEvent {
    #[serde(alias = "event_name", alias = "eventName")]
    name: String,      // accepts "name", "event_name", or "eventName"

    #[serde(alias = "ts")]
    timestamp: u64,    // accepts "timestamp" or "ts"
}

fn demo_alias() {
    println!("--- 6. Field Aliases ---\n");

    let v1 = r#"{"name":"click","timestamp":1000}"#;
    let v2 = r#"{"event_name":"click","ts":2000}"#;
    let v3 = r#"{"eventName":"click","ts":3000}"#;

    for json in &[v1, v2, v3] {
        let e: LegacyEvent = serde_json::from_str(json).unwrap();
        println!("  name={}, ts={}", e.name, e.timestamp);
    }

    println!(r#"
  C# has no direct equivalent for aliases.
  You'd typically write a custom JsonConverter or accept separate DTOs.

  Rust: #[serde(alias = "other_name")] — multiple aliases allowed.
  The primary field name is tried first; aliases are fallbacks.
"#);
}

// ─── 7. TYPE ALIASES FOR RESULT ─────────────────────────────────────────────
//
// In real projects, define a crate-level Result type alias to avoid
// repeating the error type everywhere.

// Type alias — like C# `using JsonResult<T> = Result<T, JsonError>;`
type JsonResult<T> = Result<T, serde_json::Error>;

fn parse_user(json: &str) -> JsonResult<ApiUser> {
    serde_json::from_str(json)
}

fn demo_type_aliases() {
    println!("--- 7. Type Aliases for Cleaner Error Handling ---\n");

    match parse_user(r#"{"firstName":"X","lastName":"Y","emailAddress":"x@y.com","uid":1}"#) {
        Ok(u)  => println!("  Parsed: {u:?}"),
        Err(e) => println!("  Error: {e}"),
    }

    match parse_user(r#"{"bad": "json"}"#) {
        Ok(_)  => println!("  Parsed (unexpected)"),
        Err(e) => println!("  Expected error: {e}"),
    }

    println!(r#"
  // Define once at crate root:
  type Result<T> = std::result::Result<T, serde_json::Error>;

  // Use everywhere without repeating the error type:
  fn parse(s: &str) -> Result<Config> {{ serde_json::from_str(s) }}

  C# equivalent:
  using JsonResult<T> = System.Threading.Tasks.Task<T>; // rough analogy
"#);
}

// ─── TESTS ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_all_camel_case() {
        let u = ApiUser { first_name: "A".to_string(), last_name: "B".to_string(),
                          email_address: "a@b.com".to_string(), user_id: 1 };
        let json = serde_json::to_string(&u).unwrap();
        assert!(json.contains("firstName"));
        assert!(json.contains("lastName"));
        assert!(json.contains("uid"));           // overrides rename_all
        assert!(!json.contains("user_id"));      // snake_case not present
    }

    #[test]
    fn skip_omits_field() {
        let acc = UserAccount {
            username: "u".to_string(), password_hash: "secret".to_string(),
            internal_token: "tok".to_string(), display_name: None, tags: vec![],
        };
        let json = serde_json::to_string(&acc).unwrap();
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("display_name"));
    }

    #[test]
    fn default_fills_missing_port() {
        let cfg: Config = serde_json::from_str(r#"{"host":"localhost"}"#).unwrap();
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.timeout_secs, 30);
    }

    #[test]
    fn flatten_merges_fields() {
        let json = serde_json::to_string(&ApiResponse {
            data: vec![],
            pagination: Pagination { page: 1, per_page: 10, total: 0 },
        }).unwrap();
        // All fields should be at top level:
        assert!(json.contains("\"page\""));
        assert!(json.contains("\"total\""));
        assert!(!json.contains("\"pagination\""));  // no nesting key
    }

    #[test]
    fn internally_tagged_enum_has_type_field() {
        let s = InternallyTagged::Circle { radius: 1.0 };
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"type\":\"Circle\""));
        assert!(json.contains("\"radius\""));
    }

    #[test]
    fn alias_accepts_alternate_names() {
        let e: LegacyEvent = serde_json::from_str(r#"{"event_name":"click","ts":42}"#).unwrap();
        assert_eq!(e.name, "click");
        assert_eq!(e.timestamp, 42);
    }
}
