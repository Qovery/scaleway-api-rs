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
pub struct ImportProviderDnsZoneRequest {
    #[serde(rename = "online_v1", skip_serializing_if = "Option::is_none")]
    pub online_v1: Option<Box<crate::models::ImportProviderDnsZoneRequestOnlineV1>>,
}

impl ImportProviderDnsZoneRequest {
    pub fn new() -> ImportProviderDnsZoneRequest {
        ImportProviderDnsZoneRequest { online_v1: None }
    }
}
