/*
 * Elastic metal API
 *
 * # Introduction  Elastic metal as a service allows ordering a dedicated server on-demand like a cloud instance. Dedicated servers could be used for large workloads, big data, those requiring more security, ….  ## Technical Limitations  - Elastic metal is available in `fr-par-1`,  `fr-par-2`, `nl-ams-1` zones  - Installation is done by preseed (± 10min) (preseed: complete install from a virtual media)  ## Features  - Install (Server is installed by preseed (preseed: complete install from a virtual media), you must define at least one ssh key to install your server)  - Start/Stop/Reboot  - Rescue Reboot, a rescue image is an operating system image designed to help you diagnose and fix an OS experiencing failures. When your server boot on rescue, you can mount your disks and start diagnosing/fixing your image.  - Billed by minute (The billing start when the server is delivered and stop when the server is deleted)  - IPv6, all servers are available with an IPv6 /128  - ReverseIP, You can configure your reverse IP (IPv4 and IPv6), you must register the server IP in your DNS records before calling the endpoint  - Basic monitoring with ping status  - Flexible IP is available ([documentation](https://developers.scaleway.com/en/products/flexible-ip/api/))  - IP failovers are not available in api v1, use the api v1alpha1  ## FAQ  ### How can I get my SSH key id?  You can find your `$SCW_SECRET_KEY` and your `SCW_DEFAULT_ORGANIZATION_ID` at the following page: https://console.scaleway.com/project/credentials  ### How can I add my server to a private network?  See [our online documentation](https://developers.scaleway.com/en/products/vpc-elasticmetal/api/).
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScalewayBaremetalV1OfferOptionOffer {
    /// ID of the option
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the option
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If true the option is enabled and included by default in the offer If false the option is available for the offer but not included by default
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Period of subscription for the offer
    #[serde(
        rename = "subscription_period",
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription_period: Option<SubscriptionPeriod>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Box<crate::models::ScalewayBaremetalV1OfferOptionOfferPrice>>,
    /// Boolean to know if option could be managed
    #[serde(rename = "manageable", skip_serializing_if = "Option::is_none")]
    pub manageable: Option<bool>,
    /// ID of the OS linked to the option
    #[serde(rename = "os_id", skip_serializing_if = "Option::is_none")]
    pub os_id: Option<String>,
}

impl ScalewayBaremetalV1OfferOptionOffer {
    pub fn new() -> ScalewayBaremetalV1OfferOptionOffer {
        ScalewayBaremetalV1OfferOptionOffer {
            id: None,
            name: None,
            enabled: None,
            subscription_period: None,
            price: None,
            manageable: None,
            os_id: None,
        }
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
