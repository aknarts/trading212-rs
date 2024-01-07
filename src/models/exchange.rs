//! `Exchange` model

/// `Exchange`
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Exchange {
    /// Unique identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Working schedules
    #[serde(rename = "workingSchedules", skip_serializing_if = "Option::is_none")]
    pub working_schedules: Option<Vec<crate::models::working_schedule::WorkingSchedule>>,
}

impl Exchange {
    /// Create a new `Exchange`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            name: None,
            working_schedules: None,
        }
    }
}

impl Default for Exchange {
    fn default() -> Self {
        Self::new()
    }
}
