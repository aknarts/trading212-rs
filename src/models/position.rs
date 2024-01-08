//! `Position` model

use crate::models::executor::Executor;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `Position`
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Average price
    #[serde(rename = "averagePrice")]
    pub average_price: f32,
    /// Current price
    #[serde(rename = "currentPrice")]
    pub current_price: f32,
    /// Origin
    #[serde(rename = "frontend")]
    pub frontend: Executor,
    /// Forex movement impact, only applies to positions with instrument currency that differs from the accounts'
    #[serde(rename = "fxPpl", skip_serializing_if = "Option::is_none")]
    pub fx_ppl: Option<f32>,
    /// Initial fill date
    #[serde_as(as = "Rfc3339")]
    #[serde(rename = "initialFillDate")]
    pub initial_fill_date: OffsetDateTime,
    /// Additional quantity that can be bought
    #[serde(rename = "maxBuy", skip_serializing_if = "Option::is_none")]
    pub max_buy: Option<f32>,
    /// Quantity that can be sold
    #[serde(rename = "maxSell", skip_serializing_if = "Option::is_none")]
    pub max_sell: Option<f32>,
    /// Percentage invested in pies
    #[serde(rename = "pieQuantity")]
    pub pie_quantity: f32,
    /// Projected profit/loss
    #[serde(rename = "ppl")]
    pub ppl: f32,
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Unique instrument identifier
    #[serde(rename = "ticker")]
    pub ticker: String,
}

impl Position {
    /// Create a new `Position`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            average_price: 0.0,
            current_price: 0.0,
            frontend: Executor::default(),
            fx_ppl: None,
            initial_fill_date: OffsetDateTime::now_utc(),
            max_buy: None,
            max_sell: None,
            pie_quantity: 0.0,
            ppl: 0.0,
            quantity: None,
            ticker: String::new(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
