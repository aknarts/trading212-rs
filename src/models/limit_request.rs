//! Limit request model

use crate::models::time_validity::TimeValidity;

/// Limit request model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitRequest {
    /// Limit price
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f32>,
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Expiration
    #[serde(rename = "timeValidity", skip_serializing_if = "Option::is_none")]
    pub time_validity: Option<TimeValidity>,
}

impl LimitRequest {
    /// Create a new `LimitRequest`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limit_price: None,
            quantity: None,
            ticker: None,
            time_validity: None,
        }
    }
}

impl Default for LimitRequest {
    fn default() -> Self {
        Self::new()
    }
}
