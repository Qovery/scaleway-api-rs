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
pub struct RefreshDnsZoneRequest {
    /// Whether or not to recreate the DNS zone
    #[serde(rename = "recreate_dns_zone", skip_serializing_if = "Option::is_none")]
    pub recreate_dns_zone: Option<bool>,
    /// Whether or not to recreate the sub DNS zone
    #[serde(
        rename = "recreate_sub_dns_zone",
        skip_serializing_if = "Option::is_none"
    )]
    pub recreate_sub_dns_zone: Option<bool>,
}

impl RefreshDnsZoneRequest {
    pub fn new() -> RefreshDnsZoneRequest {
        RefreshDnsZoneRequest {
            recreate_dns_zone: None,
            recreate_sub_dns_zone: None,
        }
    }
}
