/*
 * Account API
 *
 * # Introduction  The Account API allows you to manage projects. Project is Scaleway’s resource management feature. Designed to help you organize your infrastructure and cloud services, the feature allows resources to be isolated and grouped into specific projects.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScalewayInstanceV1ServerLocation : The server location

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScalewayInstanceV1ServerLocation {
    #[serde(rename = "cluster_id", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "hypervisor_id", skip_serializing_if = "Option::is_none")]
    pub hypervisor_id: Option<String>,
    #[serde(rename = "node_id", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "platform_id", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "zone_id", skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
}

impl ScalewayInstanceV1ServerLocation {
    /// The server location
    pub fn new() -> ScalewayInstanceV1ServerLocation {
        ScalewayInstanceV1ServerLocation {
            cluster_id: None,
            hypervisor_id: None,
            node_id: None,
            platform_id: None,
            zone_id: None,
        }
    }
}