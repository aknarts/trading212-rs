//! `TimeEvent`

/// Time event
/// Used by the `WorkingSchedule` model
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeEvent {
    /// Date
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl TimeEvent {
    /// Create a new `TimeEvent`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            date: None,
            r#type: None,
        }
    }
}

impl Default for TimeEvent {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of event
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Open
    #[serde(rename = "OPEN")]
    Open,
    /// Close
    #[serde(rename = "CLOSE")]
    Close,
    /// Break start
    #[serde(rename = "BREAK_START")]
    BreakStart,
    /// Break end
    #[serde(rename = "BREAK_END")]
    BreakEnd,
    /// Pre-market open
    #[serde(rename = "PRE_MARKET_OPEN")]
    PreMarketOpen,
    /// Pre-market close
    #[serde(rename = "AFTER_HOURS_OPEN")]
    AfterHoursOpen,
    /// After-hours close
    #[serde(rename = "AFTER_HOURS_CLOSE")]
    AfterHoursClose,
}

impl Default for Type {
    fn default() -> Self {
        Self::Open
    }
}
