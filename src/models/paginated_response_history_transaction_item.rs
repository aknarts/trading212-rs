//! Models for responses from the `/history/transaction` endpoint.

/// History Transaction Item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponseHistoryTransactionItem {
    /// Items.
    #[serde(rename = "items")]
    pub items: Vec<crate::models::history_transaction_item::HistoryTransactionItem>,
    /// Next page path.
    #[serde(rename = "nextPagePath", skip_serializing_if = "Option::is_none")]
    pub next_page_path: Option<String>,
}

impl PaginatedResponseHistoryTransactionItem {
    /// Create a new `PaginatedResponseHistoryTransactionItem`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            next_page_path: None,
        }
    }
}

impl Default for PaginatedResponseHistoryTransactionItem {
    fn default() -> Self {
        Self::new()
    }
}
