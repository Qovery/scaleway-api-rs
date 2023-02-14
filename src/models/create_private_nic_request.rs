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
pub struct CreatePrivateNicRequest {
    /// UUID of the private network where the private NIC will be attached
    #[serde(rename = "private_network_id")]
    pub private_network_id: String,
}

impl CreatePrivateNicRequest {
    pub fn new(private_network_id: String) -> CreatePrivateNicRequest {
        CreatePrivateNicRequest { private_network_id }
    }
}
