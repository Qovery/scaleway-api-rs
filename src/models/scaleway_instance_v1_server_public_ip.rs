/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayInstanceV1ServerPublicIp : Information about the public IP

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayInstanceV1ServerPublicIp {
    /// The unique ID of the IP address
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The server public IPv4 IP-Address (IPv4 address)
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// True if the IP address is dynamic
    #[serde(rename = "dynamic", skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl ScalewayInstanceV1ServerPublicIp {
    /// Information about the public IP
    pub fn new() -> ScalewayInstanceV1ServerPublicIp {
        ScalewayInstanceV1ServerPublicIp {
            id: None,
            address: None,
            dynamic: None,
        }
    }
}