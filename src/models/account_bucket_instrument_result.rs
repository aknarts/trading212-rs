//! Result of an instrument in a pie
//! Used in the response of the /account/buckets/{id}/instruments endpoint

/// Result of an instrument in a pie
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketInstrumentResult {
    /// Current share
    #[serde(rename = "currentShare", skip_serializing_if = "Option::is_none")]
    pub current_share: Option<f32>,
    /// Expected share
    #[serde(rename = "expectedShare")]
    pub expected_share: f32,
    /// Issues
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<crate::models::instrument_issue::InstrumentIssue>>,
    /// Owned quantity
    #[serde(rename = "ownedQuantity", skip_serializing_if = "Option::is_none")]
    pub owned_quantity: Option<f32>,
    /// Result
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<crate::models::investment_result::InvestmentResult>>,
    /// Ticker
    #[serde(rename = "ticker")]
    pub ticker: String,
}

impl AccountBucketInstrumentResult {
    /// Create a new `AccountBucketInstrumentResult`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_share: None,
            expected_share: 0.0,
            issues: None,
            owned_quantity: None,
            result: None,
            ticker: String::new(),
        }
    }
}

impl Default for AccountBucketInstrumentResult {
    fn default() -> Self {
        Self::new()
    }
}
