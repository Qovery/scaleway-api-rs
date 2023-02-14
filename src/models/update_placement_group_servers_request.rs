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
pub struct UpdatePlacementGroupServersRequest {
    #[serde(rename = "servers")]
    pub servers: Vec<String>,
}

impl UpdatePlacementGroupServersRequest {
    pub fn new(servers: Vec<String>) -> UpdatePlacementGroupServersRequest {
        UpdatePlacementGroupServersRequest { servers }
    }
}
