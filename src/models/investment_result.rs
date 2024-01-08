//! Investment result model

/// `InvestmentResult`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentResult {
    /// Invested value
    #[serde(rename = "investedValue")]
    pub invested_value: f32,
    /// Result
    #[serde(rename = "result")]
    pub result: f32,
    /// Result coef
    #[serde(rename = "resultCoef")]
    pub result_coef: f32,
    /// Value
    #[serde(rename = "value")]
    pub value: f32,
}

impl InvestmentResult {
    /// Create a new `InvestmentResult`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            invested_value: 0.0,
            result: 0.0,
            result_coef: 0.0,
            value: 0.0,
        }
    }
}

impl Default for InvestmentResult {
    fn default() -> Self {
        Self::new()
    }
}
