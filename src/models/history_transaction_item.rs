//! History transaction item model

/// History transaction item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryTransactionItem {
    /// In the account currency
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Date and time
    #[serde(rename = "dateTime", skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
    /// ID
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl HistoryTransactionItem {
    /// Create a new `HistoryTransactionItem`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            amount: None,
            date_time: None,
            reference: None,
            r#type: None,
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
}

impl Default for Type {
    fn default() -> Self {
        Self::Withdraw
    }
}
