//! Market request model

/// Market request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketRequest {
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

impl MarketRequest {
    /// Create a new `MarketRequest`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            quantity: None,
            ticker: None,
        }
    }
}

impl Default for MarketRequest {
    fn default() -> Self {
        Self::new()
    }
}
