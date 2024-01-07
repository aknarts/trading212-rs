//! Dividend details model.

/// Dividend details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DividendDetails {
    /// Gained.
    #[serde(rename = "gained", skip_serializing_if = "Option::is_none")]
    pub gained: Option<f32>,
    /// In cash.
    #[serde(rename = "inCash", skip_serializing_if = "Option::is_none")]
    pub in_cash: Option<f32>,
    /// Reinvested.
    #[serde(rename = "reinvested", skip_serializing_if = "Option::is_none")]
    pub reinvested: Option<f32>,
}

impl DividendDetails {
    /// Create a new `DividendDetails`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gained: None,
            in_cash: None,
            reinvested: None,
        }
    }
}

impl Default for DividendDetails {
    fn default() -> Self {
        Self::new()
    }
}
