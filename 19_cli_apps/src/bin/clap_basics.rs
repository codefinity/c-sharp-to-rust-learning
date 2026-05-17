// ============================================================
// CONCEPT: CLI Apps with clap (derive API)
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# CLI options: System.CommandLine, CommandLineParser, or manual
// args parsing with Environment.GetCommandLineArgs().
//
// Rust: clap is the dominant CLI library. The derive API generates
// argument parsing from struct definitions — very similar to
// System.CommandLine's [Option] attribute approach.
//
// RUN: cargo run --bin clap_basics -- --help
// RUN: cargo run --bin clap_basics -- greet --name Alice --times 3
// RUN: cargo run --bin clap_basics -- math add 10 20
// RUN: cargo run --bin clap_basics -- math mul 3 4
// ============================================================

use clap::{Args, Parser, Subcommand, ValueEnum};

// ---- Top-level CLI struct ------------------------------------------

/// A demo CLI app showcasing clap's derive API.
/// C# analogy: a RootCommand with sub-commands.
#[derive(Parser, Debug)]
#[command(
    name    = "demo",
    version = "1.0.0",
    author  = "Rust Tutorial",
    about   = "Demonstrates clap derive API",
    long_about = None,
)]
struct Cli {
    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

// ---- Subcommands ---------------------------------------------------

#[derive(Subcommand, Debug)]
enum Commands {
    /// Greet a person
    Greet(GreetArgs),

    /// Mathematical operations
    Math {
        #[command(subcommand)]
        op: MathOp,
    },

    /// Print system information
    Info,
}

// ---- Greet subcommand args ----------------------------------------

/// Arguments for the greet subcommand.
/// C# analogy: [Option("--name")] string Name, [Option("--times")] int Times
#[derive(Args, Debug)]
struct GreetArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// How many times to greet
    #[arg(short, long, default_value_t = 1)]
    times: u32,

    /// Greet loudly (uppercase)
    #[arg(long)]
    loud: bool,
}

// ---- Math nested subcommands --------------------------------------

#[derive(Subcommand, Debug)]
enum MathOp {
    /// Add two numbers
    Add {
        /// First operand
        a: f64,
        /// Second operand
        b: f64,
    },
    /// Multiply two numbers
    Mul {
        a: f64,
        b: f64,
    },
    /// Divide two numbers
    Div {
        a: f64,
        b: f64,
    },
}

// ---- ValueEnum for --format option --------------------------------

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
    Csv,
}

// ---- Main ----------------------------------------------------------

fn main() {
    let cli = Cli::parse();

    if cli.verbose > 0 {
        println!("[verbose={}] Parsed: {:?}", cli.verbose, cli);
    }

    match &cli.command {
        Commands::Greet(args) => handle_greet(args, &cli.format),
        Commands::Math { op } => handle_math(op, &cli.format),
        Commands::Info        => handle_info(&cli.format),
    }
}

// ---- Handlers ------------------------------------------------------

fn handle_greet(args: &GreetArgs, format: &OutputFormat) {
    for _ in 0..args.times {
        let msg = if args.loud {
            format!("HELLO, {}!", args.name.to_uppercase())
        } else {
            format!("Hello, {}!", args.name)
        };

        match format {
            OutputFormat::Text => println!("{msg}"),
            OutputFormat::Json => println!("{{\"greeting\":\"{msg}\"}}"),
            OutputFormat::Csv  => println!("greeting,{msg}"),
        }
    }
}

fn handle_math(op: &MathOp, format: &OutputFormat) {
    let (op_name, result) = match op {
        MathOp::Add { a, b } => ("add", a + b),
        MathOp::Mul { a, b } => ("mul", a * b),
        MathOp::Div { a, b } => {
            if *b == 0.0 {
                eprintln!("error: division by zero");
                std::process::exit(1);
            }
            ("div", a / b)
        }
    };

    match format {
        OutputFormat::Text => println!("{op_name} = {result}"),
        OutputFormat::Json => println!("{{\"op\":\"{op_name}\",\"result\":{result}}}"),
        OutputFormat::Csv  => println!("op,result\n{op_name},{result}"),
    }
}

fn handle_info(format: &OutputFormat) {
    let os   = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let rust = env!("CARGO_PKG_RUST_VERSION");

    match format {
        OutputFormat::Text => {
            println!("OS:   {os}");
            println!("Arch: {arch}");
            println!("Rust: {rust}");
        }
        OutputFormat::Json => {
            println!("{{\"os\":\"{os}\",\"arch\":\"{arch}\",\"rust\":\"{rust}\"}}");
        }
        OutputFormat::Csv => {
            println!("key,value");
            println!("os,{os}");
            println!("arch,{arch}");
            println!("rust,{rust}");
        }
    }
}
