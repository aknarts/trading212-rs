//! Tradeable instrument

/// Tradeable instrument
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TradeableInstrument {
    /// On the platform since
    #[serde(rename = "addedOn", skip_serializing_if = "Option::is_none")]
    pub added_on: Option<String>,
    /// ISO 4217
    #[serde(rename = "currencyCode", skip_serializing_if = "Option::is_none")]
    pub currency_code: String,
    /// ISIN
    #[serde(rename = "isin", skip_serializing_if = "Option::is_none")]
    pub isin: Option<String>,
    /// A single order must be equal to or less than this value
    #[serde(rename = "maxOpenQuantity", skip_serializing_if = "Option::is_none")]
    pub max_open_quantity: Option<f32>,
    /// A single order must be equal to or exceed this value
    #[serde(rename = "minTradeQuantity", skip_serializing_if = "Option::is_none")]
    pub min_trade_quantity: Option<f32>,
    /// Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Short name
    #[serde(rename = "shortname", skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// Unique identifier
    #[serde(rename = "ticker")]
    pub ticker: String,
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Get items in the /exchanges endpoint
    #[serde(rename = "workingScheduleId", skip_serializing_if = "Option::is_none")]
    pub working_schedule_id: Option<i64>,
}

impl TradeableInstrument {
    /// Create a new `TradeableInstrument`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            added_on: None,
            currency_code: String::new(),
            isin: None,
            max_open_quantity: None,
            min_trade_quantity: None,
            name: None,
            shortname: None,
            ticker: String::new(),
            r#type: None,
            working_schedule_id: None,
        }
    }
}

impl Default for TradeableInstrument {
    fn default() -> Self {
        Self::new()
    }
}

/// Instrument Type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Cryptocurrency
    #[serde(rename = "CRYPTOCURRENCY")]
    Cryptocurrency,
    /// ETF
    #[serde(rename = "ETF")]
    Etf,
    /// Forex
    #[serde(rename = "FOREX")]
    Forex,
    /// Future
    #[serde(rename = "FUTURES")]
    Futures,
    /// Index
    #[serde(rename = "INDEX")]
    Index,
    /// Stock
    #[serde(rename = "STOCK")]
    Stock,
    /// Warrant
    #[serde(rename = "WARRANT")]
    Warrant,
    /// Crypto
    #[serde(rename = "CRYPTO")]
    Crypto,
    /// CVR
    #[serde(rename = "CVR")]
    Cvr,
    /// Corpact
    #[serde(rename = "CORPACT")]
    Corpact,
}

impl Default for Type {
    fn default() -> Self {
        Self::Cryptocurrency
    }
}
