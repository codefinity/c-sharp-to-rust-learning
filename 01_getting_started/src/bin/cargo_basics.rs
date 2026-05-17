// ============================================================
// CONCEPT: Cargo — Rust's Build System and Package Manager
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses MSBuild (.csproj) + NuGet for packages. Rust uses Cargo, which
// handles building, testing, formatting, linting, documentation, and
// dependency management in a single coherent tool.
//
// CARGO CHEAT SHEET
// -----------------
//   cargo new my_project        # like `dotnet new console -n my_project`
//   cargo new --lib my_lib      # like `dotnet new classlib -n my_lib`
//   cargo build                 # like `dotnet build`
//   cargo build --release       # optimized build (like Release config)
//   cargo run                   # like `dotnet run`
//   cargo run --bin hello_world # run a specific binary
//   cargo test                  # like `dotnet test`
//   cargo fmt                   # like `dotnet format`
//   cargo clippy                # Roslyn analyzer equivalent
//   cargo doc --open            # generate + open HTML docs
//   cargo add serde             # like `dotnet add package serde`
//   cargo update                # like `dotnet restore` (updates lock file)
//   cargo clean                 # like `dotnet clean`
//   cargo check                 # type-check without linking (very fast)
//   cargo publish               # publish to crates.io (like nuget push)
//
// RUN: cargo run --bin cargo_basics
// ============================================================

fn main() {
    println!("=== Cargo Basics Demo ===\n");

    // This binary itself demonstrates how a workspace member with multiple
    // [[bin]] targets works. Each [[bin]] entry in Cargo.toml maps a name to
    // a source file under src/bin/.

    demonstrate_cargo_concepts();
    demonstrate_workspace_concepts();
    show_environment_info();
}

fn demonstrate_cargo_concepts() {
    println!("--- Cargo.toml anatomy ---");
    println!(
        r#"
[package]
name    = "my_crate"           # crate name (like AssemblyName)
version = "0.1.0"              # SemVer (MAJOR.MINOR.PATCH)
edition = "2024"               # Rust edition — NOT the language version

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
# ^ Similar to <PackageReference> in .csproj

[dev-dependencies]
proptest = "1"                 # only for tests/benches (like TestOnly NuGet)

[[bin]]
name = "my_app"                # cargo run --bin my_app
path = "src/bin/my_app.rs"
"#
    );
}

fn demonstrate_workspace_concepts() {
    println!("--- Workspace concepts ---");
    println!(
        r#"
A Cargo workspace groups multiple crates under one root Cargo.toml.
This mirrors a .NET Solution (.sln) containing multiple projects.

Root Cargo.toml:
  [workspace]
  members = ["crate_a", "crate_b"]

Benefits:
  - Shared dependency lock file (Cargo.lock) → reproducible builds
  - `cargo build` / `cargo test` operate on all members at once
  - [workspace.dependencies] lets you centralise version pins
"#
    );
}

fn show_environment_info() {
    // The CARGO_* environment variables are set by Cargo during builds.
    // Useful for conditional compilation and self-description.
    let pkg_name    = env!("CARGO_PKG_NAME");     // crate name from Cargo.toml
    let pkg_version = env!("CARGO_PKG_VERSION");  // version string
    let manifest    = env!("CARGO_MANIFEST_DIR"); // path to Cargo.toml dir

    println!("--- Build-time environment (from Cargo) ---");
    println!("Package : {pkg_name}");
    println!("Version : {pkg_version}");
    println!("Manifest: {manifest}");

    // RUST_EDITION is not a standard env var, but we can embed our edition
    // via build.rs if needed. The edition is a compile-time concept.
    println!("\nRust edition in use: 2024");
}

// ─── KEY DIFFERENCES FROM C# / MSBuild / NuGet ───────────────
// 1. Cargo.lock is for applications; .gitignore it for libraries (like
//    packages.lock.json behaviour in NuGet).
// 2. `cargo check` is vastly faster than a full build — use it in watch mode.
// 3. Features in Cargo are like conditional compilation symbols but finer-
//    grained and opt-in per dependency.
// 4. No global package cache per-project — Cargo uses ~/.cargo/registry.
// 5. Edition changes are backward-compatible; old crates continue to work.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Run `cargo check` — how does the output differ from `cargo build`?
// 2. Add `rand = "0.9"` to this crate's [dev-dependencies] and use it in
//    a test.
// 3. Run `cargo doc --open` and explore the generated documentation.
// 4. Run `cargo clippy` and observe any lint warnings.

#[cfg(test)]
mod tests {
    #[test]
    fn pkg_name_is_not_empty() {
        let name = env!("CARGO_PKG_NAME");
        assert!(!name.is_empty());
    }

    #[test]
    fn pkg_version_is_semver_like() {
        let version = env!("CARGO_PKG_VERSION");
        let parts: Vec<&str> = version.split('.').collect();
        assert_eq!(parts.len(), 3, "version should be MAJOR.MINOR.PATCH");
    }
}
