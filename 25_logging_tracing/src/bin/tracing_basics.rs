// ============================================================
// CONCEPT: Structured Logging with tracing
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# logging:
//   ILogger<MyService> _logger;
//   _logger.LogInformation("User {UserId} logged in", userId);
//   _logger.LogError(ex, "Failed to process order {OrderId}", orderId);
//
//   // Serilog structured logging:
//   Log.Information("Processing {Count} items for {User}", count, user);
//
// Rust tracing:
//   tracing::info!(user_id = userId, "User logged in");
//   tracing::error!(order_id = orderId, error = %err, "Failed to process order");
//
// `tracing` is the standard structured logging crate in Rust — it is to
// Rust what Serilog is to C#. It supports:
//   - Log levels: trace, debug, info, warn, error
//   - Structured key=value fields (not just interpolated strings)
//   - Spans: named scopes that track duration and context (like Activity/Span)
//   - Multiple backends (stdout, files, OpenTelemetry, Jaeger, etc.)
//
// The separation of concerns:
//   tracing crate         → emit events/spans (like ILogger interface)
//   tracing-subscriber    → collect and format them (like Serilog sink)
//
// RUN: cargo run --bin tracing_basics
// ============================================================

use tracing::{debug, error, info, warn, Level};
use tracing::instrument;
use tracing_subscriber::EnvFilter;

fn main() {
    // ── Subscriber setup — do this once at program startup ─────────────────
    // C# equivalent: Log.Logger = new LoggerConfiguration().WriteTo.Console().CreateLogger();
    // or: services.AddLogging(b => b.AddConsole());
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug")),  // default to DEBUG level
        )
        .with_target(false)      // hide the module path in output
        .with_level(true)        // show the level
        .init();

    println!("=== Structured Logging with tracing ===\n");

    demo_log_levels();
    demo_structured_fields();
    demo_spans();
    demo_instrument_attribute();
    demo_error_logging();
    demo_configuration_guide();
}

// ─── 1. LOG LEVELS ──────────────────────────────────────────────────────────
//
// Five levels, lowest to highest severity:
//   TRACE → DEBUG → INFO → WARN → ERROR
//
// C#:                    Rust tracing:
//   LogTrace()           tracing::trace!()
//   LogDebug()           tracing::debug!()
//   LogInformation()     tracing::info!()
//   LogWarning()         tracing::warn!()
//   LogError()           tracing::error!()
//   LogCritical()        tracing::error!() with a field like critical=true

fn demo_log_levels() {
    println!("--- 1. Log Levels ---\n");

    tracing::trace!("Very verbose — only shown at TRACE level");
    debug!("Debugging info — shown at DEBUG and below");
    info!("Server started on port 8080");
    warn!("Memory usage is at 85%");
    error!("Failed to connect to database");

    println!(r#"
  C#                                    Rust tracing
  ─────────────────────────────────────────────────────────────────
  _logger.LogTrace("msg")               tracing::trace!("msg")
  _logger.LogDebug("msg")               tracing::debug!("msg")
  _logger.LogInformation("msg")         tracing::info!("msg")
  _logger.LogWarning("msg")             tracing::warn!("msg")
  _logger.LogError("msg")               tracing::error!("msg")
  _logger.LogCritical("msg")            tracing::error!(critical = true, "msg")

  Filter at runtime:
    C#:   "Logging": {{ "LogLevel": {{ "Default": "Warning" }} }}
    Rust: RUST_LOG=warn cargo run
          EnvFilter::new("myapp=debug,warn")  // myapp module at DEBUG, rest at WARN
"#);
}

// ─── 2. STRUCTURED FIELDS ───────────────────────────────────────────────────
//
// The power of tracing: key=value pairs are first-class, not just
// interpolated strings. Backends (JSON, OpenTelemetry) can query by field.
//
// C#:
//   _logger.LogInformation("User {UserId} logged in from {IpAddress}", userId, ip);
//   // Serilog: Log.Information("User {UserId} logged in", userId);
//
// Rust: fields are named explicitly — no positional arguments:
//   info!(user_id = %userId, ip = %ip, "User logged in");

fn demo_structured_fields() {
    println!("--- 2. Structured Fields ---\n");

    let user_id   = 42_u64;
    let username  = "alice";
    let ip        = "192.168.1.1";
    let duration_ms = 127_u64;

    // Basic structured fields — key = value:
    info!(user_id, username, "User logged in");

    // % means Display formatting (toString):
    info!(user_id = %user_id, ip = %ip, "Login from IP");

    // ? means Debug formatting ({:?}):
    let tags = vec!["admin", "beta"];
    debug!(user_id, tags = ?tags, "User tags loaded");

    // Mix fields and a message:
    info!(
        user_id  = user_id,
        username = username,
        duration = duration_ms,
        "Request completed"
    );

    // Record a computed value:
    let item_count = 150_usize;
    warn!(
        threshold = 100,
        actual    = item_count,
        excess    = item_count - 100,
        "Queue depth exceeded threshold"
    );

    println!(r#"
  Rust field formatting sigils:
    field = value      → uses the field's own format (for numbers, bools, etc.)
    field = %value     → Display format (like {{:}} in C# format strings)
    field = ?value     → Debug format  (like {{:?}})

  C#                                     Rust
  ──────────────────────────────────────────────────────────────────────
  _logger.Log(Info, "{{UserId}} logged in", userId)  info!(user_id, "logged in")
  Log.Information("{{Count}} items", count)           info!(count, "items processed")
  Properties in structured log            Named fields — queryable in log backends
"#);
}

// ─── 3. SPANS ───────────────────────────────────────────────────────────────
//
// Spans represent a period of time with a name and optional fields.
// They are equivalent to C# Activity (System.Diagnostics) or Serilog's
// operation timing. Nested spans form a trace tree.
//
// C#:
//   using var activity = ActivitySource.StartActivity("ProcessOrder");
//   activity?.SetTag("order.id", orderId);
//
// Rust:
//   let span = tracing::info_span!("process_order", order_id = orderId);
//   let _guard = span.enter();  // span is active while _guard is alive

fn demo_spans() {
    println!("--- 3. Spans (Structured Context) ---\n");

    // Create and enter a span — all events inside inherit the span's context:
    let order_id = 1001_u64;
    let span = tracing::info_span!("process_order", order_id);
    let _guard = span.enter();   // activates the span for this scope

    info!("Validating order");         // these events belong to process_order span
    debug!(items = 3, "Loading items");
    info!("Payment processing started");

    {
        // Nested span — child of process_order:
        let payment_span = tracing::debug_span!("charge_card", last_four = 4242_u32);
        let _pg = payment_span.enter();

        debug!("Contacting payment gateway");
        info!(gateway = "stripe", "Charge successful");
    } // payment_span ends here

    info!("Order complete");
    // _guard dropped here → process_order span ends

    println!(r#"
  C#                                    Rust tracing
  ─────────────────────────────────────────────────────────────────
  ActivitySource.StartActivity("name")  tracing::info_span!("name", fields...)
  activity.SetTag("key", value)         field = value inside the span! macro
  using var a = source.Start(..)        let _g = span.enter(); // dropped at scope end
  Activity.Current                      tracing::Span::current()
  Nested activities                     Nested spans (automatic parent-child)
  OpenTelemetry export                  tracing-opentelemetry crate
"#);
}

// ─── 4. #[instrument] ATTRIBUTE ─────────────────────────────────────────────
//
// #[instrument] automatically creates a span for a function, capturing
// the function name and any arguments you choose as fields.
// C# equivalent: [LoggerMessage] attribute + source generators, or AOP interceptors.

#[instrument(fields(result))]
fn calculate_total(items: &[u64], discount_pct: u8) -> u64 {
    debug!("Calculating total for {} items", items.len());

    let subtotal: u64 = items.iter().sum();
    debug!(subtotal, "Subtotal computed");

    let discount = subtotal * discount_pct as u64 / 100;
    let total = subtotal - discount;

    // Record the result into the span's fields:
    tracing::Span::current().record("result", total);

    info!(subtotal, discount, total, "Calculation complete");
    total
}

#[instrument(level = "debug", skip(password))]  // skip sensitive args
fn authenticate(username: &str, password: &str) -> bool {
    debug!("Authenticating user");
    // In real code: check password hash
    let ok = !password.is_empty();
    info!(username, success = ok, "Authentication result");
    ok
}

fn demo_instrument_attribute() {
    println!("--- 4. #[instrument] Automatic Spans ---\n");

    let total = calculate_total(&[100, 250, 75, 50], 10);
    println!("Total after 10% discount: {total}\n");

    let ok = authenticate("alice", "secret");
    println!("Auth result: {ok}\n");

    println!(r#"
  #[instrument] on a function:
    - Creates a span named after the function automatically
    - Captures function arguments as span fields
    - skip(field) excludes sensitive or large arguments
    - level = "debug" overrides default INFO span level
    - ret captures the return value as a field

  C# equivalent: none built-in — requires ILogger injection + manual spans
  Closest: [LoggerMessage] for high-perf logging, or Castle.DynamicProxy AOP
"#);
}

// ─── 5. ERROR LOGGING ───────────────────────────────────────────────────────
//
// C#:
//   _logger.LogError(exception, "Failed to process {OrderId}", orderId);
//
// Rust: pass the error as a field. Use %err for Display, ?err for Debug.

fn demo_error_logging() {
    println!("--- 5. Logging Errors ---\n");

    fn process(input: &str) -> Result<u64, std::num::ParseIntError> {
        input.trim().parse::<u64>()
    }

    let inputs = ["42", "not_a_number", "100"];

    for input in &inputs {
        match process(input) {
            Ok(n)  => info!(value = n, "Parsed successfully"),
            Err(e) => error!(
                input = input,
                error = %e,      // Display format of the error
                "Parse failed"
            ),
        }
    }

    println!(r#"
  C#                                    Rust
  ─────────────────────────────────────────────────────────────────
  _logger.LogError(ex, "Failed")        error!(error = %err, "Failed")
  ex.Message in structured log          error = %err   (Display)
  ex.ToString() / full stack trace      error = ?err   (Debug, shows more detail)
  Exception.Data["key"] = value         Add extra fields: error = %e, context = "..."
"#);
}

// ─── 6. CONFIGURATION GUIDE ─────────────────────────────────────────────────

fn demo_configuration_guide() {
    println!("--- 6. Configuration Reference ---\n");

    println!(r#"
  ── Subscriber Setup Options ────────────────────────────────────────

  // Minimal (default format, respects RUST_LOG env var):
  tracing_subscriber::fmt::init();

  // Full control:
  tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .with_target(true)        // show module path
      .with_thread_ids(true)    // show thread IDs
      .with_line_number(true)   // show source line
      .with_file(true)          // show source file
      .json()                   // output as JSON (for log aggregators)
      .init();

  ── RUST_LOG Filter Syntax ──────────────────────────────────────────

  RUST_LOG=debug              → all modules at DEBUG
  RUST_LOG=warn               → all modules at WARN
  RUST_LOG=myapp=debug        → myapp crate at DEBUG, others at default
  RUST_LOG=myapp=debug,warn   → myapp at DEBUG, everything else at WARN
  RUST_LOG=myapp::db=trace    → only the db submodule at TRACE

  ── C# appsettings.json equivalent ─────────────────────────────────

  C#:
    "Logging": {{
      "LogLevel": {{
        "Default": "Warning",
        "MyApp": "Debug",
        "Microsoft": "Error"
      }}
    }}

  Rust (RUST_LOG env var):
    RUST_LOG=warning,my_app=debug,microsoft=error

  ── Common Patterns ─────────────────────────────────────────────────

  // Scoped logger (like ILogger<MyService> in C# DI):
  fn my_service() {{
      let _span = tracing::info_span!("MyService").entered();
      // All events here tagged with MyService span
  }}

  // Event with many fields:
  tracing::info!(
      user_id    = 42,
      request_id = %uuid,
      path       = "/api/orders",
      method     = "POST",
      status     = 200,
      latency_ms = 45,
      "HTTP request"
  );
"#);
}

// ─── TESTS ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    // tracing events can be tested with the tracing-test crate.
    // For now: verify instrumented functions return correct values
    // (the spans themselves run silently in tests).

    use super::*;

    #[test]
    fn calculate_total_correct() {
        // Set up a no-op subscriber for tests so tracing doesn't panic:
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::fmt()
                .with_max_level(Level::ERROR)  // silence output in tests
                .finish(),
        );
        let total = calculate_total(&[100, 200], 10);
        assert_eq!(total, 270);  // 300 - 10% = 270
    }

    #[test]
    fn authenticate_empty_password_fails() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::fmt().with_max_level(Level::ERROR).finish(),
        );
        assert!(!authenticate("alice", ""));
    }
}
