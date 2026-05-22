# Pyoway Axum Backend Conventions

## Server Architecture

The landing server is an Axum 0.8 HTTP server that:
1. Serves the pre-built WASM frontend bundle from `landing-frontend/dist/`
2. Provides a `/health` endpoint for monitoring
3. Implements SPA fallback — unknown routes serve `index.html`

## Module Structure

```
landing-server/src/
├── main.rs      # Server entrypoint, routing, middleware stack, handler functions
├── config.rs    # AppConfig struct loaded from environment variables
└── error.rs     # AppError enum with IntoResponse implementation
```

## Request Flow

```
Browser → TraceLayer (logging) → CompressionLayer (gzip/br) → CorsLayer
       → Security headers (X-Frame-Options, X-Content-Type-Options)
       → Route matching
           ├── GET /health → JSON {"status": "ok"}
           └── fallback → handle_frontend()
                            ├── Static file from dist/
                            └── SPA fallback to index.html
```

## Configuration Pattern

Configuration is loaded from environment variables (with `.env` file support via `dotenvy`):

```rust
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,        // default: "127.0.0.1"
    pub port: u16,           // default: 8080
    pub cors_origin: String, // default: "http://localhost:8080"
}
```

### Config loading pattern

```rust
impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_raw(
            std::env::var("HOST").ok(),
            std::env::var("PORT").ok(),
            std::env::var("CORS_ORIGIN").ok(),
        )
    }

    // Test helper — avoids env manipulation
    fn from_raw(host: Option<String>, port: Option<String>, cors_origin: Option<String>)
        -> Result<Self, ConfigError>
    { ... }
}
```

## Error Handling Pattern

Error types use `thiserror` + `IntoResponse`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Internal server error: {0}")]
    Internal(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            Self::Internal(ref e) => {
                tracing::error!("Internal error: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

## Middleware Stack

Applied in `build_router()` in `main.rs`:

```rust
Router::new()
    .route("/health", get(health_check))
    .fallback(get(handle_frontend))
    .layer(TraceLayer::new_for_http())
    .layer(CompressionLayer::new())
    .layer(cors)
    .layer(SetResponseHeaderLayer::if_not_present(
        X_FRAME_OPTIONS, HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"),
    ))
```

## Static File Serving

The `handle_frontend` function:
1. Extracts the URI path
2. Sanitizes it (removes `..`, `.`, empty segments)
3. Verifies the resolved path is within `dist/`
4. Attempts to serve the exact file (with correct MIME type)
5. Falls back to `index.html` for SPA client-side routing

### MIME Type Mapping

```rust
fn mime_type_from_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("html") => "text/html",
        // ... etc
        _ => "application/octet-stream",
    }
}
```

## Testing Patterns

- Tests use `axum::body::Body` and `tower::ServiceExt::oneshot`
- `AppConfig` is constructed directly (not from env) in tests
- Test module is `#[cfg(test)] #[allow(clippy::unwrap_used, clippy::expect_used)]`
- Integration tests are in the same file as the handler

```rust
#[tokio::test]
async fn test_health_check() {
    let config = AppConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        cors_origin: "http://localhost:8080".to_string(),
    };
    let app = build_router(&config);

    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
```

## Key Do's and Don'ts

### DO
- Use `axum::serve` with `TcpListener` for binding
- Validate and sanitize all user-provided paths
- Use `tracing` for all logging
- Document all public items (`#![deny(missing_docs)]`)
- Use `#[allow(clippy::expect_used)]` on `main()` where panicking on startup failure is acceptable

### DON'T
- Don't use `actix-web` — the server uses Axum (despite older specs mentioning Actix-web)
- Don't expose `unsafe` code
- Don't use `unwrap()` in production paths without a comment explaining why it's safe
- Don't forget the workspace lint reference in `Cargo.toml`: `[lints] workspace = true`
