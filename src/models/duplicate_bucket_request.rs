//! `DuplicateBucketRequest` model

/// `DuplicateBucketRequest`
/// Currently not used
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DuplicateBucketRequest {
    /// Icon
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl DuplicateBucketRequest {
    /// Create a new `DuplicateBucketRequest`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            icon: None,
            name: None,
        }
    }
}

impl Default for DuplicateBucketRequest {
    fn default() -> Self {
        Self::new()
    }
}
