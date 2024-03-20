//! `Exchange` model

use crate::models::time_event::Type;
use std::cmp::Ordering;

/// `Exchange`
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
    /// Name
    #[serde(rename = "name")]
    pub name: String,
    /// Working schedules
    #[serde(rename = "workingSchedules")]
    pub working_schedules: Vec<crate::models::working_schedule::WorkingSchedule>,
}

impl Exchange {
    /// Create a new `Exchange`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            working_schedules: Vec::new(),
        }
    }

    /// Get the current types
    pub fn current_types(&self) -> Vec<Type> {
        let mut result: Vec<Type> = Vec::new();
        for schedule in self.working_schedules.clone() {
            result.push(schedule.current_type());
        }
        result
    }

    /// Get next events
    pub fn next_events(&self) -> Vec<crate::models::time_event::TimeEvent> {
        let mut result: Vec<crate::models::time_event::TimeEvent> = Vec::new();
        for schedule in self.working_schedules.clone() {
            if let Some(event) = schedule.next_event() {
                result.push(event);
            }
        }
        result
    }

    /// Get next event
    pub fn next_event(&self) -> Option<crate::models::time_event::TimeEvent> {
        let mut events = self.next_events();
        events.sort_by(|a, b| a.date.cmp(&b.date));
        events.first().cloned()
    }

    /// Get the current type
    pub fn current_type(&self) -> Type {
        let mut current = self.current_types();
        current.sort();
        current.first().copied().unwrap_or(Type::Unknown)
    }

    /// Is market open?
    pub fn is_open(&self) -> bool {
        self.current_types().contains(&Type::Open)
    }

    /// Is market closed?
    pub fn is_closed(&self) -> bool {
        self.current_types().contains(&Type::Close)
    }

    /// Is tradeable?
    pub fn is_tradeable(&self) -> bool {
        self.is_open() || self.is_pre_market() || self.is_after_hours()
    }

    /// Is market in pre-market?
    pub fn is_pre_market(&self) -> bool {
        self.current_types().contains(&Type::PreMarketOpen)
    }

    /// Is market in break?
    pub fn is_break(&self) -> bool {
        self.current_types().contains(&Type::BreakStart)
    }

    /// Is market after hours?
    pub fn is_after_hours(&self) -> bool {
        self.current_types().contains(&Type::AfterHoursOpen)
    }
}

impl Default for Exchange {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialOrd<Self> for Exchange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Exchange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
