//! Models for `AccountBucketResultResponse`

use crate::models::investment_result::InvestmentResult;

/// `AccountBucketResultResponse`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketResultResponse {
    /// Amount of money put into the pie in account currency
    #[serde(rename = "cash", skip_serializing_if = "Option::is_none")]
    pub cash: Option<f32>,
    /// Dividend details
    #[serde(rename = "dividendDetails", skip_serializing_if = "Option::is_none")]
    pub dividend_details: Option<Box<crate::models::dividend_details::DividendDetails>>,
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
    /// Progress of the pie based on the set goal
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<f32>,
    /// Result of the pie in account currency
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<InvestmentResult>>,
    /// Status of the pie based on the set goal
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl AccountBucketResultResponse {
    /// Create a new `AccountBucketResultResponse`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cash: None,
            dividend_details: None,
            id: 0,
            progress: None,
            result: None,
            status: None,
        }
    }
}

impl Default for AccountBucketResultResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Status of the pie based on the set goal
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Ahead of the set goal
    #[serde(rename = "AHEAD")]
    Ahead,
    /// On track to reach the set goal
    #[serde(rename = "ON_TRACK")]
    OnTrack,
    /// Behind the set goal
    #[serde(rename = "BEHIND")]
    Behind,
}

impl Default for Status {
    fn default() -> Self {
        Self::Ahead
    }
}
