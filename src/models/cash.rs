//! Cash model

/// `Cash`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cash {
    /// Blocked cash
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<f32>,
    /// Free cash
    #[serde(rename = "free", skip_serializing_if = "Option::is_none")]
    pub free: Option<f32>,
    /// Invested cash
    #[serde(rename = "invested", skip_serializing_if = "Option::is_none")]
    pub invested: Option<f32>,
    /// Invested cash in pies
    #[serde(rename = "pieCash", skip_serializing_if = "Option::is_none")]
    pub pie_cash: Option<f32>,
    /// PPL
    #[serde(rename = "ppl", skip_serializing_if = "Option::is_none")]
    pub ppl: Option<f32>,
    /// Result
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<f32>,
    /// Total cash
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
}

impl Cash {
    /// Create a new `Cash`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blocked: None,
            free: None,
            invested: None,
            pie_cash: None,
            ppl: None,
            result: None,
            total: None,
        }
    }
}

impl Default for Cash {
    fn default() -> Self {
        Self::new()
    }
}
