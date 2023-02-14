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
pub struct SetPlacementGroupServersRequest {
    #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<String>>,
}

impl SetPlacementGroupServersRequest {
    pub fn new() -> SetPlacementGroupServersRequest {
        SetPlacementGroupServersRequest { servers: None }
    }
}