//! `Position` model

use crate::models::executor::Executor;

/// `Position`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Average price
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f32>,
    /// Current price
    #[serde(rename = "currentPrice", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<f32>,
    /// Origin
    #[serde(rename = "frontend", skip_serializing_if = "Option::is_none")]
    pub frontend: Option<Executor>,
    /// Forex movement impact, only applies to positions with instrument currency that differs from the accounts'
    #[serde(rename = "fxPpl", skip_serializing_if = "Option::is_none")]
    pub fx_ppl: Option<f32>,
    /// Initial fill date
    #[serde(rename = "initialFillDate", skip_serializing_if = "Option::is_none")]
    pub initial_fill_date: Option<String>,
    /// Additional quantity that can be bought
    #[serde(rename = "maxBuy", skip_serializing_if = "Option::is_none")]
    pub max_buy: Option<f32>,
    /// Quantity that can be sold
    #[serde(rename = "maxSell", skip_serializing_if = "Option::is_none")]
    pub max_sell: Option<f32>,
    /// Invested in pies
    #[serde(rename = "pieQuantity", skip_serializing_if = "Option::is_none")]
    pub pie_quantity: Option<f32>,
    /// PPL
    #[serde(rename = "ppl", skip_serializing_if = "Option::is_none")]
    pub ppl: Option<f32>,
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Unique instrument identifier
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
}

impl Position {
    /// Create a new `Position`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            average_price: None,
            current_price: None,
            frontend: None,
            fx_ppl: None,
            initial_fill_date: None,
            max_buy: None,
            max_sell: None,
            pie_quantity: None,
            ppl: None,
            quantity: None,
            ticker: None,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
