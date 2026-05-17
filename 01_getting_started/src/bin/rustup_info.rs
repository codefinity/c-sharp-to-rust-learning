// ============================================================
// CONCEPT: rustup — The Rust Toolchain Installer
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# uses the .NET SDK which you install once. Rust has `rustup`, a
// toolchain manager that lets you install, update, and switch between
// stable/beta/nightly Rust versions per project or globally.
//
// RUSTUP CHEAT SHEET
// ------------------
//   rustup show                     # show active toolchain + targets
//   rustup update                   # update all installed toolchains
//   rustup toolchain install nightly # install a specific toolchain
//   rustup override set 1.85.0      # pin a project to a specific version
//   rustup target add wasm32-unknown-unknown # cross-compile target
//   rustup component add rust-analyzer       # LSP server
//   rustup component add rust-src            # standard library source
//   rustup component add llvm-tools          # LLVM utilities (profiling)
//
// RUST-TOOLCHAIN.TOML
// -------------------
// Place a rust-toolchain.toml at the project root to pin the toolchain:
//
//   [toolchain]
//   channel  = "1.95.0"
//   components = ["rust-analyzer", "clippy", "rustfmt"]
//
// This is similar to global.json for .NET SDK version pinning.
//
// RUN: cargo run --bin rustup_info
// ============================================================

fn main() {
    println!("=== Rust Toolchain Information ===\n");

    // RUSTC_VERSION and similar are not standard env vars at runtime, but
    // we can embed compile-time version info using a build script or the
    // rustc_version crate. Here we use the built-in env! macro approach:
    println!("Compiled with: rustc {}", rustc_version_string());

    display_edition_features();
    display_toolchain_components();
    display_target_info();
}

fn rustc_version_string() -> &'static str {
    // env!("CARGO_PKG_RUST_VERSION") gives the minimum version from Cargo.toml.
    // The actual compiler version must be queried via `rustc --version` or
    // a build script. For demonstration we use the workspace minimum:
    env!("CARGO_PKG_RUST_VERSION")
}

fn display_edition_features() {
    println!("--- Edition 2024 key features ---");
    println!(
        r#"
Edition 2024 (stabilised in Rust 1.85.0) brings:
  • Async closures:  let f = async |x: u32| x * 2;
  • RPIT precise capturing: impl Trait + use<'a>
  • if let chains no longer need extra braces in some positions
  • gen blocks (generators) — first-class lazy sequences
  • Newly reserved keywords: gen
  • NLL (Non-Lexical Lifetimes) is the only borrowck mode
  • Standard library additions for async traits

Each edition is opt-in per crate via Cargo.toml:
  edition = "2024"
Old crates compiled with edition = "2015" / "2018" / "2021" still work.
"#
    );
}

fn display_toolchain_components() {
    println!("--- Standard toolchain components ---");
    println!(
        r#"
Component        | Purpose                        | C# analogy
-----------------+--------------------------------+------------------
rustc            | Compiler                       | csc / Roslyn
cargo            | Build & package manager        | dotnet CLI + NuGet
rustfmt          | Code formatter                 | dotnet format
clippy           | Linter / code quality          | Roslyn analyzers
rust-analyzer    | LSP server (IDE support)       | Roslyn LSP
rust-docs        | Standard library HTML docs     | msdocs
rust-src         | Standard library source        | Reference source
llvm-tools       | Profiling / coverage           | dotTrace / dotCover
"#
    );
}

fn display_target_info() {
    // std::env::consts gives us platform information at runtime.
    println!("--- Runtime target triple info ---");
    println!("OS family : {}", std::env::consts::FAMILY);   // "windows" / "unix"
    println!("OS name   : {}", std::env::consts::OS);        // "windows" / "linux" / "macos"
    println!("Arch      : {}", std::env::consts::ARCH);      // "x86_64" / "aarch64"

    println!(
        r#"
Common cross-compilation targets:
  x86_64-unknown-linux-musl    # static Linux binary
  aarch64-unknown-linux-gnu    # ARM64 Linux (e.g. AWS Graviton)
  wasm32-unknown-unknown       # WebAssembly
  x86_64-pc-windows-msvc       # Windows (MSVC linker)
  aarch64-apple-darwin         # Apple Silicon macOS
"#
    );
}

// ─── COMMON MISTAKES ─────────────────────────────────────────
// 1. Forgetting to run `rustup update` — always use the latest stable.
// 2. Mixing nightly features into production code without a feature gate.
// 3. Not adding rust-toolchain.toml — leads to "works on my machine" bugs.

// ─── EXERCISES ───────────────────────────────────────────────
// 1. Run `rustup show` and identify your host triple.
// 2. Create a rust-toolchain.toml pinning this project to 1.95.0.
// 3. Run `rustup component add rust-analyzer` and verify it installed.
// 4. Run `rustup target list --installed`.

#[cfg(test)]
mod tests {
    #[test]
    fn os_family_is_known() {
        let family = std::env::consts::FAMILY;
        assert!(
            family == "windows" || family == "unix",
            "unexpected OS family: {family}"
        );
    }
}
