//! Tax model.

/// Tax.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tax {
    /// Fill identifier.
    #[serde(rename = "fillId", skip_serializing_if = "Option::is_none")]
    pub fill_id: Option<String>,
    /// Name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// Quantity.
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Time charged.
    #[serde(rename = "timeCharged", skip_serializing_if = "Option::is_none")]
    pub time_charged: Option<String>,
}

impl Tax {
    /// Create a new `Tax`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fill_id: None,
            name: None,
            quantity: None,
            time_charged: None,
        }
    }
}

impl Default for Tax {
    fn default() -> Self {
        Self::new()
    }
}

/// Tax Name
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Name {
    /// Commission turnover
    #[serde(rename = "COMMISSION_TURNOVER")]
    CommissionTurnover,
    /// Currency conversion fee
    #[serde(rename = "CURRENCY_CONVERSION_FEE")]
    CurrencyConversionFee,
    /// Finra fee
    #[serde(rename = "FINRA_FEE")]
    FinraFee,
    /// French transaction tax
    #[serde(rename = "FRENCH_TRANSACTION_TAX")]
    FrenchTransactionTax,
    /// PTM levy
    #[serde(rename = "PTM_LEVY")]
    PtmLevy,
    /// Stamp duty
    #[serde(rename = "STAMP_DUTY")]
    StampDuty,
    /// Stamp duty reserve tax
    #[serde(rename = "STAMP_DUTY_RESERVE_TAX")]
    StampDutyReserveTax,
    /// Transaction fee
    #[serde(rename = "TRANSACTION_FEE")]
    TransactionFee,
}

impl Default for Name {
    fn default() -> Self {
        Self::CommissionTurnover
    }
}
