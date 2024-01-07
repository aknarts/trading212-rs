//! Instrument issue model.

/// Instrument issue.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InstrumentIssue {
    /// Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// Severity.
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
}

impl InstrumentIssue {
    /// Create a new `InstrumentIssue`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            name: None,
            severity: None,
        }
    }
}

impl Default for InstrumentIssue {
    fn default() -> Self {
        Self::new()
    }
}

/// Instrument issue name.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    /// Delisted.
    #[serde(rename = "DELISTED")]
    Delisted,
    /// Suspended.
    #[serde(rename = "SUSPENDED")]
    Suspended,
    /// No longer tradable.
    #[serde(rename = "NO_LONGER_TRADABLE")]
    NoLongerTradable,
    /// Max position size reached.
    #[serde(rename = "MAX_POSITION_SIZE_REACHED")]
    MaxPositionSizeReached,
    /// Approaching max position size.
    #[serde(rename = "APPROACHING_MAX_POSITION_SIZE")]
    ApproachingMaxPositionSize,
    /// Approaching max order size.
    #[serde(rename = "COMPLEX_INSTRUMENT_APP_TEST_REQUIRED")]
    ComplexInstrumentAppTestRequired,
}

impl Default for Name {
    fn default() -> Self {
        Self::Delisted
    }
}
/// Severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Severity {
    /// Irreversible.
    #[serde(rename = "IRREVERSIBLE")]
    Irreversible,
    /// Reversible.
    #[serde(rename = "REVERSIBLE")]
    Reversible,
    /// Informative.
    #[serde(rename = "INFORMATIVE")]
    Informative,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Irreversible
    }
}
