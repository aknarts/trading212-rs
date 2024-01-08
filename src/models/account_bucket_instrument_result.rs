//! Result of an instrument in a pie
//! Used in the response of the /account/buckets/{id}/instruments endpoint

/// Result of an instrument in a pie
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketInstrumentResult {
    /// Current share
    #[serde(rename = "currentShare")]
    pub current_share: f32,
    /// Expected share
    #[serde(rename = "expectedShare")]
    pub expected_share: f32,
    /// Issues
    #[serde(rename = "issues")]
    pub issues: Vec<crate::models::instrument_issue::InstrumentIssue>,
    /// Owned quantity
    #[serde(rename = "ownedQuantity")]
    pub owned_quantity: f32,
    /// Result
    #[serde(rename = "result")]
    pub result: crate::models::investment_result::InvestmentResult,
    /// Ticker
    #[serde(rename = "ticker")]
    pub ticker: String,
}

impl AccountBucketInstrumentResult {
    /// Create a new `AccountBucketInstrumentResult`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_share: 0.0,
            expected_share: 0.0,
            issues: Vec::new(),
            owned_quantity: 0.0,
            result: crate::models::investment_result::InvestmentResult::new(),
            ticker: String::new(),
        }
    }
}

impl Default for AccountBucketInstrumentResult {
    fn default() -> Self {
        Self::new()
    }
}
