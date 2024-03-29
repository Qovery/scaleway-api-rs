/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayRdbV1EndpointPrivateNetwork : Private network details. One at the most per RDB instance or read replica (an RDB instance and its read replica can have different private networks). Cannot be updated (has to be deleted and recreated)

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayRdbV1EndpointPrivateNetwork {
    /// UUID of the private network (UUID format)
    #[serde(rename = "private_network_id", skip_serializing_if = "Option::is_none")]
    pub private_network_id: Option<String>,
    /// CIDR notation of the endpoint IPv4 address (IP network)
    #[serde(rename = "service_ip", skip_serializing_if = "Option::is_none")]
    pub service_ip: Option<String>,
    /// Private network zone
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl ScalewayRdbV1EndpointPrivateNetwork {
    /// Private network details. One at the most per RDB instance or read replica (an RDB instance and its read replica can have different private networks). Cannot be updated (has to be deleted and recreated)
    pub fn new() -> ScalewayRdbV1EndpointPrivateNetwork {
        ScalewayRdbV1EndpointPrivateNetwork {
            private_network_id: None,
            service_ip: None,
            zone: None,
        }
    }
}
