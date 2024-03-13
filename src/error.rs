//! Error types
use thiserror::Error;

/// Error types
#[derive(Error, Debug)]
pub enum Error {
    /// Reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Serde JSON error
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// API limit reached
    #[error("We have hit the API limit, try again later")]
    Limit,
    /// Invalid token
    #[error("The token does not exist or is deactivated")]
    Token,
    /// Scope( metadata ) missing for API key
    #[error("Scope( metadata ) missing for API key")]
    Scope,
    /// Malformed request
    #[error("The request was malformed")]
    Malformed,
    /// Timed out request
    #[error("The request has timed out")]
    Timeout,
    /// Too large request
    #[error("The requesting too many items")]
    TooLarge,
    /// Invalid date format
    #[error("Invalid date format")]
    InvalidDateFormat,
    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    /// Missing body
    #[error("Missing body")]
    MissingBody,
    /// Invalid Client
    #[error("Invalid client")]
    NoClient,
}
