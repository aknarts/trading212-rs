//! Historical order model

use crate::models::order_type::Type;
use crate::models::time_validity::TimeValidity;
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Historical order model
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoricalOrder {
    /// Date created
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Rfc3339>")]
    pub date_created: Option<OffsetDateTime>,
    /// Date executed
    #[serde(rename = "dateExecuted", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Rfc3339>")]
    pub date_executed: Option<OffsetDateTime>,
    /// Date modified
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Rfc3339>")]
    pub date_modified: Option<OffsetDateTime>,
    /// Executor
    #[serde(rename = "executor", skip_serializing_if = "Option::is_none")]
    pub executor: Option<crate::models::executor::Executor>,
    /// In the instrument currency
    #[serde(rename = "fillCost", skip_serializing_if = "Option::is_none")]
    pub fill_cost: Option<f32>,
    /// Fill ID
    #[serde(rename = "fillId", skip_serializing_if = "Option::is_none")]
    pub fill_id: Option<i64>,
    /// In the instrument currency
    #[serde(rename = "fillPrice", skip_serializing_if = "Option::is_none")]
    pub fill_price: Option<f32>,
    /// Fill result
    #[serde(rename = "fillResult", skip_serializing_if = "Option::is_none")]
    pub fill_result: Option<f32>,
    /// Fill type
    #[serde(rename = "fillType", skip_serializing_if = "Option::is_none")]
    pub fill_type: Option<FillType>,
    /// Applicable to quantity orders
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<f32>,
    /// Applicable to value orders
    #[serde(rename = "filledValue", skip_serializing_if = "Option::is_none")]
    pub filled_value: Option<f32>,
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
    /// Applicable to limit orders
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f32>,
    /// Applicable to quantity orders
    #[serde(rename = "orderedQuantity", skip_serializing_if = "Option::is_none")]
    pub ordered_quantity: Option<f32>,
    /// Applicable to value orders
    #[serde(rename = "orderedValue", skip_serializing_if = "Option::is_none")]
    pub ordered_value: Option<f32>,
    /// Parent order
    #[serde(rename = "parentOrder", skip_serializing_if = "Option::is_none")]
    pub parent_order: Option<i64>,
    /// Status
    #[serde(rename = "status")]
    pub status: Status,
    /// Applicable to stop orders
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f32>,
    /// Taxes
    #[serde(rename = "taxes")]
    pub taxes: Vec<crate::models::tax::Tax>,
    /// Ticker
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Applicable to stop, limit and stopLimit orders
    #[serde(rename = "timeValidity", skip_serializing_if = "Option::is_none")]
    pub time_validity: Option<TimeValidity>,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl HistoricalOrder {
    /// Create a new `HistoricalOrder`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            date_created: None,
            date_executed: None,
            date_modified: None,
            executor: None,
            fill_cost: None,
            fill_id: None,
            fill_price: None,
            fill_result: None,
            fill_type: None,
            filled_quantity: None,
            filled_value: None,
            id: 0,
            limit_price: None,
            ordered_quantity: None,
            ordered_value: None,
            parent_order: None,
            status: Status::Unknown,
            stop_price: None,
            taxes: Vec::new(),
            ticker: None,
            time_validity: None,
            r#type: Type::Unknown,
        }
    }
}

impl Default for HistoricalOrder {
    fn default() -> Self {
        Self::new()
    }
}

/// Fill type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FillType {
    /// TOTV
    #[serde(rename = "TOTV")]
    Totv,
    /// OTC
    #[serde(rename = "OTC")]
    Otc,
}

impl Default for FillType {
    fn default() -> Self {
        Self::Totv
    }
}
/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    /// LOCAL
    #[serde(rename = "LOCAL")]
    Local,
    /// Unconfirmed
    #[serde(rename = "UNCONFIRMED")]
    Unconfirmed,
    /// CONFIRMED
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    /// NEW
    #[serde(rename = "NEW")]
    New,
    /// Cancelling
    #[serde(rename = "CANCELLING")]
    Cancelling,
    /// CANCELLED
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// PARTIALLY_FILLED
    #[serde(rename = "PARTIALLY_FILLED")]
    PartiallyFilled,
    /// Filled
    #[serde(rename = "FILLED")]
    Filled,
    /// Rejected
    #[serde(rename = "REJECTED")]
    Rejected,
    /// Replacing
    #[serde(rename = "REPLACING")]
    Replacing,
    /// Replaced
    #[serde(rename = "REPLACED")]
    Replaced,
    /// Unknown
    #[serde(other)]
    Unknown,
}

impl Default for Status {
    fn default() -> Self {
        Self::Local
    }
}
