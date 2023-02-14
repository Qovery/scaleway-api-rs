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
pub struct DuplicateMacAddrRequest {
    /// Flexible IPs need to be attached to the same server.
    #[serde(rename = "duplicate_from_fip_id")]
    pub duplicate_from_fip_id: String,
}

impl DuplicateMacAddrRequest {
    pub fn new(duplicate_from_fip_id: String) -> DuplicateMacAddrRequest {
        DuplicateMacAddrRequest {
            duplicate_from_fip_id,
        }
    }
}
