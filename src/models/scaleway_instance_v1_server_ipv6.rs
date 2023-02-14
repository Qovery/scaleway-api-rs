/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayInstanceV1ServerIpv6 : The server IPv6 address

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayInstanceV1ServerIpv6 {
    /// The server IPv6 IP-Address (IPv6 address)
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The IPv6 IP-addresses gateway (IPv6 address)
    #[serde(rename = "gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// The IPv6 IP-addresses CIDR netmask
    #[serde(rename = "netmask", skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
}

impl ScalewayInstanceV1ServerIpv6 {
    /// The server IPv6 address
    pub fn new() -> ScalewayInstanceV1ServerIpv6 {
        ScalewayInstanceV1ServerIpv6 {
            address: None,
            gateway: None,
            netmask: None,
        }
    }
}