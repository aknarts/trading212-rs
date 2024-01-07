//! Models for responses from the `/orders/historical` endpoint

/// Paginated response of historical orders
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedResponseHistoricalOrder {
    /// Items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::historical_order::HistoricalOrder>>,
    /// Next page path.
    #[serde(rename = "nextPagePath", skip_serializing_if = "Option::is_none")]
    pub next_page_path: Option<String>,
}

impl PaginatedResponseHistoricalOrder {
    /// Create a new `PaginatedResponseHistoricalOrder`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            items: None,
            next_page_path: None,
        }
    }
}

impl Default for PaginatedResponseHistoricalOrder {
    fn default() -> Self {
        Self::new()
    }
}
