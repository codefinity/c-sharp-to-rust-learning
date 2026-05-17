// ============================================================
// CONCEPT: Web API with axum
// ============================================================
//
// WHY THIS MATTERS FOR C# DEVELOPERS
// ------------------------------------
// C# ASP.NET Core: Controllers, [HttpGet], [Route], IActionResult,
//   dependency injection via IServiceCollection.
//
// Rust axum: Router, handler fns, extractors (Path, Query, Json, State),
//   tower middleware, no DI framework needed (pass state via Arc<AppState>).
//
// axum handlers are async functions — their arguments are "extractors"
// that automatically parse the request.
//
// RUN: cargo run --bin web_server
// Then in another terminal:
//   curl http://localhost:3000/
//   curl http://localhost:3000/users
//   curl http://localhost:3000/users/1
//   curl -X POST http://localhost:3000/users -H 'Content-Type: application/json' -d '{"name":"Alice","email":"alice@example.com"}'
//   curl -X DELETE http://localhost:3000/users/1
//   curl "http://localhost:3000/search?q=ali&limit=5"
// ============================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_subscriber::EnvFilter;

// ---- Domain types --------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    limit: Option<usize>,
}

// ---- Application state (C# analogy: singleton service) ------------

// Arc<Mutex<T>> is the idiomatic shared-mutable state in axum.
// C# analogy: IMemoryCache or a service registered as AddSingleton.
type AppState = Arc<Mutex<AppDb>>;

#[derive(Default)]
struct AppDb {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl AppDb {
    fn insert(&mut self, name: String, email: String) -> User {
        self.next_id += 1;
        let user = User { id: self.next_id, name, email };
        self.users.insert(user.id, user.clone());
        user
    }
}

// ---- Error type ----------------------------------------------------

// axum requires errors to implement IntoResponse.
// C# analogy: IActionResult with ProblemDetails.
#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::NotFound   => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal   => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::json!({ "error": self.to_string() });
        (status, Json(body)).into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

// ---- Handlers (C# analogy: Controller action methods) -------------

// GET /
async fn root() -> &'static str {
    "Welcome to the axum demo API. Try GET /users"
}

// GET /users
// State extractor injects our shared AppState.
async fn list_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let db = state.lock().unwrap();
    let mut users: Vec<User> = db.users.values().cloned().collect();
    users.sort_by_key(|u| u.id);
    Json(users)
}

// GET /users/:id
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> ApiResult<Json<User>> {
    let db = state.lock().unwrap();
    db.users.get(&id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound)
}

// POST /users  { "name": "...", "email": "..." }
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    if payload.name.is_empty() {
        // Can't use ? here easily with mixed return types, so return directly:
        return (StatusCode::BAD_REQUEST, Json(User { id: 0, name: String::new(), email: String::new() }));
    }
    let mut db = state.lock().unwrap();
    let user = db.insert(payload.name, payload.email);
    info!("created user {}", user.id);
    (StatusCode::CREATED, Json(user))
}

// DELETE /users/:id
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> ApiResult<StatusCode> {
    let mut db = state.lock().unwrap();
    if db.users.remove(&id).is_none() {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

// GET /search?q=alice&limit=10
async fn search_users(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<Vec<User>> {
    let db = state.lock().unwrap();
    let q = params.q.unwrap_or_default().to_lowercase();
    let limit = params.limit.unwrap_or(20);

    let results: Vec<User> = db.users.values()
        .filter(|u| {
            q.is_empty()
                || u.name.to_lowercase().contains(&q)
                || u.email.to_lowercase().contains(&q)
        })
        .cloned()
        .take(limit)
        .collect();

    Json(results)
}

// ---- Router and main -----------------------------------------------

fn build_router(state: AppState) -> Router {
    // C# analogy: app.MapGet(...) / app.MapPost(...) route registration
    Router::new()
        .route("/", get(root))
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).delete(delete_user))
        .route("/search", get(search_users))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    // Initialise tracing (C# analogy: ILogger / Serilog)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    // Pre-populate some users so the demo is interesting:
    let state: AppState = Arc::new(Mutex::new(AppDb::default()));
    {
        let mut db = state.lock().unwrap();
        db.insert("Alice Smith".to_string(), "alice@example.com".to_string());
        db.insert("Bob Jones".to_string(), "bob@example.com".to_string());
        db.insert("Carol White".to_string(), "carol@example.com".to_string());
    }

    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");
    println!("Try:");
    println!("  curl http://localhost:3000/users");
    println!("  curl http://localhost:3000/users/1");
    println!("  curl -X POST http://localhost:3000/users \\");
    println!("       -H 'Content-Type: application/json' \\");
    println!("       -d '{{\"name\":\"Dave\",\"email\":\"dave@example.com\"}}'");
    println!("  curl -X DELETE http://localhost:3000/users/2");
    println!("  curl 'http://localhost:3000/search?q=alice'");

    axum::serve(listener, app).await.unwrap();
}

// ---- C# ASP.NET Core → axum mapping --------------------------------
//
// C# ASP.NET Core            | axum
// ---------------------------|----------------------------------
// [ApiController]            | Router (no attribute needed)
// [HttpGet("/users")]        | .route("/users", get(handler))
// [HttpPost]                 | .route("/users", post(handler))
// [FromRoute] int id         | Path(id): Path<u64>
// [FromQuery] string q       | Query(p): Query<SearchParams>
// [FromBody] CreateUser body | Json(body): Json<CreateUser>
// IActionResult Ok(x)        | (StatusCode::OK, Json(x))
// IActionResult NotFound()   | StatusCode::NOT_FOUND
// AddSingleton<T>()          | .with_state(Arc::new(...))
// ILogger<T>                 | tracing::info!/debug!/warn!
// Middleware (Use...)        | tower Layer (e.g. TraceLayer)
