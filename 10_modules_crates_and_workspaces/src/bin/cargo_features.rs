// ============================================================
// CONCEPT: Cargo Features and Conditional Compilation
// ============================================================
// RUN: cargo run --bin cargo_features
// ============================================================

fn main() {
    conditional_compilation();
    features_demo();
    build_info();
}

fn conditional_compilation() {
    println!("=== Conditional Compilation (#[cfg]) ===");

    // cfg attributes check compile-time conditions.
    // C# analogy: #if DEBUG / #if NETCOREAPP / [Conditional("DEBUG")]

    // cfg on a function:
    #[cfg(debug_assertions)]
    fn debug_only() -> &'static str { "debug mode" }

    #[cfg(not(debug_assertions))]
    fn debug_only() -> &'static str { "release mode" }

    println!("build mode: {}", debug_only());

    // cfg on platform:
    #[cfg(target_os = "windows")]
    println!("Running on Windows");

    #[cfg(target_os = "linux")]
    println!("Running on Linux");

    #[cfg(target_os = "macos")]
    println!("Running on macOS");

    // Target architecture:
    #[cfg(target_arch = "x86_64")]
    println!("64-bit x86");

    #[cfg(target_arch = "aarch64")]
    println!("ARM64");

    // Runtime checks (cfg! macro returns bool):
    let is_windows = cfg!(target_os = "windows");
    let is_debug   = cfg!(debug_assertions);
    println!("is_windows: {is_windows}  is_debug: {is_debug}");

    // cfg in struct fields / variants:
    #[allow(dead_code)]
    struct PlatformData {
        common: String,
        #[cfg(target_os = "windows")]
        win_handle: u64,
        #[cfg(not(target_os = "windows"))]
        unix_fd: i32,
    }
    println!("PlatformData size: {}", std::mem::size_of::<PlatformData>());
}

fn features_demo() {
    println!("\n=== Cargo Features ===");

    println!(
        r#"
Cargo features are optional compilation flags — like C# conditional compilation
symbols but declared in Cargo.toml and usable by dependents.

In Cargo.toml:
  [features]
  default = ["std"]        # enabled by default
  std     = []             # enables std support
  logging = ["dep:tracing"] # enables tracing crate
  full    = ["std", "logging"]

Use in code:
  #[cfg(feature = "logging")]
  use tracing::info;

  pub fn process(x: i32) -> i32 {{
      #[cfg(feature = "logging")]
      info!("processing {{x}}");
      x * 2
  }}

Enabling features in Cargo.toml:
  [dependencies]
  tokio = {{ version = "1", features = ["full"] }}
  serde = {{ version = "1", features = ["derive"] }}

Running with a feature:
  cargo run --features logging
  cargo build --no-default-features --features std
"#
    );

    // The cfg! macro checks features at runtime (for branching):
    if cfg!(debug_assertions) {
        println!("debug assertions enabled (cargo run or cargo build)");
    } else {
        println!("release mode (cargo build --release)");
    }
}

fn build_info() {
    println!("\n=== Build-Time Info from Cargo ===");

    // Environment variables set by Cargo during build:
    println!("Package name:    {}", env!("CARGO_PKG_NAME"));
    println!("Package version: {}", env!("CARGO_PKG_VERSION"));
    println!("Authors:         {}", env!("CARGO_PKG_AUTHORS"));
    println!("Manifest dir:    {}", env!("CARGO_MANIFEST_DIR"));

    // These are set only during cargo builds (not `rustc` directly).
    // Useful for embedding version info in your binary.

    // option_env! — returns Option<&str>, useful for optional env vars:
    let opt = option_env!("MY_CUSTOM_VAR");
    println!("MY_CUSTOM_VAR: {:?}", opt);

    println!(
        r#"
Build Scripts (build.rs):
  A build.rs file at the crate root runs before compilation.
  Use cases:
    • Generate code (e.g., protobuf, WASM bindings)
    • Compile C/C++ code (via `cc` crate)
    • Set cargo:rustc-cfg = ... (add cfg flags)
    • Set cargo:rustc-link-lib = ... (link libraries)
    • Set cargo:rerun-if-changed=file (invalidate cache)

  Example build.rs:
    fn main() {{
        println!("cargo:rustc-cfg=my_feature");
        println!("cargo:rerun-if-changed=build.rs");
    }}
"#
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn cfg_macro_works() {
        let _ = cfg!(target_os = "windows");
        // just verify it compiles — actual value depends on platform
    }

    #[test]
    fn env_vars_set() {
        let name = env!("CARGO_PKG_NAME");
        assert!(!name.is_empty());
    }
}
