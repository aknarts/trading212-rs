//! `TimeEvent`

/// Time event
/// Used by the `WorkingSchedule` model
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimeEvent {
    /// Date
    #[serde(rename = "date", with = "time::serde::rfc3339")]
    pub date: time::OffsetDateTime,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl TimeEvent {
    /// Create a new `TimeEvent`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            date: time::OffsetDateTime::UNIX_EPOCH,
            r#type: Type::Unknown,
        }
    }

    /// Is event in the past?
    pub fn is_past(&self) -> bool {
        self.date < time::OffsetDateTime::now_utc()
    }

    /// Is event in the future?
    pub fn is_future(&self) -> bool {
        self.date > time::OffsetDateTime::now_utc()
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
    /// Pre-market open
    #[serde(rename = "PRE_MARKET_OPEN")]
    PreMarketOpen,
    /// Pre-market close
    #[serde(rename = "AFTER_HOURS_OPEN")]
    AfterHoursOpen,
    /// Break start
    #[serde(rename = "BREAK_START")]
    BreakStart,
    /// Break end
    #[serde(rename = "BREAK_END")]
    BreakEnd,
    /// Close
    #[serde(rename = "CLOSE")]
    Close,
    /// After-hours close
    #[serde(rename = "AFTER_HOURS_CLOSE")]
    AfterHoursClose,
    /// Unknown
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::Open
    }
}
