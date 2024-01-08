//! Pie creation request model
//! Used by the `PieControllerApi::create_pie` endpoint

use crate::models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse;
use crate::models::dividend_cache_action::DividendCashAction;
use crate::models::icon::Icon;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// `PieRequest`
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PieRequest {
    /// What action to take with the dividend cash.
    #[serde(rename = "dividendCashAction")]
    pub dividend_cash_action: DividendCashAction,
    /// End date for the goal
    #[serde(rename = "endDate")]
    #[serde_as(as = "Rfc3339")]
    pub end_date: OffsetDateTime,
    /// Goal
    #[serde(rename = "goal")]
    pub goal: f32,
    /// Icon
    #[serde(rename = "icon")]
    pub icon: Icon,
    /// Instrument shares
    #[serde(rename = "instrumentShares")]
    pub instrument_shares: ::std::collections::HashMap<String, f32>,
    /// Name
    #[serde(rename = "name")]
    pub name: String,
}

impl PieRequest {
    /// Create a new `PieRequest`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dividend_cash_action: DividendCashAction::Reinvest,
            end_date: OffsetDateTime::now_utc(),
            goal: 0.0,
            icon: Icon::Home,
            instrument_shares: std::collections::HashMap::new(),
            name: String::new(),
        }
    }
}

impl TryFrom<AccountBucketInstrumentsDetailedResponse> for PieRequest {
    type Error = ();

    fn try_from(bucket: AccountBucketInstrumentsDetailedResponse) -> Result<Self, Self::Error> {
        let mut instrument_shares = std::collections::HashMap::new();

        for instrument in bucket.instruments {
            instrument_shares.insert(instrument.ticker, instrument.expected_share);
        }

        Ok(Self {
            dividend_cash_action: bucket.settings.dividend_cash_action.clone(),
            end_date: bucket
                .settings
                .end_date
                .clone()
                .unwrap_or(OffsetDateTime::now_utc()),
            goal: bucket.settings.goal.clone().unwrap_or_default(),
            icon: bucket.settings.icon.clone().unwrap_or_default(),
            instrument_shares,
            name: bucket.settings.name.clone(),
        })
    }
}

impl Default for PieRequest {
    fn default() -> Self {
        Self::new()
    }
}
