//! Account model

/// Account
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    /// ISO 4217
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    /// Unique identifier
    #[serde(rename = "id")]
    pub id: i64,
}

impl Account {
    /// Create a new `Account`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            currency_code: "EUR".to_string(),
            id: 0,
        }
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}
