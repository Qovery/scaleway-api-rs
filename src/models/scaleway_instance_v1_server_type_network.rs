/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayInstanceV1ServerTypeNetwork : Network available for the instance

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayInstanceV1ServerTypeNetwork {
    /// List of available network interfaces
    #[serde(rename = "interfaces", skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<Vec<crate::models::ScalewayPeriodInstancePeriodV1PeriodServerTypePeriodNetworkPeriodInterface>>,
    /// Total maximum internal bandwidth in bits per seconds
    #[serde(rename = "sum_internal_bandwidth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sum_internal_bandwidth: Option<Option<i32>>,
    /// Total maximum internet bandwidth in bits per seconds
    #[serde(rename = "sum_internet_bandwidth", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sum_internet_bandwidth: Option<Option<i32>>,
    /// True if IPv6 is enabled
    #[serde(rename = "ipv6_support", skip_serializing_if = "Option::is_none")]
    pub ipv6_support: Option<bool>,
}

impl ScalewayInstanceV1ServerTypeNetwork {
    /// Network available for the instance
    pub fn new() -> ScalewayInstanceV1ServerTypeNetwork {
        ScalewayInstanceV1ServerTypeNetwork {
            interfaces: None,
            sum_internal_bandwidth: None,
            sum_internet_bandwidth: None,
            ipv6_support: None,
        }
    }
}
