//! Order model

use crate::models::order_type::Type;

/// Order
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Creation Time
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    /// Applicable to quantity orders
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<f32>,
    /// Applicable to value orders
    #[serde(rename = "filledValue", skip_serializing_if = "Option::is_none")]
    pub filled_value: Option<f32>,
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
    /// Applicable to LIMIT and STOP_LIMIT orders
    #[serde(rename = "limitPrice", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f32>,
    /// Applicable to quantity orders
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Order status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Applicable to STOP and STOP_LIMIT orders
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f32>,
    /// Stragetgy
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    /// Unique instrument identifier. Get from the /instruments endpoint
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Order type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Applicable to value orders
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
}

impl Order {
    /// Create a new `Order`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            creation_time: None,
            filled_quantity: None,
            filled_value: None,
            id: 0,
            limit_price: None,
            quantity: None,
            status: None,
            stop_price: None,
            strategy: None,
            ticker: None,
            r#type: None,
            value: None,
        }
    }
}

impl Default for Order {
    fn default() -> Self {
        Self::new()
    }
}

/// Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Local
    #[serde(rename = "LOCAL")]
    Local,
    /// Unconfirmed
    #[serde(rename = "UNCONFIRMED")]
    Unconfirmed,
    /// Confirmed
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    /// New
    #[serde(rename = "NEW")]
    New,
    /// Pending
    #[serde(rename = "CANCELLING")]
    Cancelling,
    /// Cancelled
    #[serde(rename = "CANCELLED")]
    Cancelled,
    /// Partially filled
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
}

impl Default for Status {
    fn default() -> Self {
        Self::Local
    }
}
/// Strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    /// Quantity
    #[serde(rename = "QUANTITY")]
    Quantity,
    /// Value
    #[serde(rename = "VALUE")]
    Value,
}

impl Default for Strategy {
    fn default() -> Self {
        Self::Quantity
    }
}
