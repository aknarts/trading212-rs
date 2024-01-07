//! Time validity of an event

/// Expiration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeValidity {
    /// Good for the day
    #[serde(rename = "DAY")]
    Day,
    /// Good till cancelled
    #[serde(rename = "GTC")]
    Gtc,
}

impl Default for TimeValidity {
    fn default() -> Self {
        Self::Day
    }
}
