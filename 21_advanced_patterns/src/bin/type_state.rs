// ============================================================
// CONCEPT: Type-State Pattern
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// The type-state pattern encodes a state machine in the TYPE SYSTEM
// so invalid transitions are compile errors, not runtime panics.
//
// C# equivalent would require separate interfaces per state and
// careful design. Rust's zero-cost generics make it natural.
//
// Example: a database connection that must be:
//   1. Created (Disconnected)
//   2. Connected before querying
//   3. In a transaction before committing
//
// RUN: cargo run --bin type_state
// ============================================================

use std::marker::PhantomData;

fn main() {
    println!("=== Type-State Pattern ===\n");

    connection_demo();
    file_builder_demo();
    lock_demo();
}

// ---- 1. Database connection type-state ----------------------------

// State markers — zero-sized types, erased at compile time:
struct Disconnected;
struct Connected;
struct InTransaction;

// Generic over state S — different methods available per state:
struct DbConnection<S> {
    dsn: String,
    _state: PhantomData<S>,
}

// Methods available only when Disconnected:
impl DbConnection<Disconnected> {
    fn new(dsn: impl Into<String>) -> Self {
        DbConnection { dsn: dsn.into(), _state: PhantomData }
    }

    fn connect(self) -> Result<DbConnection<Connected>, String> {
        println!("  [DB] connecting to {}", self.dsn);
        // In real code: actually open a connection
        Ok(DbConnection { dsn: self.dsn, _state: PhantomData })
    }
}

// Methods available only when Connected:
impl DbConnection<Connected> {
    fn query(&self, sql: &str) -> Vec<String> {
        println!("  [DB] query: {sql}");
        vec!["row1".to_string(), "row2".to_string()]
    }

    fn begin_transaction(self) -> DbConnection<InTransaction> {
        println!("  [DB] BEGIN");
        DbConnection { dsn: self.dsn, _state: PhantomData }
    }

    fn disconnect(self) -> DbConnection<Disconnected> {
        println!("  [DB] disconnecting");
        DbConnection { dsn: self.dsn, _state: PhantomData }
    }
}

// Methods available only when InTransaction:
impl DbConnection<InTransaction> {
    fn execute(&self, sql: &str) {
        println!("  [DB] execute: {sql}");
    }

    fn commit(self) -> DbConnection<Connected> {
        println!("  [DB] COMMIT");
        DbConnection { dsn: self.dsn, _state: PhantomData }
    }

    fn rollback(self) -> DbConnection<Connected> {
        println!("  [DB] ROLLBACK");
        DbConnection { dsn: self.dsn, _state: PhantomData }
    }
}

fn connection_demo() {
    println!("--- Database Connection Type-State ---");

    let conn = DbConnection::<Disconnected>::new("postgres://localhost/mydb");
    let conn = conn.connect().expect("connect failed");

    let rows = conn.query("SELECT * FROM users");
    println!("  rows: {rows:?}");

    let tx = conn.begin_transaction();
    tx.execute("INSERT INTO users VALUES (4, 'Dave')");
    let conn = tx.commit();

    // Start another transaction but roll it back:
    let tx2 = conn.begin_transaction();
    tx2.execute("DELETE FROM users WHERE id = 1");
    let conn = tx2.rollback();

    conn.disconnect();

    // These would be COMPILE ERRORS:
    // conn.begin_transaction().query("SELECT 1")  // query not on InTransaction
    // DbConnection::<Disconnected>::new("x").query("x")  // not connected yet
    println!("  (calling .query() on a disconnected conn is a compile error)");
}

// ---- 2. File builder type-state -----------------------------------

struct NeedsPath;
struct HasPath;
struct HasContent;

struct FileWriter<S> {
    path: String,
    content: String,
    append: bool,
    _state: PhantomData<S>,
}

impl FileWriter<NeedsPath> {
    fn new() -> Self {
        FileWriter { path: String::new(), content: String::new(), append: false, _state: PhantomData }
    }

    fn path(self, p: impl Into<String>) -> FileWriter<HasPath> {
        FileWriter { path: p.into(), content: self.content, append: self.append, _state: PhantomData }
    }
}

impl FileWriter<HasPath> {
    fn content(self, c: impl Into<String>) -> FileWriter<HasContent> {
        FileWriter { path: self.path, content: c.into(), append: self.append, _state: PhantomData }
    }

    fn append(mut self, a: bool) -> Self {
        self.append = a;
        self
    }
}

impl FileWriter<HasContent> {
    fn write(self) {
        let mode = if self.append { "appending" } else { "writing" };
        println!("  [File] {} {} bytes to '{}'", mode, self.content.len(), self.path);
    }
}

fn file_builder_demo() {
    println!("\n--- File Writer Type-State Builder ---");

    FileWriter::new()
        .path("/tmp/demo.txt")
        .content("Hello, Type-State World!")
        .write();

    // Cannot .write() without .content() — compile error:
    // FileWriter::new().path("/tmp/x").write();
    println!("  (.write() without .content() is a compile error)");
}

// ---- 3. Read/Write lock type-state --------------------------------

struct Unlocked;
struct ReadLocked;
struct WriteLocked;

struct GuardedData<S> {
    value: i32,
    _state: PhantomData<S>,
}

impl GuardedData<Unlocked> {
    fn new(v: i32) -> Self {
        GuardedData { value: v, _state: PhantomData }
    }

    fn read_lock(self) -> GuardedData<ReadLocked> {
        println!("  acquiring read lock");
        GuardedData { value: self.value, _state: PhantomData }
    }

    fn write_lock(self) -> GuardedData<WriteLocked> {
        println!("  acquiring write lock");
        GuardedData { value: self.value, _state: PhantomData }
    }
}

impl GuardedData<ReadLocked> {
    fn read(&self) -> i32 {
        println!("  reading: {}", self.value);
        self.value
    }

    fn unlock(self) -> GuardedData<Unlocked> {
        println!("  releasing read lock");
        GuardedData { value: self.value, _state: PhantomData }
    }
}

impl GuardedData<WriteLocked> {
    fn read(&self) -> i32 { self.value }

    fn write(&mut self, v: i32) {
        println!("  writing: {} → {}", self.value, v);
        self.value = v;
    }

    fn unlock(self) -> GuardedData<Unlocked> {
        println!("  releasing write lock");
        GuardedData { value: self.value, _state: PhantomData }
    }
}

fn lock_demo() {
    println!("\n--- Read/Write Lock Type-State ---");

    let data = GuardedData::new(42);

    let reader = data.read_lock();
    let v = reader.read();
    let data = reader.unlock();
    println!("  read value: {v}");

    let mut writer = data.write_lock();
    writer.write(100);
    let data = writer.unlock();

    let reader = data.read_lock();
    reader.read();
    let _data = reader.unlock();

    // This is a compile error — can't write through a ReadLocked:
    // reader.write(99);
    println!("  (.write() on ReadLocked is a compile error)");
}

// ---- Key insight --------------------------------------------------
//
// The type-state pattern moves state machine correctness from
// runtime (match/if + error handling) to compile time (type errors).
//
// Cost: zero — PhantomData<S> is zero-sized; all transitions are
//       just moves, which are no-ops in the generated code.
//
// Trade-off: adds complexity and generics noise. Use when invalid
//            transitions would be serious errors worth preventing
//            at compile time. For simpler machines, enum-based
//            state machines (state_machine.rs) are more readable.
