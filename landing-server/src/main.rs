//! Pyoway landing page server.
//!
//! An Axum-based HTTP server that serves the Leptos WASM frontend
//! and provides API routes.

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

use std::net::SocketAddr;

use std::path::{Path, PathBuf};

use axum::{
    Router,
    http::{
        StatusCode, Uri,
        header::{self, HeaderValue},
    },
    response::IntoResponse,
    routing::get,
};
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

mod config;
mod error;

use config::AppConfig;

/// Health check endpoint.
async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(serde_json::json!({"status": "ok"})),
    )
}

/// Determine the MIME type from a file extension.
fn mime_type_from_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("js") => "application/javascript",
        Some("wasm") => "application/wasm",
        Some("css") => "text/css",
        Some("html") => "text/html",
        Some("png") => "image/png",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

/// Single handler for all frontend routes.
///
/// Serves static files directly and falls back to `index.html`
/// for client-side routing (SPA fallback). Paths are sanitized
/// to prevent directory traversal attacks.
async fn handle_frontend(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    // Sanitize path: remove any path traversal segments
    let safe_path: PathBuf = path
        .split(['/', '\\'])
        .filter(|&segment| !segment.is_empty() && segment != ".." && segment != ".")
        .collect();

    let full_path = PathBuf::from("landing-frontend/dist").join(&safe_path);

    // Verify the resolved path is within the dist directory
    let dist_root = Path::new("landing-frontend/dist");
    if !full_path.starts_with(dist_root) {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    // Try to serve the exact file requested
    if let Ok(content) = tokio::fs::read(&full_path).await {
        let content_type = mime_type_from_path(&full_path);
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, content_type)],
            content,
        )
            .into_response();
    }

    tracing::debug!("SPA fallback for: {uri}");
    // SPA fallback: serve index.html for client-side routing
    tokio::fs::read_to_string("landing-frontend/dist/index.html")
        .await
        .map_or_else(
            |_| {
                (
                    StatusCode::NOT_FOUND,
                    [(header::CONTENT_TYPE, "text/plain")],
                    b"Not Found"[..].to_vec(),
                )
                    .into_response()
            },
            |html| {
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                    html.into_bytes(),
                )
                    .into_response()
            },
        )
}

/// Build the application router with all middleware.
#[allow(clippy::expect_used)]
fn build_router(config: &AppConfig) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origin
                .parse::<HeaderValue>()
                .expect("Invalid CORS origin"),
        )
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    Router::new()
        .route("/health", get(health_check))
        .fallback(get(handle_frontend))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
}

#[allow(clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Load .env file if present
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load configuration
    let config = AppConfig::from_env().expect("Failed to load configuration");

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .expect("Invalid bind address");

    tracing::info!("Pyoway landing server starting on {addr}");

    let app = build_router(&config);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server exited with error");
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[test]
    fn test_mime_type_js() {
        let path = PathBuf::from("test.js");
        assert_eq!(mime_type_from_path(&path), "application/javascript");
    }

    #[test]
    fn test_mime_type_wasm() {
        let path = PathBuf::from("test.wasm");
        assert_eq!(mime_type_from_path(&path), "application/wasm");
    }

    #[test]
    fn test_mime_type_css() {
        let path = PathBuf::from("test.css");
        assert_eq!(mime_type_from_path(&path), "text/css");
    }

    #[test]
    fn test_mime_type_html() {
        let path = PathBuf::from("test.html");
        assert_eq!(mime_type_from_path(&path), "text/html");
    }

    #[test]
    fn test_mime_type_png() {
        let path = PathBuf::from("test.png");
        assert_eq!(mime_type_from_path(&path), "image/png");
    }

    #[test]
    fn test_mime_type_svg() {
        let path = PathBuf::from("test.svg");
        assert_eq!(mime_type_from_path(&path), "image/svg+xml");
    }

    #[test]
    fn test_mime_type_ico() {
        let path = PathBuf::from("test.ico");
        assert_eq!(mime_type_from_path(&path), "image/x-icon");
    }

    #[test]
    fn test_mime_type_woff2() {
        let path = PathBuf::from("test.woff2");
        assert_eq!(mime_type_from_path(&path), "font/woff2");
    }

    #[test]
    fn test_mime_type_json() {
        let path = PathBuf::from("test.json");
        assert_eq!(mime_type_from_path(&path), "application/json");
    }

    #[test]
    fn test_mime_type_unknown() {
        let path = PathBuf::from("test.xyz");
        assert_eq!(mime_type_from_path(&path), "application/octet-stream");
    }

    #[test]
    fn test_mime_type_no_extension() {
        let path = PathBuf::from("README");
        assert_eq!(mime_type_from_path(&path), "application/octet-stream");
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn test_health_check_method() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        // POST to /health should return 405 Method Not Allowed
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_404_fallback_when_dist_missing() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/some-unknown-page")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Not Found");
    }

    #[tokio::test]
    async fn test_root_fallback_when_dist_missing() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_path_traversal_sanitized() {
        // Path traversal attempts via "../" are sanitized by filtering out
        // dangerous segments, keeping the result within `dist/`. The handler
        // then tries to serve the (non-existent) file, returning NOT_FOUND.
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        // "../../etc/passwd" -> "etc/passwd" after sanitization
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/../../etc/passwd")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Traversal is sanitized, not caught by the startswith check.
        // The file won't exist, so we get NOT_FOUND — meaning the path
        // stayed safely within dist/ and no escape occurred.
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_path_traversal_dot_sanitized() {
        let config = AppConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
            cors_origin: "http://localhost:8080".to_string(),
        };
        let app = build_router(&config);

        // "././../secret" -> "secret" after sanitization
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/././../secret")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
