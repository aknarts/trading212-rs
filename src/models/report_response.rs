//! {`ReportResponse`} struct (with {Status} enum)

/// Report Response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReportResponse {
    /// Data included
    #[serde(rename = "dataIncluded", skip_serializing_if = "Option::is_none")]
    pub data_included: Option<Box<crate::models::report_data_included::ReportDataIncluded>>,
    /// Download link
    #[serde(rename = "downloadLink", skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    /// Report id
    #[serde(rename = "reportId", skip_serializing_if = "Option::is_none")]
    pub report_id: Option<i64>,
    /// Status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Time from
    #[serde(rename = "timeFrom", skip_serializing_if = "Option::is_none")]
    pub time_from: Option<String>,
    /// Time to
    #[serde(rename = "timeTo", skip_serializing_if = "Option::is_none")]
    pub time_to: Option<String>,
}

impl ReportResponse {
    /// Create a new `ReportResponse`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data_included: None,
            download_link: None,
            report_id: None,
            status: None,
            time_from: None,
            time_to: None,
        }
    }
}

impl Default for ReportResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// Report status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    /// Queued
    #[serde(rename = "Queued")]
    Queued,
    /// Processing
    #[serde(rename = "Processing")]
    Processing,
    /// Running
    #[serde(rename = "Running")]
    Running,
    /// Canceled
    #[serde(rename = "Canceled")]
    Canceled,
    /// Failed
    #[serde(rename = "Failed")]
    Failed,
    /// Finished
    #[serde(rename = "Finished")]
    Finished,
}

impl Default for Status {
    fn default() -> Self {
        Self::Queued
    }
}
