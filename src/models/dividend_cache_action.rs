//! Dividend cash action.

/// What action to take with the dividend cash.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DividendCashAction {
    /// Reinvest
    #[serde(rename = "REINVEST")]
    Reinvest,
    /// To account cash
    #[serde(rename = "TO_ACCOUNT_CASH")]
    ToAccountCash,
}

impl Default for DividendCashAction {
    fn default() -> Self {
        Self::Reinvest
    }
}
