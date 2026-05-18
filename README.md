# C# to Rust — A Practical Tutorial for .NET Developers

> **Rust 1.95.0 · Edition 2024 · Cargo Workspace**

A complete, runnable Cargo workspace teaching Rust to experienced C# / .NET developers. Every concept is shown side-by-side with its C# equivalent, each example is self-contained, and all code compiles on Rust 1.95.0.

---

## Table of Contents

1. [Prerequisites & Setup](#prerequisites--setup)
2. [Repository Structure](#repository-structure)
3. [How to Run Examples](#how-to-run-examples)
4. [C# → Rust Concept Map](#c--rust-concept-map)
5. [Module Reference](#module-reference)
6. [1-Week Study Plan](#1-week-study-plan)
7. [Glossary](#glossary)
8. [Capstone Project](#capstone-project)
9. [Recommended Resources](#recommended-resources)

---

## Prerequisites & Setup

### Install Rust 1.95.0

```sh
# Install rustup (skip if already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Linux/macOS
# or download from https://rustup.rs on Windows

# Install Rust 1.95.0 and set as default
rustup install 1.95.0
rustup default 1.95.0

# Verify
rustc --version   # rustc 1.95.0 (...)
cargo --version   # cargo 1.95.0 (...)
```

### Clone and Build

```sh
git clone <this-repo>
cd c-sharp-to-rust-learning

# Check all crates compile (fast — no linking)
cargo check --workspace

# Build everything in release mode
cargo build --workspace --release

# Run all tests across the workspace
cargo test --workspace
```

### VS Code Setup (Recommended)

```sh
# Install rust-analyzer extension
code --install-extension rust-lang.rust-analyzer

# Optional: better error lenses
code --install-extension usernamehw.errorlens
```

---

## Repository Structure

```
c-sharp-to-rust-learning/
├── Cargo.toml                         ← Workspace root (shared deps, edition 2024)
├── README.md                          ← This file
│
├── 01_getting_started/
│   └── src/bin/
│       ├── hello_world.rs             ← println!, format!, basic I/O
│       ├── cargo_basics.rs            ← Cargo commands cheat-sheet
│       └── rustup_info.rs             ← Toolchain management
│
├── 02_syntax_and_basic_types/
│   └── src/bin/
│       ├── variables_mutability.rs    ← let, mut, shadowing, inference
│       ├── scalar_types.rs            ← int/float/bool/char, overflow
│       ├── compound_types.rs          ← tuples, arrays, slices
│       ├── strings.rs                 ← &str vs String, UTF-8
│       ├── constants_statics.rs       ← const, static, const fn
│       └── control_flow.rs            ← if/loop/for/match/if let/let-else
│
├── 03_ownership_borrowing_and_moves/
│   └── src/bin/
│       ├── ownership.rs               ← Three ownership rules, Drop
│       ├── moves.rs                   ← Move semantics, partial moves
│       ├── copy_clone.rs              ← Copy vs Clone traits
│       └── borrowing.rs               ← &T / &mut T, borrow rules, NLL
│
├── 04_references_and_lifetimes/
│   └── src/bin/
│       ├── lifetimes.rs               ← 'a annotations, struct lifetimes
│       ├── lifetime_elision.rs        ← Three elision rules, use<>
│       └── dangling_refs.rs           ← Use-after-free prevention
│
├── 05_structs_enums_and_pattern_matching/
│   └── src/bin/
│       ├── structs.rs                 ← Named/tuple/unit structs, impl
│       ├── enums.rs                   ← ADTs, Option<T>, Result<T,E>
│       ├── pattern_matching.rs        ← match, guards, bindings, nested
│       └── option_result.rs           ← Combinators, ?, From, conversion
│
├── 06_traits_generics_and_impl_blocks/
│   └── src/bin/
│       ├── traits.rs                  ← Trait definition, default methods
│       ├── generics.rs                ← fn/struct generics, where, assoc types
│       ├── trait_objects.rs           ← &dyn Trait, Box<dyn>, fat pointer
│       └── advanced_traits.rs        ← Operator overloading, Deref, Index
│
├── 07_error_handling/
│   └── src/bin/
│       ├── panic_basics.rs            ← panic!, assert!, catch_unwind
│       ├── result_patterns.rs         ← ?, chaining, collect, partition
│       ├── thiserror_example.rs       ← #[derive(Error)], #[from], chains
│       └── anyhow_example.rs          ← anyhow!, bail!, context(), downcast
│
├── 08_collections_and_iterators/
│   └── src/bin/
│       ├── vec_collections.rs         ← Vec, VecDeque, BTreeMap, BTreeSet
│       ├── hashmap_hashset.rs         ← HashMap CRUD, entry API, HashSet
│       ├── iterators.rs               ← All iterator adapters & consumers
│       └── custom_iterators.rs        ← impl Iterator, Fibonacci, BST
│
├── 09_closures_and_functional_patterns/
│   └── src/bin/
│       ├── closures.rs                ← Fn/FnMut/FnOnce, capture modes
│       ├── function_pointers.rs       ← fn type, dispatch tables
│       └── functional_patterns.rs    ← map/filter/fold, currying, Cow
│
├── 10_modules_crates_and_workspaces/
│   └── src/bin/
│       ├── modules.rs                 ← mod, pub, use, crate/super/self
│       ├── visibility.rs              ← pub(crate), pub(super), pub(in)
│       └── cargo_features.rs          ← #[cfg], features, build.rs
│
├── 11_testing_and_documentation/
│   ├── src/
│   │   ├── lib.rs                     ← Library with doc tests, proptest
│   │   └── bin/doc_examples.rs        ← Doc comment styles
│   └── tests/integration_tests.rs    ← Integration tests
│
├── 12_memory_management_and_smart_pointers/
│   └── src/bin/
│       ├── box_pointer.rs             ← Box<T>, recursive types
│       ├── rc_arc.rs                  ← Rc/Arc, Weak, reference cycles
│       ├── cell_refcell.rs            ← Cell<T>, RefCell<T>, Rc<RefCell<T>>
│       ├── mutex_rwlock.rs            ← Mutex<T>, RwLock, Arc<Mutex>
│       └── deref_drop.rs              ← Deref coercion, RAII, Drop order
│
├── 13_concurrency_threads_and_channels/
│   └── src/bin/
│       ├── threads.rs                 ← thread::spawn, join, scope
│       ├── channels.rs                ← mpsc, clone tx, try_recv
│       └── shared_state.rs            ← Barrier, Condvar, OnceLock, Atomic
│
├── 14_async_await_and_tokio/
│   └── src/bin/
│       ├── async_basics.rs            ← async fn, .await, join!, select!
│       ├── tokio_tasks.rs             ← spawn, try_join_all, Semaphore
│       ├── streams.rs                 ← Stream trait, adapters, channels
│       └── pin_unpin.rs               ← Pin<T>, Unpin, stack/heap pinning
│
├── 15_macros/
│   └── src/bin/
│       ├── macro_rules.rs             ← macro_rules!, repetition, recursion
│       ├── builtin_macros.rs          ← println!, assert!, vec!, env!, cfg!
│       └── derive_macros.rs           ← #[derive], #[cfg], attributes
│
├── 16_unsafe_rust/
│   └── src/bin/
│       ├── unsafe_blocks.rs           ← 5 unsafe superpowers, safe wrappers
│       └── raw_pointers.rs            ← *const/*mut, arithmetic, Box round-trip
│
├── 17_ffi_and_interop/
│   └── src/bin/
│       └── ffi_basics.rs              ← extern "C", CString/CStr, #[no_mangle]
│
├── 18_performance_and_benchmarking/
│   ├── src/bin/performance_tips.rs    ← Allocation, zero-cost iterators, Cow
│   └── benches/
│       ├── string_bench.rs            ← Criterion string benchmarks
│       └── iter_bench.rs              ← Criterion iterator benchmarks
│
├── 19_cli_apps/
│   └── src/bin/
│       ├── clap_basics.rs             ← clap derive, subcommands, ValueEnum
│       └── file_tool.rs               ← wc/grep/head clone, anyhow, stdin
│
├── 20_web_api_project/
│   └── src/bin/
│       └── web_server.rs              ← axum CRUD API, State, Json, Path
│
├── 21_advanced_patterns/
│   └── src/bin/
│       ├── builder_pattern.rs         ← Fluent builder, validated, phantom-typed
│       ├── newtype_pattern.rs         ← Units, validated types, Deref
│       ├── state_machine.rs           ← Enum-based state machine
│       └── type_state.rs              ← Phantom-typed compile-time states
│
└── 22_csharp_to_rust_migration_guides/
    └── src/bin/
        ├── gc_vs_ownership.rs         ← GC pauses vs RAII, Weak<T>
        ├── classes_vs_structs.rs      ← Class → struct+impl, interface → trait
        ├── linq_vs_iterators.rs       ← Every LINQ method mapped to Iterator
        ├── async_model_comparison.rs  ← Task<T> vs Future, cancellation
        └── exceptions_vs_results.rs   ← throw/catch vs Result<T,E>, ?
```

---

## How to Run Examples

### Naming convention

Every example is a standalone binary. The **bin name is the filename without `.rs`**:

```
14_async_await_and_tokio/src/bin/streams.rs  →  cargo run --bin streams
21_advanced_patterns/src/bin/state_machine.rs  →  cargo run --bin state_machine
```

All commands below must be run from the **workspace root** (the folder containing the top-level `Cargo.toml`).

---

### Discover available bins

```sh
# List every binary in the workspace:
cargo run --bin 2>&1 | grep "  " 

# Or just browse the source tree:
find . -path "*/src/bin/*.rs" -printf "%f\n" | sed 's/\.rs$//' | sort
# Windows (PowerShell):
Get-ChildItem -Recurse -Filter "*.rs" -Path "*/src/bin" | Select-Object BaseName | Sort-Object BaseName
```

---

### Run a single example

```sh
cargo run --bin <bin_name>

# Examples:
cargo run --bin hello_world
cargo run --bin ownership
cargo run --bin async_basics
cargo run --bin streams
cargo run --bin state_machine
cargo run --bin performance_tips
cargo run --bin ffi_basics
```

---

### Run in release mode (optimised, required for benchmarks)

```sh
cargo run --release --bin <bin_name>

# Example:
cargo run --release --bin performance_tips
```

---

### Run an example from inside its module directory

```sh
cd 14_async_await_and_tokio
cargo run --bin streams
cargo run --bin async_basics
```

---

### Pass arguments to CLI examples (module 19)

```sh
cargo run --bin clap_basics -- --help
cargo run --bin clap_basics -- greet --name Alice --times 3
cargo run --bin file_tool -- count src/bin/file_tool.rs
cargo run --bin file_tool -- find . --ext rs
```

The `--` separator tells Cargo to stop parsing its own flags and forward the rest to your program.

---

### Run tests

```sh
# Tests for one binary:
cargo test --bin async_basics

# Tests for one module (package):
cargo test -p async_await_and_tokio

# All tests in the workspace:
cargo test --workspace

# A specific test function by name:
cargo test --bin async_basics -- async_test
```

---

### Run benchmarks (module 18)

```sh
# All benchmarks:
cargo bench -p performance_and_benchmarking

# One benchmark suite:
cargo bench -p performance_and_benchmarking --bench string_bench
cargo bench -p performance_and_benchmarking --bench iter_bench
```

Benchmarks always run in release mode automatically.

---

### Quick-reference: all bins by module

| Module | Bin name | Topic |
|--------|----------|-------|
| 01 | `hello_world` · `cargo_basics` · `rustup_info` | Getting started |
| 02 | `variables_mutability` · `scalar_types` · `compound_types` · `strings` · `control_flow` · `constants_statics` | Types & syntax |
| 03 | `ownership` · `moves` · `copy_clone` · `borrowing` | Ownership |
| 04 | `lifetimes` · `lifetime_elision` · `dangling_refs` | Lifetimes |
| 05 | `structs` · `enums` · `pattern_matching` · `option_result` | Structs & enums |
| 06 | `traits` · `generics` · `trait_objects` · `advanced_traits` | Traits & generics |
| 07 | `result_patterns` · `thiserror_example` · `anyhow_example` · `panic_basics` | Error handling |
| 08 | `vec_collections` · `hashmap_hashset` · `iterators` · `custom_iterators` | Collections |
| 09 | `closures` · `functional_patterns` · `function_pointers` | Closures |
| 10 | `modules` · `visibility` · `cargo_features` | Modules & crates |
| 11 | `doc_examples` | Testing & docs |
| 12 | `box_pointer` · `rc_arc` · `cell_refcell` · `mutex_rwlock` · `deref_drop` | Smart pointers |
| 13 | `threads` · `channels` · `shared_state` | Concurrency |
| 14 | `async_basics` · `tokio_tasks` · `streams` · `pin_unpin` | Async / Tokio |
| 15 | `macro_rules` · `derive_macros` · `builtin_macros` | Macros |
| 16 | `unsafe_blocks` · `raw_pointers` | Unsafe Rust |
| 17 | `ffi_basics` | FFI & interop |
| 18 | `performance_tips` | Performance |
| 19 | `clap_basics` · `file_tool` | CLI apps |
| 20 | `web_server` | Web API (axum) |
| 21 | `builder_pattern` · `newtype_pattern` · `state_machine` · `type_state` | Advanced patterns |
| 22 | `gc_vs_ownership` · `classes_vs_structs` · `linq_vs_iterators` · `async_model_comparison` · `exceptions_vs_results` | Migration guides |
| 23 | `encapsulation` · `inheritance_composition` · `polymorphism` · `operator_overloading` | OOP in Rust |
| 24 | `json_basics` · `advanced_serde` | Serde serialization |
| 25 | `tracing_basics` | Logging & tracing |
| 13+ | `send_sync` | Send + Sync (added to module 13) |
| 11+ | `unit_testing` · `proptest_basics` | Testing (added to module 11) |

---

## C# → Rust Concept Map

### Types

| C#                          | Rust                              |
|-----------------------------|-----------------------------------|
| `int`, `long`, `uint`       | `i32`, `i64`, `u32`, `u64`       |
| `double`, `float`           | `f64`, `f32`                      |
| `bool`                      | `bool`                            |
| `char` (UTF-16)             | `char` (Unicode scalar, 4 bytes)  |
| `string` (heap, immutable)  | `String` (owned heap)             |
| `ReadOnlySpan<char>`        | `&str` (borrowed slice)           |
| `T[]`                       | `[T; N]` (fixed) / `Vec<T>` (dynamic) |
| `List<T>`                   | `Vec<T>`                          |
| `Dictionary<K,V>`           | `HashMap<K,V>`                    |
| `HashSet<T>`                | `HashSet<T>`                      |
| `SortedDictionary<K,V>`     | `BTreeMap<K,V>`                   |
| `(T1, T2)`                  | `(T1, T2)` tuple                  |
| `T?` nullable               | `Option<T>`                       |
| `IEnumerable<T>`            | `impl Iterator<Item=T>`           |
| `IAsyncEnumerable<T>`       | `impl Stream<Item=T>`             |

### Memory

| C#                          | Rust                              |
|-----------------------------|-----------------------------------|
| GC (non-deterministic)      | Ownership (deterministic RAII)    |
| `IDisposable` / `using`     | `Drop` trait (automatic)          |
| Reference type (heap)       | `Box<T>` / `Vec<T>` / `String`   |
| Value type (stack)          | Any `T` without `Box`            |
| Shared mutable              | `Arc<Mutex<T>>`                   |
| Shared single-thread        | `Rc<RefCell<T>>`                  |
| `WeakReference<T>`          | `Weak<T>`                         |
| `GC.KeepAlive(obj)`         | `Pin<T>`                          |
| `Span<T>`                   | `&mut [T]`                        |
| `ReadOnlySpan<T>`           | `&[T]`                            |

### Object-Oriented

| C#                          | Rust                              |
|-----------------------------|-----------------------------------|
| `class`                     | `struct` + `impl`                 |
| `interface`                 | `trait`                           |
| Abstract method             | Trait method (no default)         |
| Default interface method    | Trait default method              |
| Inheritance                 | Composition + trait impls         |
| `virtual` / `override`      | `dyn Trait` (vtable dispatch)     |
| Generic constraint `where T : IFoo` | `where T: Foo`           |
| `static` method             | Associated function (no `self`)   |
| `this`                      | `self`                            |
| Property `{ get; set; }`    | Methods `fn value(&self)` / `fn set_value(&mut self)` |
| `sealed class`              | All structs are "sealed" by default |
| `record`                    | `#[derive(Clone, PartialEq, Debug)]` struct |

### Error Handling

| C#                          | Rust                              |
|-----------------------------|-----------------------------------|
| `throw new Exception()`     | `return Err(MyError::...)`        |
| `try { } catch { }`         | `match result { Ok(v) => ..., Err(e) => ... }` |
| `Exception.Message`         | `e.to_string()` (via Display)     |
| `InnerException`            | `#[source]` / `#[from]`          |
| `InvalidOperationException` | `panic!("...")` (truly unrecoverable) |
| `NullReferenceException`    | Cannot occur (Option forces check)|
| `ArgumentException`         | Return `Err(InvalidInput)` from Result |
| `AggregateException`        | `Vec<Error>` or `try_join_all`   |

### Concurrency

| C#                          | Rust                              |
|-----------------------------|-----------------------------------|
| `Thread`                    | `std::thread::spawn`              |
| `Task<T>`                   | `tokio::task::JoinHandle<T>`      |
| `Task.WhenAll`              | `tokio::join!`                    |
| `Task.WhenAny`              | `tokio::select!`                  |
| `Task.Run`                  | `tokio::spawn`                    |
| `await task`                | `handle.await.unwrap()`           |
| `CancellationToken`         | `CancellationToken` (tokio) / drop future |
| `Mutex<T>`                  | `std::sync::Mutex<T>`             |
| `SemaphoreSlim`             | `tokio::sync::Semaphore`          |
| `ConcurrentQueue<T>`        | `std::sync::mpsc::channel`        |
| `Channel<T>`                | `tokio::sync::mpsc::channel`      |
| `Interlocked`               | `std::sync::atomic::AtomicXxx`    |
| `Volatile.Read/Write`       | `Ordering::Acquire/Release`       |
| `Lazy<T>`                   | `std::sync::OnceLock<T>`          |
| `IAsyncEnumerable<T>`       | `tokio_stream::Stream`            |
| `Parallel.ForEachAsync`     | `JoinSet` + `spawn` per item      |

### Functional / LINQ

| C# LINQ                     | Rust Iterator                     |
|-----------------------------|-----------------------------------|
| `.Where(pred)`              | `.filter(pred)`                   |
| `.Select(f)`                | `.map(f)`                         |
| `.SelectMany(f)`            | `.flat_map(f)`                    |
| `.FirstOrDefault()`         | `.next()` or `.find()`            |
| `.Any(pred)`                | `.any(pred)`                      |
| `.All(pred)`                | `.all(pred)`                      |
| `.Count()`                  | `.count()`                        |
| `.Sum()` / `.Min()` / `.Max()` | `.sum()` / `.min()` / `.max()` |
| `.OrderBy(key)`             | `.sort_by_key()` (not lazy)       |
| `.GroupBy(key)`             | `fold` into `HashMap`             |
| `.Zip(other)`               | `.zip(other)`                     |
| `.Concat(other)`            | `.chain(other)`                   |
| `.Distinct()`               | `.collect::<HashSet<_>>()`        |
| `.Take(n)` / `.Skip(n)`     | `.take(n)` / `.skip(n)`          |
| `.Aggregate(seed, f)`       | `.fold(seed, f)`                  |
| `.ToList()`                 | `.collect::<Vec<_>>()`            |
| `.ToDictionary(k, v)`       | `.collect::<HashMap<_, _>>()`    |

---

## Module Reference

| # | Module | Key Concepts | C# Analogy |
|---|--------|-------------|------------|
| 01 | Getting Started | Cargo, rustup, hello world | dotnet new, nuget |
| 02 | Basic Types | Primitives, strings, control flow | C# primitives, switch |
| 03 | Ownership & Moves | Move semantics, Copy, Clone | Value types vs reference types |
| 04 | Lifetimes | Borrow checker, 'a annotations | Ref validity (no direct equiv) |
| 05 | Structs & Enums | ADTs, Option, Result | Classes, nullable types |
| 06 | Traits & Generics | Polymorphism, impl Trait, dyn | Interfaces, generics |
| 07 | Error Handling | Result, ?, thiserror, anyhow | Exception hierarchy |
| 08 | Collections & Iterators | Vec, HashMap, iterator adapters | List, Dictionary, LINQ |
| 09 | Closures | Fn/FnMut/FnOnce, capture | Func<>, Action<>, lambda |
| 10 | Modules & Cargo | pub, use, workspace, features | namespaces, NuGet |
| 11 | Testing | #[test], doc tests, proptest | xUnit, NUnit |
| 12 | Smart Pointers | Box, Rc, Arc, Cell, Mutex | GC, IDisposable |
| 13 | Concurrency | Threads, channels, atomics | Thread, Channel<T> |
| 14 | Async/Tokio | async/await, streams, Pin | Task<T>, IAsyncEnumerable |
| 15 | Macros | macro_rules!, derive, attributes | Source generators |
| 16 | Unsafe | Raw pointers, unsafe blocks | C# unsafe pointer code |
| 17 | FFI | extern "C", P/Invoke from C# | [DllImport] / P/Invoke |
| 18 | Performance | Zero-cost, criterion benchmarks | BenchmarkDotNet |
| 19 | CLI Apps | clap derive, subcommands | System.CommandLine |
| 20 | Web API | axum, REST, State, JSON | ASP.NET Core minimal API |
| 21 | Advanced Patterns | Builder, newtype, type-state | Design patterns |
| 22 | Migration Guides | GC, classes, LINQ, async, errors | Everything above, together |
| 23 | OOP in Rust | Encapsulation, inheritance→composition, polymorphism, operators | Module 06 + 22 |
| 24 | Serde | JSON serialization, attributes, enum tagging | Module 20 |
| 25 | Logging | tracing spans, structured fields, #[instrument] | Module 20 |
| 13+ | Send + Sync | Thread-safety type system, Arc/Mutex guide | Module 13 |
| 11+ | Testing | Unit tests, proptest, cargo test flags | Module 11 |

---

## 1-Week Study Plan

### Day 1 — Foundations (Modules 01–03)
- Morning: 01 (setup), 02 (types) — focus on `scalar_types.rs` and `strings.rs`
- Afternoon: 03 (ownership) — read all four files; run each example
- Goal: understand why moves happen and when Copy applies

### Day 2 — Borrowing and Types (Modules 04–05)
- Morning: 04 (lifetimes) — read `lifetimes.rs`, skim `dangling_refs.rs`
- Afternoon: 05 (structs/enums) — focus on `enums.rs` and `option_result.rs`
- Goal: write a `Shape` enum with `area()` method; return `Result` from a function

### Day 3 — Traits and Errors (Modules 06–07)
- Morning: 06 (traits/generics) — `traits.rs`, `generics.rs`, `trait_objects.rs`
- Afternoon: 07 (error handling) — all four files
- Goal: define a trait, implement it for two types, use `?` in a chain

### Day 4 — Collections and Functional (Modules 08–09)
- Morning: 08 — `iterators.rs` (map/filter/fold)
- Afternoon: 09 — closures and functional patterns
- Goal: rewrite a C# LINQ query as a Rust iterator chain

### Day 5 — Concurrency and Async (Modules 13–14)
- Morning: 13 — threads, channels, Mutex
- Afternoon: 14 — async/await, tokio::spawn, join!
- Goal: spawn 5 tasks concurrently, collect results

### Day 6 — Applications (Modules 19–20)
- Morning: 19 — build a CLI app with clap
- Afternoon: 20 — run the axum web server; test with curl
- Goal: add a new endpoint to the web server

### Day 7 — Advanced and Review (Modules 21–22)
- Morning: 21 — builder pattern, type-state
- Afternoon: 22 — read all migration guides; compare with your C# mental model
- Goal: start the capstone project (below)

---

## Glossary

| Term | Definition |
|------|-----------|
| **Owner** | The variable that holds a value and is responsible for freeing it |
| **Move** | Transfer of ownership from one variable to another; original is invalid |
| **Borrow** | Temporary access via `&T` (immutable) or `&mut T` (mutable) |
| **Lifetime** | The scope during which a borrow is valid; tracked by the compiler |
| **Trait** | A set of method signatures (+ defaults); like C# interface |
| **impl Trait** | Static dispatch — compiler generates one copy per concrete type |
| **dyn Trait** | Dynamic dispatch — vtable lookup at runtime; like C# virtual |
| **Monomorphisation** | Each generic instantiation gets its own compiled copy |
| **RAII** | Resource Acquisition Is Initialisation — resource freed on scope exit |
| **Drop** | The trait whose `drop()` is called when a value goes out of scope |
| **Option<T>** | `Some(T)` or `None` — replaces `null` |
| **Result<T,E>** | `Ok(T)` or `Err(E)` — replaces exceptions |
| **`?` operator** | Early-return on `Err`/`None`; equivalent to `.unwrap()` + `return` |
| **Box<T>** | Heap-allocated single owner; like `new T()` in C# value types |
| **Rc<T>** | Reference-counted single-thread shared ownership |
| **Arc<T>** | Atomically reference-counted thread-safe shared ownership |
| **Cell<T>** | Interior mutability for Copy types |
| **RefCell<T>** | Interior mutability with runtime borrow checks |
| **Mutex<T>** | Thread-safe interior mutability via OS lock |
| **Send** | Type is safe to transfer to another thread |
| **Sync** | Type is safe to share between threads (`&T` is Send) |
| **Future** | A lazily-computed async value; C# `Task<T>` but not started yet |
| **executor** | The runtime that polls futures (Tokio for async) |
| **macro_rules!** | Hygienic syntactic macro system (C# has no direct equiv) |
| **proc macro** | Compile-time code generation via Rust code (C# source generators) |
| **workspace** | Multiple crates sharing a root `Cargo.toml`; like a .NET solution |
| **crate** | A compilation unit; binary crate = exe, library crate = .dll |
| **edition** | A backwards-compatible language version (2015/2018/2021/2024) |
| **unsafe** | A block/fn that unlocks 5 specific low-level operations |
| **PhantomData<T>** | Zero-sized type marker for phantom type parameters |
| **newtype** | A one-field tuple struct wrapping another type |
| **RPIT** | Return-Position impl Trait: `fn foo() -> impl Iterator<...>` |

---

## Capstone Project

Build a **task manager CLI** that demonstrates all major concepts:

```
tasks add "Buy groceries" --due 2026-06-01 --priority high
tasks list --filter pending --sort due
tasks complete 3
tasks export --format json > tasks.json
```

### Requirements

1. **Data model** — `Task` struct with id, title, due date, priority, status (use enum)
2. **Storage** — read/write `tasks.json` via `serde_json`
3. **CLI** — `clap` derive with subcommands (add/list/complete/export)
4. **Error handling** — `thiserror` for domain errors, `anyhow` in main
5. **Filtering/sorting** — iterator chains (map/filter/sort_by)
6. **Tests** — unit tests for domain logic, integration test for CLI output

### Stretch Goals

- `tasks serve` — start an axum web API (module 20 pattern)
- Async file I/O via Tokio
- Property-based tests with `proptest`
- Custom `Display` for coloured terminal output

---

## Recommended Resources

| Resource | URL |
|----------|-----|
| The Rust Book (official) | https://doc.rust-lang.org/book/ |
| Rust by Example | https://doc.rust-lang.org/rust-by-example/ |
| Rustlings exercises | https://github.com/rust-lang/rustlings |
| Comprehensive Rust (Google) | https://google.github.io/comprehensive-rust/ |
| Tokio tutorial | https://tokio.rs/tokio/tutorial |
| Jon Gjengset — Rust for Rustaceans | https://nostarch.com/rust-rustaceans |
| Exercism Rust track | https://exercism.org/tracks/rust |
| This Week in Rust | https://this-week-in-rust.org |
| crates.io | https://crates.io |
| docs.rs (crate documentation) | https://docs.rs |

---

*Generated for Rust 1.95.0 (Edition 2024). All examples compile with `cargo check --workspace`.*
