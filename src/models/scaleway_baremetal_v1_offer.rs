/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1Offer {
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
    pub bandwidth: Option<i64>,
    /// Commercial range of the offer
    #[serde(rename = "commercial_range", skip_serializing_if = "Option::is_none")]
    pub commercial_range: Option<String>,
    #[serde(rename = "price_per_hour", skip_serializing_if = "Option::is_none")]
    pub price_per_hour: Option<Box<crate::models::ScalewayBaremetalV1OfferPricePerHour>>,
    #[serde(rename = "price_per_month", skip_serializing_if = "Option::is_none")]
    pub price_per_month: Option<Box<crate::models::ScalewayBaremetalV1OfferPricePerMonth>>,
    /// Disks specifications of the offer
    #[serde(rename = "disks", skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<crate::models::ScalewayBaremetalV1Disk>>,
    /// True if the offer is currently available
    #[serde(rename = "enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// CPU specifications of the offer
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<Vec<crate::models::ScalewayBaremetalV1Cpu>>,
    /// Memory specifications of the offer
    #[serde(rename = "memories", skip_serializing_if = "Option::is_none")]
    pub memories: Option<Vec<crate::models::ScalewayBaremetalV1Memory>>,
    /// Name of the quota associated to the offer
    #[serde(rename = "quota_name", skip_serializing_if = "Option::is_none")]
    pub quota_name: Option<String>,
    /// Persistent memory specifications of the offer
    #[serde(
        rename = "persistent_memories",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_memories: Option<Vec<crate::models::ScalewayBaremetalV1PersistentMemory>>,
    /// Raid controller specifications of the offer
    #[serde(rename = "raid_controllers", skip_serializing_if = "Option::is_none")]
    pub raid_controllers: Option<Vec<crate::models::ScalewayBaremetalV1RaidController>>,
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
    pub options: Option<Vec<crate::models::ScalewayBaremetalV1OfferOptionOffer>>,
}

impl ScalewayBaremetalV1Offer {
    pub fn new() -> ScalewayBaremetalV1Offer {
        ScalewayBaremetalV1Offer {
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
