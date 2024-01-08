//! Cash model

/// `Cash`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cash {
    /// Blocked cash
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<f32>,
    /// Free cash
    #[serde(rename = "free", default)]
    pub free: f32,
    /// Invested cash
    #[serde(rename = "invested", default)]
    pub invested: f32,
    /// Invested cash in pies
    #[serde(rename = "pieCash", default)]
    pub pie_cash: f32,
    /// Projected Profit Loss
    #[serde(rename = "ppl", default)]
    pub ppl: f32,
    /// Result
    #[serde(rename = "result", default)]
    pub result: f32,
    /// Total cash
    #[serde(rename = "total", default)]
    pub total: f32,
}

impl Cash {
    /// Create a new `Cash`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blocked: None,
            free: 0.0,
            invested: 0.0,
            pie_cash: 0.0,
            ppl: 0.0,
            result: 0.0,
            total: 0.0,
        }
    }
}

impl Default for Cash {
    fn default() -> Self {
        Self::new()
    }
}
