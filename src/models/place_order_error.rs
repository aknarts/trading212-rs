//! Place order error model

/// Place order error
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlaceOrderError {
    /// Clarification
    #[serde(rename = "clarification", skip_serializing_if = "Option::is_none")]
    pub clarification: Option<String>,
    /// Code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
}

impl PlaceOrderError {
    /// Create a new `PlaceOrderError`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            clarification: None,
            code: None,
        }
    }
}

impl Default for PlaceOrderError {
    fn default() -> Self {
        Self::new()
    }
}

/// Error code
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    /// Selling equity not owned
    #[serde(rename = "SellingEquityNotOwned")]
    SellingEquityNotOwned,
    /// Cant legally trade
    #[serde(rename = "CantLegalyTradeException")]
    CantLegalyTradeException,
    /// Insufficient resources
    #[serde(rename = "InsufficientResources")]
    InsufficientResources,
    /// Insufficient value for stocks sell
    #[serde(rename = "InsufficientValueForStocksSell")]
    InsufficientValueForStocksSell,
    /// Target price too far
    #[serde(rename = "TargetPriceTooFar")]
    TargetPriceTooFar,
    /// Target price too close
    #[serde(rename = "TargetPriceTooClose")]
    TargetPriceTooClose,
    /// Not eligible for ISA
    #[serde(rename = "NotEligibleForISA")]
    NotEligibleForIsa,
    /// Share lending agreement not accepted
    #[serde(rename = "ShareLendingAgreementNotAccepted")]
    ShareLendingAgreementNotAccepted,
    /// Instrument not found
    #[serde(rename = "InstrumentNotFound")]
    InstrumentNotFound,
    /// Max equity buy quantity exceeded
    #[serde(rename = "MaxEquityBuyQuantityExceeded")]
    MaxEquityBuyQuantityExceeded,
    /// Max equity sell quantity exceeded
    #[serde(rename = "MaxEquitySellQuantityExceeded")]
    MaxEquitySellQuantityExceeded,
    /// Limit price missing
    #[serde(rename = "LimitPriceMissing")]
    LimitPriceMissing,
    /// Stop price missing
    #[serde(rename = "StopPriceMissing")]
    StopPriceMissing,
    /// Ticker missing
    #[serde(rename = "TickerMissing")]
    TickerMissing,
    /// Quantity missing
    #[serde(rename = "QuantityMissing")]
    QuantityMissing,
    /// Max quantity exceeded
    #[serde(rename = "MaxQuantityExceeded")]
    MaxQuantityExceeded,
    /// Invalid Value
    #[serde(rename = "InvalidValue")]
    InvalidValue,
    /// Insufficient free for stocks
    #[serde(rename = "InsufficientFreeForStocksException")]
    InsufficientFreeForStocksException,
    /// Min value exceeded
    #[serde(rename = "MinValueExceeded")]
    MinValueExceeded,
    /// Min quantity exceeded
    #[serde(rename = "MinQuantityExceeded")]
    MinQuantityExceeded,
    /// Price too far
    #[serde(rename = "PriceTooFar")]
    PriceTooFar,
    /// Undefined
    #[serde(rename = "UNDEFINED")]
    Undefined,
    /// Not available for real money accounts
    #[serde(rename = "NotAvailableForRealMoneyAccounts")]
    NotAvailableForRealMoneyAccounts,
}

impl Default for Code {
    fn default() -> Self {
        Self::SellingEquityNotOwned
    }
}
