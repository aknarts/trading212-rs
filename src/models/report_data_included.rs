//! Report data included model.

/// Report data included
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReportDataIncluded {
    /// Include dividends
    #[serde(rename = "includeDividends")]
    pub include_dividends: bool,
    /// Include interest
    #[serde(rename = "includeInterest")]
    pub include_interest: bool,
    /// Include orders
    #[serde(rename = "includeOrders")]
    pub include_orders: bool,
    /// Include transactions
    #[serde(rename = "includeTransactions")]
    pub include_transactions: bool,
}

impl ReportDataIncluded {
    /// Create a new `ReportDataIncluded`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            include_dividends: false,
            include_interest: false,
            include_orders: false,
            include_transactions: false,
        }
    }
}

impl Default for ReportDataIncluded {
    fn default() -> Self {
        Self::new()
    }
}
