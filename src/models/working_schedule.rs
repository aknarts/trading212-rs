//! `WorkingSchedule` model
//! Used by the Exchange model

/// `WorkingSchedule`
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WorkingSchedule {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
    /// Time events
    #[serde(rename = "timeEvents")]
    pub time_events: Vec<crate::models::time_event::TimeEvent>,
}

impl WorkingSchedule {
    /// Create a new `WorkingSchedule`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: 0,
            time_events: Vec::new(),
        }
    }

    /// Get the current type
    pub fn current_type(&self) -> crate::models::time_event::Type {
        let mut events = self.time_events.clone();
        events.sort_by(|a, b| a.date.cmp(&b.date));
        let mut last_type = crate::models::time_event::Type::Unknown;
        for event in events {
            if event.is_future() {
                return last_type;
            }
            last_type = event.r#type;
        }
        crate::models::time_event::Type::Unknown
    }

    /// Get next event
    pub fn next_event(&self) -> Option<crate::models::time_event::TimeEvent> {
        let mut events = self.time_events.clone();
        events.sort_by(|a, b| a.date.cmp(&b.date));
        for event in events {
            if event.is_future() {
                return Some(event);
            }
        }
        None
    }
}

impl Default for WorkingSchedule {
    fn default() -> Self {
        Self::new()
    }
}
