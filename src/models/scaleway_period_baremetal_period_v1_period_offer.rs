/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayPeriodBaremetalPeriodV1PeriodOffer {
    /// ID of the offer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the offer
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Stock level
    #[serde(rename = "stock", skip_serializing_if = "Option::is_none")]
    pub stock: Option<Stock>,
    /// Bandwidth available in bits/s with the offer
    #[serde(rename = "bandwidth", skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i32>,
    /// Commercial range of the offer
    #[serde(rename = "commercial_range", skip_serializing_if = "Option::is_none")]
    pub commercial_range: Option<String>,
    #[serde(rename = "price_per_hour", skip_serializing_if = "Option::is_none")]
    pub price_per_hour: Option<Box<crate::models::ScalewayBaremetalV1OfferPricePerHour>>,
    #[serde(rename = "price_per_month", skip_serializing_if = "Option::is_none")]
    pub price_per_month: Option<Box<crate::models::ScalewayBaremetalV1OfferPricePerMonth>>,
    /// Disks specifications of the offer
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodDisk>>,
    /// True if the offer is currently available
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// CPU specifications of the offer
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodCpu>>,
    /// Memory specifications of the offer
    #[serde(rename = "memories", skip_serializing_if = "Option::is_none")]
    pub memories: Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodMemory>>,
    /// Name of the quota associated to the offer
    #[serde(rename = "quota_name", skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// Persistent memory specifications of the offer
    #[serde(
        rename = "persistent_memories",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_memories:
        Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodPersistentMemory>>,
    /// Raid controller specifications of the offer
    #[serde(rename = "raid_controllers", skip_serializing_if = "Option::is_none")]
    pub raid_controllers:
        Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodRaidController>>,
    /// Array of incompatible OS ids
    #[serde(
        rename = "incompatible_os_ids",
        skip_serializing_if = "Option::is_none"
    )]
    pub incompatible_os_ids: Option<Vec<String>>,
    /// Period of subscription for the offer
    #[serde(
        rename = "subscription_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_period: Option<SubscriptionPeriod>,
    /// Operation path of the service
    #[serde(rename = "operation_path", skip_serializing_if = "Option::is_none")]
    pub operation_path: Option<String>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<Box<crate::models::ScalewayBaremetalV1OfferFee>>,
    /// Options available on offer
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options:
        Option<Vec<crate::models::ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer>>,
}

impl ScalewayPeriodBaremetalPeriodV1PeriodOffer {
    pub fn new() -> ScalewayPeriodBaremetalPeriodV1PeriodOffer {
        ScalewayPeriodBaremetalPeriodV1PeriodOffer {
            id: None,
            name: None,
            stock: None,
            bandwidth: None,
            commercial_range: None,
            price_per_hour: None,
            price_per_month: None,
            disks: None,
            enable: None,
            cpus: None,
            memories: None,
            quota_name: None,
            persistent_memories: None,
            raid_controllers: None,
            incompatible_os_ids: None,
            subscription_period: None,
            operation_path: None,
            fee: None,
            options: None,
        }
    }
}

/// Stock level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Stock {
    #[serde(rename = "empty")]
    Empty,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "available")]
    Available,
}

impl Default for Stock {
    fn default() -> Stock {
        Self::Empty
    }
}
/// Period of subscription for the offer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubscriptionPeriod {
    #[serde(rename = "unknown_subscription_period")]
    UnknownSubscriptionPeriod,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "monthly")]
    Monthly,
}

impl Default for SubscriptionPeriod {
    fn default() -> SubscriptionPeriod {
        Self::UnknownSubscriptionPeriod
    }
}
