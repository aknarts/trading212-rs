//! Tradeable instrument

use time::OffsetDateTime;

/// Tradeable instrument
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeableInstrument {
    /// On the platform since
    #[serde(with = "time::serde::rfc3339")]
    pub added_on: time::OffsetDateTime,
    /// ISO 4217
    pub currency_code: String,
    /// ISIN
    pub isin: String,
    /// A single order must be equal to or less than this value
    pub max_open_quantity: f64,
    /// A single order must be equal to or exceed this value
    pub min_trade_quantity: f64,
    /// Name
    pub name: String,
    /// Short name
    pub short_name: String,
    /// Unique identifier
    pub ticker: String,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Get items in the /exchanges endpoint
    pub working_schedule_id: i64,
}

impl TradeableInstrument {
    /// Create a new `TradeableInstrument`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            added_on: OffsetDateTime::UNIX_EPOCH,
            currency_code: String::new(),
            isin: String::new(),
            max_open_quantity: 0.0,
            min_trade_quantity: 0.0,
            name: String::new(),
            short_name: String::new(),
            ticker: String::new(),
            r#type: Type::Unknown,
            working_schedule_id: 0,
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
    /// Unknown
    #[serde(other)]
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::Cryptocurrency
    }
}
