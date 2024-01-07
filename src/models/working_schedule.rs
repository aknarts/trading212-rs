//! `WorkingSchedule` model
//! Used by the Exchange model

/// `WorkingSchedule`
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkingSchedule {
    /// Unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Time events
    #[serde(rename = "timeEvents", skip_serializing_if = "Option::is_none")]
    pub time_events: Option<Vec<crate::models::time_event::TimeEvent>>,
}

impl WorkingSchedule {
    /// Create a new `WorkingSchedule`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            time_events: None,
        }
    }
}

impl Default for WorkingSchedule {
    fn default() -> Self {
        Self::new()
    }
}
