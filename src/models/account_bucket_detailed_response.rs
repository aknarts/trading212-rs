//! Models for detailed account pie response

use crate::models::dividend_cache_action::DividendCashAction;
use crate::models::icon::Icon;
use serde_with::serde_as;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

/// Pie response
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountBucketDetailedResponse {
    /// Creation date
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// Dividend cash action
    #[serde(rename = "dividendCashAction")]
    pub dividend_cash_action: DividendCashAction,
    /// End date
    #[serde_as(as = "Iso8601")]
    #[serde(rename = "endDate")]
    pub end_date: OffsetDateTime,
    /// Goal
    #[serde(rename = "goal", skip_serializing_if = "Option::is_none")]
    pub goal: Option<f32>,
    /// Icon
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    /// Id
    #[serde(rename = "id")]
    pub id: i64,
    /// Initial investment
    #[serde(rename = "initialInvestment", skip_serializing_if = "Option::is_none")]
    pub initial_investment: Option<f32>,
    /// Instrument shares
    #[serde(rename = "instrumentShares", skip_serializing_if = "Option::is_none")]
    pub instrument_shares: Option<::std::collections::HashMap<String, f32>>,
    /// Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Public url
    #[serde(rename = "pubicUrl", skip_serializing_if = "Option::is_none")]
    pub pubic_url: Option<String>,
}

impl AccountBucketDetailedResponse {
    /// Create a new `AccountBucketDetailedResponse`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            creation_date: None,
            dividend_cash_action: DividendCashAction::default(),
            end_date: OffsetDateTime::now_utc(),
            goal: None,
            icon: None,
            id: 0,
            initial_investment: None,
            instrument_shares: None,
            name: None,
            pubic_url: None,
        }
    }
}

impl Default for AccountBucketDetailedResponse {
    fn default() -> Self {
        Self::new()
    }
}
