// ============================================================
// CONCEPT: A practical CLI file utility
// ============================================================
//
// A simple file-processing CLI demonstrating:
//   - clap derive with file path arguments
//   - anyhow for ergonomic error handling in main
//   - Reading stdin or a file (like Unix tools)
//   - Exit codes
//
// RUN: cargo run --bin file_tool -- count src/bin/file_tool.rs
// RUN: cargo run --bin file_tool -- search --pattern "fn " src/bin/file_tool.rs
// RUN: echo "hello world" | cargo run --bin file_tool -- count -
// ============================================================

use std::fs;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ftool", about = "A simple file utility")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Count lines, words, and bytes in a file (like wc)
    Count {
        /// File to read (use '-' for stdin)
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Search for a pattern in a file (like grep)
    Search {
        /// Pattern to search for
        #[arg(short, long)]
        pattern: String,

        /// File to search
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Show line numbers
        #[arg(short = 'n', long)]
        line_numbers: bool,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },

    /// Show the first N lines of a file (like head)
    Head {
        /// Number of lines
        #[arg(short, long, default_value_t = 10)]
        lines: usize,

        /// File to read
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    // anyhow::Result in main means errors print nicely and exit 1.
    // C# analogy: top-level try/catch + Environment.Exit(1)
    let cli = Cli::parse();

    match cli.command {
        Cmd::Count { file }  => cmd_count(&file),
        Cmd::Search { pattern, file, line_numbers, ignore_case } =>
            cmd_search(&file, &pattern, line_numbers, ignore_case),
        Cmd::Head { lines, file } => cmd_head(&file, lines),
    }
}

// ---- count ---------------------------------------------------------

fn cmd_count(path: &PathBuf) -> Result<()> {
    let content = read_file_or_stdin(path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    println!("{lines:>8} {words:>8} {bytes:>8}  {}", path.display());
    Ok(())
}

// ---- search --------------------------------------------------------

fn cmd_search(path: &PathBuf, pattern: &str, line_numbers: bool, ignore_case: bool) -> Result<()> {
    let content = read_file_or_stdin(path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;

    let pat = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let mut found = 0;
    for (i, line) in content.lines().enumerate() {
        let haystack = if ignore_case { line.to_lowercase() } else { line.to_string() };
        if haystack.contains(&pat) {
            if line_numbers {
                println!("{}:{}", i + 1, line);
            } else {
                println!("{line}");
            }
            found += 1;
        }
    }

    if found == 0 {
        std::process::exit(1); // grep convention: exit 1 if no match
    }
    Ok(())
}

// ---- head ----------------------------------------------------------

fn cmd_head(path: &PathBuf, n: usize) -> Result<()> {
    let content = read_file_or_stdin(path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;

    for line in content.lines().take(n) {
        println!("{line}");
    }
    Ok(())
}

// ---- helpers -------------------------------------------------------

fn read_file_or_stdin(path: &PathBuf) -> Result<String> {
    if path.as_os_str() == "-" {
        // Read from stdin
        let mut buf = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buf)
            .context("failed to read stdin")?;
        Ok(buf)
    } else {
        fs::read_to_string(path)
            .with_context(|| format!("cannot read file '{}'", path.display()))
    }
}

// ---- Key patterns demonstrated ------------------------------------
//
// 1. anyhow::Result<()> as main return type
//    → errors print automatically with context chain
//
// 2. .with_context(|| ...) — add context to errors (like C# inner exceptions)
//
// 3. PathBuf argument type — clap validates the type at parse time
//
// 4. Reading stdin OR a file with "-" convention (Unix standard)
//
// 5. std::process::exit(code) — set exit code (C# Environment.Exit)
//
// 6. Subcommand enum — each command is its own variant with typed args
