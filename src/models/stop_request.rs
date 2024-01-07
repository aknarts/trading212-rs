//! Stop request model

use crate::models::time_validity::TimeValidity;

/// Stop request model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopRequest {
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Stop price
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f32>,
    /// Ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Expiration
    #[serde(rename = "timeValidity", skip_serializing_if = "Option::is_none")]
    pub time_validity: Option<TimeValidity>,
}

impl StopRequest {
    /// Create a new `StopRequest`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            quantity: None,
            stop_price: None,
            ticker: None,
            time_validity: None,
        }
    }
}

impl Default for StopRequest {
    fn default() -> Self {
        Self::new()
    }
}
