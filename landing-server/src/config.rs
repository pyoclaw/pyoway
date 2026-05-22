//! Application configuration loaded from environment variables.

use std::num::ParseIntError;

/// Configuration-related errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Failed to parse an environment variable.
    #[error("Failed to parse {key}: {source}")]
    EnvParse {
        /// The environment variable name.
        key: String,
        /// The underlying parse error.
        #[source]
        source: ParseIntError,
    },
}

/// Application configuration.
///
/// Loaded from environment variables with dotenvy support for `.env` files.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Host address to bind the server to.
    pub host: String,
    /// Port number to listen on.
    pub port: u16,
    /// Allowed CORS origin (e.g., `http://localhost:8080`).
    pub cors_origin: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing
    /// or cannot be parsed.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_raw(
            std::env::var("HOST").ok(),
            std::env::var("PORT").ok(),
            std::env::var("CORS_ORIGIN").ok(),
        )
    }

    /// Build configuration from optional raw string values.
    ///
    /// `None` values fall back to their defaults. This is exposed for testing
    /// to avoid manipulating environment variables.
    fn from_raw(
        host: Option<String>,
        port: Option<String>,
        cors_origin: Option<String>,
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            host: host.unwrap_or_else(|| "127.0.0.1".to_string()),
            port: port
                .unwrap_or_else(|| "8080".to_string())
                .parse()
                .map_err(|e| ConfigError::EnvParse {
                    key: "PORT".to_string(),
                    source: e,
                })?,
            cors_origin: cors_origin.unwrap_or_else(|| "http://localhost:8080".to_string()),
            // RUST_LOG is read directly by tracing-subscriber; no need to store it
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        let config = AppConfig::from_raw(None, None, None).unwrap();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.cors_origin, "http://localhost:8080");
    }

    #[test]
    fn test_custom_values() {
        let config = AppConfig::from_raw(
            Some("0.0.0.0".to_string()),
            Some("3000".to_string()),
            Some("https://example.com".to_string()),
        )
        .unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 3000);
        assert_eq!(config.cors_origin, "https://example.com");
    }

    #[test]
    fn test_invalid_port() {
        let result = AppConfig::from_raw(None, Some("not-a-number".to_string()), None);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            ConfigError::EnvParse { key, .. } => {
                assert_eq!(key, "PORT");
            }
        }
    }

    #[test]
    fn test_port_out_of_range() {
        let result = AppConfig::from_raw(None, Some("99999".to_string()), None);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            ConfigError::EnvParse { key, .. } => {
                assert_eq!(key, "PORT");
            }
        }
    }

    #[test]
    fn test_partial_override() {
        let config = AppConfig::from_raw(Some("0.0.0.0".to_string()), None, None).unwrap();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert_eq!(config.cors_origin, "http://localhost:8080");
    }

    #[test]
    fn test_empty_host() {
        let config = AppConfig::from_raw(Some(String::new()), None, None).unwrap();
        assert_eq!(config.host, "");
        assert_eq!(config.port, 8080);
    }
}
