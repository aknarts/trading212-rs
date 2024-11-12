//! `TimeEvent`

use std::fmt;

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
    /// Overnight Open
    #[serce(rename = "OVERNIGHT_OPEN")]
    OvernightOpen,
    /// Unknown
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "Open"),
            Self::PreMarketOpen => write!(f, "Pre-market open"),
            Self::AfterHoursOpen => write!(f, "After-hours open"),
            Self::BreakStart => write!(f, "Break start"),
            Self::BreakEnd => write!(f, "Break end"),
            Self::Close => write!(f, "Close"),
            Self::AfterHoursClose => write!(f, "After-hours close"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Open
    }
}
