//! `AccountBucketInstrumentsDetailedResponse` model

/// Account Pie instruments detailed response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketInstrumentsDetailedResponse {
    /// Instruments
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments:
        Option<Vec<crate::models::account_bucket_instrument_result::AccountBucketInstrumentResult>>,
    /// Settings
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings:
        Option<Box<crate::models::account_bucket_detailed_response::AccountBucketDetailedResponse>>,
}

impl AccountBucketInstrumentsDetailedResponse {
    /// Create a new `AccountBucketInstrumentsDetailedResponse`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            instruments: None,
            settings: None,
        }
    }
}

impl Default for AccountBucketInstrumentsDetailedResponse {
    fn default() -> Self {
        Self::new()
    }
}
