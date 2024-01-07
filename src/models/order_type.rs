//! Order type model

/// Type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    /// LIMIT
    #[serde(rename = "LIMIT")]
    Limit,
    /// Stop
    #[serde(rename = "STOP")]
    Stop,
    /// Market
    #[serde(rename = "MARKET")]
    Market,
    /// Stop limit
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
}

impl Default for Type {
    fn default() -> Self {
        Self::Limit
    }
}
