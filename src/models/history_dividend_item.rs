//! History dividend item model

use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

/// History dividend item.
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryDividendItem {
    /// In instrument currency
    #[serde(rename = "amount")]
    pub amount: f32,
    /// In EUR
    #[serde(rename = "amountInEuro", skip_serializing_if = "Option::is_none")]
    pub amount_in_euro: Option<f32>,
    /// In instrument currency
    #[serde(
        rename = "grossAmountPerShare",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_amount_per_share: Option<f32>,
    /// Paid on
    /// 2024-03-21T15:46:51.000+02:00
    #[serde(rename = "paidOn")]
    #[serde_as(as = "Rfc3339")]
    pub paid_on: OffsetDateTime,
    /// Quantity
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    /// Reference
    #[serde(rename = "reference")]
    pub reference: Uuid,
    /// Ticker
    #[serde(rename = "ticker")]
    pub ticker: String,
    /// Type
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl HistoryDividendItem {
    /// Create a new `HistoryDividendItem`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            amount: 0.0,
            amount_in_euro: None,
            gross_amount_per_share: None,
            paid_on: OffsetDateTime::UNIX_EPOCH,
            quantity: None,
            reference: Uuid::nil(),
            ticker: String::new(),
            r#type: Type::Unknown,
        }
    }
}

impl Default for HistoryDividendItem {
    fn default() -> Self {
        Self::new()
    }
}

/// Dividend item Type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    /// Ordinary
    // Not in spec
    #[serde(rename = "DIVIDEND")]
    Dividend,
    /// Ordinary
    #[serde(rename = "ORDINARY")]
    Ordinary,
    /// Bonus
    #[serde(rename = "BONUS")]
    Bonus,
    /// Property income
    #[serde(rename = "PROPERTY_INCOME")]
    PropertyIncome,
    /// Return of capital
    #[serde(rename = "RETURN_OF_CAPITAL_NON_US")]
    ReturnOfCapitalNonUs,
    /// Demerger
    #[serde(rename = "DEMERGER")]
    Demerger,
    /// Interest
    #[serde(rename = "INTEREST")]
    Interest,
    /// Capital gains distribution
    #[serde(rename = "CAPITAL_GAINS_DISTRIBUTION_NON_US")]
    CapitalGainsDistributionNonUs,
    /// Interim liquidation
    #[serde(rename = "INTERIM_LIQUIDATION")]
    InterimLiquidation,
    /// Ordinary Manufactured Payment
    #[serde(rename = "ORDINARY_MANUFACTURED_PAYMENT")]
    OrdinaryManufacturedPayment,
    /// Bonus Manufactured Payment
    #[serde(rename = "BONUS_MANUFACTURED_PAYMENT")]
    BonusManufacturedPayment,
    /// Property income Manufactured Payment
    #[serde(rename = "PROPERTY_INCOME_MANUFACTURED_PAYMENT")]
    PropertyIncomeManufacturedPayment,
    /// Return of capital Manufactured Payment
    #[serde(rename = "RETURN_OF_CAPITAL_NON_US_MANUFACTURED_PAYMENT")]
    ReturnOfCapitalNonUsManufacturedPayment,
    /// Demerger Manufactured Payment
    #[serde(rename = "DEMERGER_MANUFACTURED_PAYMENT")]
    DemergerManufacturedPayment,
    /// Interest Manufactured Payment
    #[serde(rename = "INTEREST_MANUFACTURED_PAYMENT")]
    InterestManufacturedPayment,
    /// Capital gains distribution Manufactured Payment
    #[serde(rename = "CAPITAL_GAINS_DISTRIBUTION_NON_US_MANUFACTURED_PAYMENT")]
    CapitalGainsDistributionNonUsManufacturedPayment,
    /// Interim liquidation Manufactured Payment
    #[serde(rename = "INTERIM_LIQUIDATION_MANUFACTURED_PAYMENT")]
    InterimLiquidationManufacturedPayment,
    /// Interest paid by US obligors
    #[serde(rename = "INTEREST_PAID_BY_US_OBLIGORS")]
    InterestPaidByUsObligors,
    /// Interest paid by foreign corporations
    #[serde(rename = "INTEREST_PAID_BY_FOREIGN_CORPORATIONS")]
    InterestPaidByForeignCorporations,
    /// Dividends paid by US corporations
    #[serde(rename = "DIVIDENDS_PAID_BY_US_CORPORATIONS")]
    DividendsPaidByUsCorporations,
    /// Dividends paid by foreign corporations
    #[serde(rename = "DIVIDENDS_PAID_BY_FOREIGN_CORPORATIONS")]
    DividendsPaidByForeignCorporations,
    /// Capital gains
    #[serde(rename = "CAPITAL_GAINS")]
    CapitalGains,
    /// Real property income and natural resources royalties
    #[serde(rename = "REAL_PROPERTY_INCOME_AND_NATURAL_RESOURCES_ROYALTIES")]
    RealPropertyIncomeAndNaturalResourcesRoyalties,
    /// Other income
    #[serde(rename = "OTHER_INCOME")]
    OtherIncome,
    /// Qualified investment entity
    #[serde(rename = "QUALIFIED_INVESTMENT_ENTITY")]
    QualifiedInvestmentEntity,
    /// Trust distribution
    #[serde(rename = "TRUST_DISTRIBUTION")]
    TrustDistribution,
    /// Publicly traded partnership distribution
    #[serde(rename = "PUBLICLY_TRADED_PARTNERSHIP_DISTRIBUTION")]
    PubliclyTradedPartnershipDistribution,
    /// Capital gains distribution
    #[serde(rename = "CAPITAL_GAINS_DISTRIBUTION")]
    CapitalGainsDistribution,
    /// Return of capital
    #[serde(rename = "RETURN_OF_CAPITAL")]
    ReturnOfCapital,
    /// Other dividend equivalent
    #[serde(rename = "OTHER_DIVIDEND_EQUIVALENT")]
    OtherDividendEquivalent,
    /// Tax event 1446f for publicly traded securities
    #[serde(rename = "TAX_EVENT_1446F_FOR_PUBLICLY_TRADED_SECURITIES")]
    TaxEvent1446FForPubliclyTradedSecurities,
    /// PTP uncharacterised income
    #[serde(rename = "PTP_UNCHARACTERISED_INCOME")]
    PtpUncharacterisedIncome,
    /// Multiple 1042s tax components
    #[serde(rename = "MULTIPLE_1042S_TAX_COMPONENTS")]
    Multiple1042STaxComponents,
    /// Interest paid by US obligors Manufactured Payment
    #[serde(rename = "INTEREST_PAID_BY_US_OBLIGORS_MANUFACTURED_PAYMENT")]
    InterestPaidByUsObligorsManufacturedPayment,
    /// Interest paid by foreign corporations Manufactured Payment
    #[serde(rename = "INTEREST_PAID_BY_FOREIGN_CORPORATIONS_MANUFACTURED_PAYMENT")]
    InterestPaidByForeignCorporationsManufacturedPayment,
    /// Dividends paid by US corporations Manufactured Payment
    #[serde(rename = "DIVIDENDS_PAID_BY_US_CORPORATIONS_MANUFACTURED_PAYMENT")]
    DividendsPaidByUsCorporationsManufacturedPayment,
    /// Dividends paid by foreign corporations Manufactured Payment
    #[serde(rename = "DIVIDENDS_PAID_BY_FOREIGN_CORPORATIONS_MANUFACTURED_PAYMENT")]
    DividendsPaidByForeignCorporationsManufacturedPayment,
    /// Capital gains Manufactured Payment
    #[serde(rename = "CAPITAL_GAINS_MANUFACTURED_PAYMENT")]
    CapitalGainsManufacturedPayment,
    /// Real property income and natural resources royalties Manufactured Payment
    #[serde(rename = "REAL_PROPERTY_INCOME_AND_NATURAL_RESOURCES_ROYALTIES_MANUFACTURED_PAYMENT")]
    RealPropertyIncomeAndNaturalResourcesRoyaltiesManufacturedPayment,
    /// Other income Manufactured Payment
    #[serde(rename = "OTHER_INCOME_MANUFACTURED_PAYMENT")]
    OtherIncomeManufacturedPayment,
    /// Qualified investment entity Manufactured Payment
    #[serde(rename = "QUALIFIED_INVESTMENT_ENTITY_MANUFACTURED_PAYMENT")]
    QualifiedInvestmentEntityManufacturedPayment,
    /// Trust distribution Manufactured Payment
    #[serde(rename = "TRUST_DISTRIBUTION_MANUFACTURED_PAYMENT")]
    TrustDistributionManufacturedPayment,
    /// Publicly traded partnership distribution Manufactured Payment
    #[serde(rename = "PUBLICLY_TRADED_PARTNERSHIP_DISTRIBUTION_MANUFACTURED_PAYMENT")]
    PubliclyTradedPartnershipDistributionManufacturedPayment,
    /// Capital gains distribution Manufactured Payment
    #[serde(rename = "CAPITAL_GAINS_DISTRIBUTION_MANUFACTURED_PAYMENT")]
    CapitalGainsDistributionManufacturedPayment,
    /// Return of capital Manufactured Payment
    #[serde(rename = "RETURN_OF_CAPITAL_MANUFACTURED_PAYMENT")]
    ReturnOfCapitalManufacturedPayment,
    /// Other dividend equivalent Manufactured Payment
    #[serde(rename = "OTHER_DIVIDEND_EQUIVALENT_MANUFACTURED_PAYMENT")]
    OtherDividendEquivalentManufacturedPayment,
    /// Tax event 1446f for publicly traded securities Manufactured Payment
    #[serde(rename = "TAX_EVENT_1446F_FOR_PUBLICLY_TRADED_SECURITIES_MANUFACTURED_PAYMENT")]
    TaxEvent1446FForPubliclyTradedSecuritiesManufacturedPayment,
    /// PTP uncharacterised income Manufactured Payment
    #[serde(rename = "PTP_UNCHARACTERISED_INCOME_MANUFACTURED_PAYMENT")]
    PtpUncharacterisedIncomeManufacturedPayment,
    /// Multiple 1042s tax components Manufactured Payment
    #[serde(rename = "MULTIPLE_1042S_TAX_COMPONENTS_MANUFACTURED_PAYMENT")]
    Multiple1042STaxComponentsManufacturedPayment,
    /// PROPERTY_INCOME_DISTRIBUTION
    #[serde(rename = "PROPERTY_INCOME_DISTRIBUTION")]
    PropertyIncomeDistribution,
    /// Unknown
    #[serde(other)]
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::Unknown
    }
}
