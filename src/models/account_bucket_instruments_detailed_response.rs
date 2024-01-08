//! `AccountBucketInstrumentsDetailedResponse` model

/// Account Pie instruments detailed response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketInstrumentsDetailedResponse {
    /// Instruments
    #[serde(rename = "instruments")]
    pub instruments:
        Vec<crate::models::account_bucket_instrument_result::AccountBucketInstrumentResult>,
    /// Settings
    #[serde(rename = "settings")]
    pub settings: crate::models::account_bucket_detailed_response::AccountBucketDetailedResponse,
}

impl AccountBucketInstrumentsDetailedResponse {
    /// Create a new `AccountBucketInstrumentsDetailedResponse`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            instruments: Vec::new(),
            settings:
                crate::models::account_bucket_detailed_response::AccountBucketDetailedResponse::new(
                ),
        }
    }
}

impl Default for AccountBucketInstrumentsDetailedResponse {
    fn default() -> Self {
        Self::new()
    }
}
