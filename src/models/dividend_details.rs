//! Dividend details model.
//! Used by the `AccountBucketResultResponse` model.

/// Dividend details.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DividendDetails {
    /// Gained.
    #[serde(rename = "gained")]
    pub gained: f32,
    /// In cash.
    #[serde(rename = "inCash")]
    pub in_cash: f32,
    /// Reinvested.
    #[serde(rename = "reinvested")]
    pub reinvested: f32,
}

impl DividendDetails {
    /// Create a new `DividendDetails`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            gained: 0.0,
            in_cash: 0.0,
            reinvested: 0.0,
        }
    }
}

impl Default for DividendDetails {
    fn default() -> Self {
        Self::new()
    }
}
