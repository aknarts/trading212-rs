//! Icons used by the pies

/// Icons
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Icon {
    /// Home
    #[serde(rename = "Home")]
    Home,
    /// PiggyBank
    #[serde(rename = "PiggyBank")]
    PiggyBank,
    /// Iceberg
    #[serde(rename = "Iceberg")]
    Iceberg,
    /// Airplane
    #[serde(rename = "Airplane")]
    Airplane,
    /// Rv
    #[serde(rename = "RV")]
    Rv,
    /// Unicorn
    #[serde(rename = "Unicorn")]
    Unicorn,
    /// Whale
    #[serde(rename = "Whale")]
    Whale,
    /// Convertable
    #[serde(rename = "Convertable")]
    Convertable,
    /// Family
    #[serde(rename = "Family")]
    Family,
    /// Coins
    #[serde(rename = "Coins")]
    Coins,
    /// Education
    #[serde(rename = "Education")]
    Education,
    /// Bills and coins
    #[serde(rename = "BillsAndCoins")]
    BillsAndCoins,
    /// Bills
    #[serde(rename = "Bills")]
    Bills,
    /// Water
    #[serde(rename = "Water")]
    Water,
    /// Wind
    #[serde(rename = "Wind")]
    Wind,
    /// Car
    #[serde(rename = "Car")]
    Car,
    /// Briefcase
    #[serde(rename = "Briefcase")]
    Briefcase,
    /// Medical
    #[serde(rename = "Medical")]
    Medical,
    /// Landscape
    #[serde(rename = "Landscape")]
    Landscape,
    /// Child
    #[serde(rename = "Child")]
    Child,
    /// Vault
    #[serde(rename = "Vault")]
    Vault,
    /// Travel
    #[serde(rename = "Travel")]
    Travel,
    /// Cabin
    #[serde(rename = "Cabin")]
    Cabin,
    /// Apartments
    #[serde(rename = "Apartments")]
    Apartments,
    /// Burger
    #[serde(rename = "Burger")]
    Burger,
    /// Bus
    #[serde(rename = "Bus")]
    Bus,
    /// Energy
    #[serde(rename = "Energy")]
    Energy,
    /// Factory
    #[serde(rename = "Factory")]
    Factory,
    /// Global
    #[serde(rename = "Global")]
    Global,
    /// Leaf
    #[serde(rename = "Leaf")]
    Leaf,
    /// Materials
    #[serde(rename = "Materials")]
    Materials,
    /// Pill
    #[serde(rename = "Pill")]
    Pill,
    /// Ring
    #[serde(rename = "Ring")]
    Ring,
    /// Shipping
    #[serde(rename = "Shipping")]
    Shipping,
    /// Storefront
    #[serde(rename = "Storefront")]
    Storefront,
    /// Tech
    #[serde(rename = "Tech")]
    Tech,
    /// Umbrella
    #[serde(rename = "Umbrella")]
    Umbrella,
}

impl Default for Icon {
    fn default() -> Self {
        Self::Home
    }
}
