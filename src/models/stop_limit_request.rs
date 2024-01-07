//! Stop limit request model

use crate::models::time_validity::TimeValidity;

/// Stop limit request model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StopLimitRequest {
    /// Limit price
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f32>,
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

impl StopLimitRequest {
    /// Create a new `StopLimitRequest`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limit_price: None,
            quantity: None,
            stop_price: None,
            ticker: None,
            time_validity: None,
        }
    }
}

impl Default for StopLimitRequest {
    fn default() -> Self {
        Self::new()
    }
}
