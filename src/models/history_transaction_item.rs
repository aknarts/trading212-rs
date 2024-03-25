//! History transaction item model

use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

/// History transaction item.
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryTransactionItem {
    /// In the account currency
    #[serde(rename = "amount")]
    pub amount: f32,
    /// Date and time
    #[serde(rename = "dateTime")]
    #[serde_as(as = "Rfc3339")]
    pub date_time: OffsetDateTime,
    /// ID
    #[serde(rename = "reference")]
    pub reference: Uuid,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl HistoryTransactionItem {
    /// Create a new `HistoryTransactionItem`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            amount: 0.0,
            date_time: OffsetDateTime::UNIX_EPOCH,
            reference: Uuid::nil(),
            r#type: Type::Unknown,
        }
    }
}

impl Default for HistoryTransactionItem {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Withdraw
    #[serde(rename = "WITHDRAW")]
    Withdraw,
    /// Deposit
    #[serde(rename = "DEPOSIT")]
    Deposit,
    /// Fee
    #[serde(rename = "FEE")]
    Fee,
    /// Transfer
    #[serde(rename = "TRANSFER")]
    Transfer,
    /// Unknown
    #[serde(other)]
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::Withdraw
    }
}
