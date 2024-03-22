//! Public report request model

use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// Public report request model
#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicReportRequest {
    /// What data should be included in the report
    #[serde(rename = "dataIncluded")]
    pub data_included: crate::models::report_data_included::ReportDataIncluded,
    /// Time from
    #[serde_as(as = "Rfc3339")]
    #[serde(rename = "timeFrom")]
    pub time_from: OffsetDateTime,
    /// Time to
    #[serde_as(as = "Rfc3339")]
    #[serde(rename = "timeTo")]
    pub time_to: OffsetDateTime,
}

impl PublicReportRequest {
    /// Create a new `PublicReportRequest`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            data_included: crate::models::report_data_included::ReportDataIncluded::new(),
            time_from: OffsetDateTime::UNIX_EPOCH,
            time_to: OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Default for PublicReportRequest {
    fn default() -> Self {
        Self::new()
    }
}
