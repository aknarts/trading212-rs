//! Models for responses from the `/history/dividend` API endpoint

/// History dividend item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponseHistoryDividendItem {
    /// Items
    #[serde(rename = "items")]
    pub items: Vec<crate::models::history_dividend_item::HistoryDividendItem>,
    /// Next page path
    #[serde(rename = "nextPagePath", skip_serializing_if = "Option::is_none")]
    pub next_page_path: Option<String>,
}

impl PaginatedResponseHistoryDividendItem {
    /// Create a new `PaginatedResponseHistoryDividendItem`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: Vec::new(),
            next_page_path: None,
        }
    }
}

impl Default for PaginatedResponseHistoryDividendItem {
    fn default() -> Self {
        Self::new()
    }
}
