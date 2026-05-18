// ============================================================
// CONCEPT: Serde — Serialization and Deserialization
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# serialization:
//   using System.Text.Json;
//   var json = JsonSerializer.Serialize(obj);
//   var obj  = JsonSerializer.Deserialize<MyType>(json);
//
//   [JsonPropertyName("user_name")]  // attribute on field
//   public string UserName { get; set; }
//
// Rust serialization with serde + serde_json:
//   use serde::{Serialize, Deserialize};
//   let json = serde_json::to_string(&obj)?;
//   let obj: MyType = serde_json::from_str(&json)?;
//
//   #[serde(rename = "user_name")]   // attribute on field
//   pub user_name: String,
//
// Serde is the universal serialization framework in Rust.
// The same #[derive(Serialize, Deserialize)] works for JSON,
// TOML, YAML, MessagePack, CSV, and 30+ other formats — just
// swap the crate. No code change to your struct.
//
// C# equivalent: implementing IJsonSerializable for one format,
// then needing Newtonsoft for another, then CsvHelper for CSV, etc.
//
// RUN: cargo run --bin json_basics
// ============================================================

use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() {
    println!("=== Serde — JSON Serialization Basics ===\n");

    basic_serialize_deserialize();
    nested_structs();
    enums_in_json();
    option_fields();
    dynamic_json_value();
    json_to_file_pattern();
}

// ─── 1. BASIC SERIALIZE / DESERIALIZE ───────────────────────────────────────
//
// Step 1: derive Serialize and Deserialize — that's it.
// Serde's derive macros generate all the boilerplate at compile time.
//
// C#:
//   public record User(string Username, int Age, string Email);
//   var json = JsonSerializer.Serialize(user);
//   var user = JsonSerializer.Deserialize<User>(json);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    username: String,
    age: u32,
    email: String,
}

fn basic_serialize_deserialize() {
    println!("--- 1. Basic Serialize / Deserialize ---\n");

    let user = User {
        username: "alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Serialize to JSON string — like JsonSerializer.Serialize():
    let json = serde_json::to_string(&user).unwrap();
    println!("Serialized:   {json}");

    // Pretty-printed — like JsonSerializer.Serialize(obj, new JsonSerializerOptions {{ WriteIndented = true }}):
    let pretty = serde_json::to_string_pretty(&user).unwrap();
    println!("Pretty:\n{pretty}");

    // Deserialize from JSON string — like JsonSerializer.Deserialize<User>():
    let json_input = r#"{"username":"bob","age":25,"email":"bob@example.com"}"#;
    let parsed: User = serde_json::from_str(json_input).unwrap();
    println!("Deserialized: {parsed:?}");

    // Round-trip check:
    let round_trip: User = serde_json::from_str(&json).unwrap();
    assert_eq!(user, round_trip);
    println!("Round-trip:   OK\n");

    println!(r#"  C#                                       Rust
  ─────────────────────────────────────────────────────────────────
  [JsonSerializable] / record             #[derive(Serialize, Deserialize)]
  JsonSerializer.Serialize(obj)           serde_json::to_string(&obj)?
  JsonSerializer.Serialize(obj, opts)     serde_json::to_string_pretty(&obj)?
  JsonSerializer.Deserialize<T>(json)     serde_json::from_str::<T>(&json)?
  JsonSerializer.Deserialize<T>(stream)   serde_json::from_reader(reader)?
"#);
}

// ─── 2. NESTED STRUCTS ──────────────────────────────────────────────────────
//
// Nested types just need their own Serialize + Deserialize derives.
// No extra configuration — it composes automatically.
//
// C#:
//   public record Address(string Street, string City, string Country);
//   public record Person(string Name, Address HomeAddress, List<string> Hobbies);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    address: Address,       // nested struct — serializes as nested object
    hobbies: Vec<String>,   // Vec → JSON array
    scores: Vec<f64>,
}

fn nested_structs() {
    println!("--- 2. Nested Structs ---\n");

    let person = Person {
        name: "Carol".to_string(),
        age: 28,
        address: Address {
            street: "123 Rust Lane".to_string(),
            city: "Ferrisville".to_string(),
            country: "US".to_string(),
        },
        hobbies: vec!["coding".to_string(), "hiking".to_string()],
        scores: vec![9.5, 8.8, 9.2],
    };

    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("{json}\n");

    // Deserialize back:
    let back: Person = serde_json::from_str(&json).unwrap();
    println!("Name:    {}", back.name);
    println!("City:    {}", back.address.city);
    println!("Hobbies: {:?}\n", back.hobbies);
}

// ─── 3. ENUMS IN JSON ───────────────────────────────────────────────────────
//
// Rust enums with data map to JSON in different ways depending on
// the #[serde(tag)] attribute (see advanced_serde.rs for all variants).
// Default: externally-tagged {"Variant": data}.
//
// C#:
//   [JsonPolymorphic]
//   [JsonDerivedType(typeof(Dog), "dog")]
//   abstract class Animal { }
//   class Dog : Animal { public string Breed { get; set; } }

#[derive(Debug, Serialize, Deserialize)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Active,             // unit variant → serializes as "Active"
    Pending,
    Closed { reason: String },
}

fn enums_in_json() {
    println!("--- 3. Enums in JSON ---\n");

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 3.0, height: 4.0 },
    ];

    for shape in &shapes {
        println!("{}", serde_json::to_string(shape).unwrap());
    }
    println!();

    let statuses = vec![
        Status::Active,
        Status::Pending,
        Status::Closed { reason: "Expired".to_string() },
    ];

    for s in &statuses {
        println!("{}", serde_json::to_string(s).unwrap());
    }

    // Deserialize enum from string:
    let json = r#"{"Circle":{"radius":7.5}}"#;
    let shape: Shape = serde_json::from_str(json).unwrap();
    println!("\nDeserialized: {shape:?}\n");
}

// ─── 4. OPTION FIELDS ───────────────────────────────────────────────────────
//
// Option<T> maps to null in JSON when None, or the value when Some.
// This is exactly how C# nullable reference types behave.
//
// C#:
//   public record Profile(string Username, string? Bio, int? Age);
//   // Bio = null → "bio": null in JSON

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    username: String,
    bio: Option<String>,    // None → null in JSON
    website: Option<String>,
    age: Option<u32>,
}

fn option_fields() {
    println!("--- 4. Option Fields (nullable) ---\n");

    let with_all = Profile {
        username: "dave".to_string(),
        bio: Some("Rust enthusiast".to_string()),
        website: Some("https://dave.dev".to_string()),
        age: Some(35),
    };

    let minimal = Profile {
        username: "eve".to_string(),
        bio: None,
        website: None,
        age: None,
    };

    println!("Full profile:    {}", serde_json::to_string(&with_all).unwrap());
    println!("Sparse profile:  {}", serde_json::to_string(&minimal).unwrap());

    // Deserialize with missing fields (they become None):
    let json = r#"{"username":"frank","bio":null,"website":null,"age":null}"#;
    let parsed: Profile = serde_json::from_str(json).unwrap();
    println!("Parsed bio:      {:?}\n", parsed.bio);

    println!(r#"  C# nullable                   Rust Option<T>
  ─────────────────────────────────────────────
  string? Bio = null;           bio: Option<String>  // None
  int? Age = 42;                age: Option<u32>     // Some(42)
  "bio": null in JSON           "bio": null in JSON  (identical)
  "bio": "text" in JSON         "bio": "text" in JSON
"#);
}

// ─── 5. DYNAMIC JSON WITH Value ─────────────────────────────────────────────
//
// Sometimes you don't know the shape of the JSON at compile time.
// serde_json::Value is the dynamic JSON type — like JsonNode/JObject in C#.
//
// C#:
//   using JsonNode = System.Text.Json.Nodes.JsonNode;
//   var node = JsonNode.Parse(json);
//   string name = node!["name"]!.GetValue<string>();

fn dynamic_json_value() {
    println!("--- 5. Dynamic JSON (serde_json::Value) ---\n");

    let json = r#"{
        "name": "Grace",
        "age": 32,
        "active": true,
        "scores": [10, 20, 30],
        "meta": { "role": "admin" }
    }"#;

    // Parse into a dynamic Value — no struct needed:
    let value: Value = serde_json::from_str(json).unwrap();

    // Index with ["key"] — returns Option<&Value>:
    println!("name:   {}", value["name"]);
    println!("age:    {}", value["age"]);
    println!("active: {}", value["active"]);
    println!("score0: {}", value["scores"][0]);
    println!("role:   {}", value["meta"]["role"]);

    // Extract typed values:
    if let Some(name) = value["name"].as_str() {
        println!("name as &str: {name}");
    }
    if let Some(age) = value["age"].as_u64() {
        println!("age as u64:   {age}");
    }

    // Build JSON dynamically with the json! macro:
    let dynamic = serde_json::json!({
        "event": "login",
        "user": "henry",
        "timestamp": 1_716_000_000_u64,
        "metadata": { "ip": "127.0.0.1", "attempts": 1 }
    });
    println!("\nBuilt dynamically:\n{}", serde_json::to_string_pretty(&dynamic).unwrap());

    println!(r#"
  C#                                    Rust
  ─────────────────────────────────────────────────────────────────
  JsonNode.Parse(json)                  serde_json::from_str::<Value>(&json)?
  node["name"].GetValue<string>()       value["name"].as_str()
  node["age"].GetValue<int>()           value["age"].as_u64()
  new JsonObject {{ ["k"] = "v" }}      serde_json::json!({{ "k": "v" }})
  JsonArray                             Value::Array(Vec<Value>)
  JsonObject                            Value::Object(Map<String, Value>)
"#);
}

// ─── 6. FILE I/O PATTERN ────────────────────────────────────────────────────
//
// Real-world pattern: read config from file, write results to file.
// serde_json::from_reader / to_writer work with any io::Read/io::Write.

fn json_to_file_pattern() {
    println!("--- 6. File I/O Pattern ---\n");

    println!(r#"  Reading JSON from a file:
    use std::fs;
    let content = fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&content)?;

    // Or stream directly (no String allocation):
    let file = std::fs::File::open("config.json")?;
    let config: Config = serde_json::from_reader(file)?;

  Writing JSON to a file:
    let file = std::fs::File::create("output.json")?;
    serde_json::to_writer_pretty(file, &data)?;

  C# equivalent:
    var json = File.ReadAllText("config.json");
    var config = JsonSerializer.Deserialize<Config>(json);
    File.WriteAllText("output.json", JsonSerializer.Serialize(data));

  Key advantage: serde_json::from_reader streams the parse —
  no need to load the entire file into a String first.
"#);
}

// ─── TESTS ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_round_trip() {
        let u = User { username: "test".to_string(), age: 20, email: "t@t.com".to_string() };
        let json = serde_json::to_string(&u).unwrap();
        let back: User = serde_json::from_str(&json).unwrap();
        assert_eq!(u, back);
    }

    #[test]
    fn option_none_serializes_as_null() {
        let p = Profile { username: "x".to_string(), bio: None, website: None, age: None };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("null"));
    }

    #[test]
    fn json_macro_builds_value() {
        let v = serde_json::json!({ "key": 42 });
        assert_eq!(v["key"], 42);
    }

    #[test]
    fn enum_unit_variant_serializes_as_string() {
        let s = Status::Active;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, r#""Active""#);
    }

    #[test]
    fn deserialize_unknown_field_fails_gracefully() {
        // Extra fields in JSON are ignored by default:
        let json = r#"{"username":"z","age":1,"email":"z@z.com","extra":"ignored"}"#;
        let u: User = serde_json::from_str(json).unwrap();
        assert_eq!(u.username, "z");
    }
}
