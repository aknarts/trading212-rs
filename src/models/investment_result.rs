//! Investment result model

/// `InvestmentResult`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentResult {
    /// Invested value
    #[serde(rename = "investedValue", skip_serializing_if = "Option::is_none")]
    pub invested_value: Option<f32>,
    /// Result
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<f32>,
    /// Result coef
    #[serde(rename = "resultCoef", skip_serializing_if = "Option::is_none")]
    pub result_coef: Option<f32>,
    /// Value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
}

impl InvestmentResult {
    /// Create a new `InvestmentResult`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            invested_value: None,
            result: None,
            result_coef: None,
            value: None,
        }
    }
}

impl Default for InvestmentResult {
    fn default() -> Self {
        Self::new()
    }
}
