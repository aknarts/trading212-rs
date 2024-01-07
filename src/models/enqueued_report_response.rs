//! `EnqueuedReportResponse` type.

/// Enqueued report response.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnqueuedReportResponse {
    /// Report identifier
    #[serde(rename = "reportId", skip_serializing_if = "Option::is_none")]
    pub report_id: Option<i64>,
}

impl EnqueuedReportResponse {
    /// Create a new `EnqueuedReportResponse`.
    #[must_use]
    pub const fn new() -> Self {
        Self { report_id: None }
    }
}

impl Default for EnqueuedReportResponse {
    fn default() -> Self {
        Self::new()
    }
}
