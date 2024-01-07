//! Executor model

/// `Executor`'s supported by the API
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Executor {
    /// API
    #[serde(rename = "API")]
    Api,
    /// IOS
    #[serde(rename = "IOS")]
    Ios,
    /// ANDROID
    #[serde(rename = "ANDROID")]
    Android,
    /// WEB
    #[serde(rename = "WEB")]
    Web,
    /// SYSTEM
    #[serde(rename = "SYSTEM")]
    System,
    /// AUTOINVEST
    #[serde(rename = "AUTOINVEST")]
    Autoinvest,
}

impl Default for Executor {
    fn default() -> Self {
        Self::Api
    }
}
