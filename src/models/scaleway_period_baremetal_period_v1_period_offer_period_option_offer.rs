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
pub struct ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
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
    #[serde(
        rename = "os_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_id: Option<Option<String>>,
}

impl ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
    pub fn new() -> ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
        ScalewayPeriodBaremetalPeriodV1PeriodOfferPeriodOptionOffer {
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